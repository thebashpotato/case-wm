//! A simple wrapper for dmenu-rs. Modified to accomodate the
//! rust re-write which uses sane flag conventions, but suckless's
//! dmenu does not.

use crate::config::ColorSchemeConfig;
use penrose::extensions::util::dmenu::MenuMatch;
use penrose::{Color, Error, Result as PenroseResult};
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

/// Two different derivatives of dmenu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMenuKind {
    /// Suckless's version of dmenu
    ///
    /// [1]: https://tools.suckless.org/dmenu/
    Suckless,
    /// Newer `dmenu-rs`
    ///
    /// [1]: https://github.com/Shizcow/dmenu-rs
    Rust,
}

/// A configuration for `DMenuRS`
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy)]
pub struct DMenuConfig<'c> {
    /// Should line numbers be displayed to the user?
    ///
    /// Default: false
    pub show_line_numbers: bool,

    /// Show dmenu at the bottom the the screen.
    ///
    /// Default: false
    pub show_on_bottom: bool,

    /// Should dmenu treat the input as a password and render characters as '*'?
    ///
    /// NOTE: This requires the [Password][1] patch in order to work.
    /// or in the case of dmenu-rs it requires the password plugin.
    ///
    /// Default: false
    ///
    /// [1]: https://tools.suckless.org/dmenu/patches/password/
    /// [1]: https://github.com/Shizcow/dmenu-rs/tree/master/src/plugins
    pub password_input: bool,

    /// Should dmenu ignore case in the user input when matching?
    ///
    /// Default: false
    pub ignore_case: bool,

    /// Background color for the rendered window
    ///
    /// Default: #1d2021
    pub bg_color: Color,

    /// Foreground color for text
    ///
    /// Default: #ebdbb2
    pub fg_color: Color,

    /// Selected line background color
    ///
    /// Default: #458588
    pub selected_color: Color,

    /// Number of lines to display at a time.
    ///
    /// Setting n_lines=0 will result in the choices being displayed horizontally
    /// instead of vertically.
    ///
    /// Default: 10
    pub n_lines: u8,

    /// Allow the user to load a custom font
    ///
    /// Default: None
    pub custom_font: Option<&'c str>,

    /// Specify to kind of dmenu to use
    ///
    /// Default: Suckless
    pub kind: DMenuKind,
}

impl<'c> Default for DMenuConfig<'c> {
    fn default() -> Self {
        let cs = ColorSchemeConfig::new();
        Self {
            show_line_numbers: false,
            show_on_bottom: false,
            password_input: false,
            ignore_case: false,
            bg_color: cs.background(),
            fg_color: cs.foreground(),
            selected_color: cs.primary(),
            n_lines: 10,
            custom_font: None,
            kind: DMenuKind::Suckless,
        }
    }
}

impl<'c> DMenuConfig<'c> {
    /// Build the dmenu flags
    fn flags(&self, custom_prompt: Option<&str>, screen_index: usize) -> Vec<String> {
        let &Self {
            show_on_bottom,
            password_input,
            ignore_case,
            bg_color,
            fg_color,
            selected_color,
            n_lines,
            custom_font,
            kind,
            ..
        } = self;

        // Only some command line options require the "--" for the rust version.
        let prefix = match kind {
            DMenuKind::Suckless => "-",
            DMenuKind::Rust => "--",
        };

        let mut flags = vec!["-m".to_owned(), screen_index.to_string()];

        flags.push(format!("{prefix}nb"));
        flags.push(bg_color.as_rgb_hex_string());

        flags.push(format!("{prefix}nf"));
        flags.push(fg_color.as_rgb_hex_string());

        flags.push(format!("{prefix}sb"));
        flags.push(selected_color.as_rgb_hex_string());

        if n_lines > 0 {
            flags.push("-l".to_owned());
            flags.push(n_lines.to_string());
        }

        if show_on_bottom {
            flags.push("-b".to_owned());
        }

        if password_input {
            flags.push("-P".to_owned());
        }

        if ignore_case {
            flags.push("-i".to_owned());
        }

        if let Some(font) = custom_font {
            flags.push(format!("{prefix}fn"));
            flags.push(font.to_owned());
        }

        if let Some(prompt) = custom_prompt {
            flags.push("-p".to_owned());
            flags.push(prompt.to_owned());
        }

        flags
    }
}

/// A wrapper around the suckless [dmenu][1] program for creating dynamic menus
/// in penrose.
#[derive(Debug, Clone)]
pub struct DMenu<'c> {
    /// Optional prompt customization.
    prompt: Option<&'c str>,
    /// Holds the custom dmenu configuration for this instance.
    config: DMenuConfig<'c>,
    /// The screen index this instance of dmenu will show up on
    screen_index: usize,
}

impl<'c> DMenu<'c> {
    /// Create a new `DMenu` command which can be triggered and reused by calling
    /// the `run` method.
    pub const fn new(
        prompt: Option<&'c str>,
        config: DMenuConfig<'c>,
        screen_index: usize,
    ) -> Self {
        Self {
            prompt,
            config,
            screen_index,
        }
    }

    /// Used for launching dmenu with no menu matching.
    /// So the user can pass flags to their dmenu instance.
    pub fn run(&self) -> PenroseResult<()> {
        let args = self.config.flags(self.prompt, self.screen_index);
        let spawned_process = Command::new("dmenu_run").args(args).spawn();

        match spawned_process {
            Ok(mut process) => match process.wait() {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
    }

    /// Run this [`DMenu`] command and return the selected choice.
    ///
    /// # Example
    /// ```ignore
    /// # use penrose::extensions::util::dmenu::*;
    /// let screen_index = 0;
    /// let dmenu = DMenu::new(Some(">>>"), DMenuConfig::default(), screen_index);
    ///
    /// let choices = vec!["some", "choices", "to", "pick", "from"];
    ///
    /// match dmenu.build_menu(choices).unwrap() {
    ///     MenuMatch::Line(i, s) => println!("matched '{}' on line '{}'", s, i),
    ///     MenuMatch::UserInput(s) => println!("user input: '{}'", s),
    ///     MenuMatch::NoMatch => println!("no match"),
    /// }
    /// ```
    #[allow(clippy::pattern_type_mismatch)]
    pub fn build_menu(&self, param_choices: Vec<impl Into<String>>) -> PenroseResult<MenuMatch> {
        let choices: Vec<String> = param_choices
            .into_iter()
            .map(std::convert::Into::into)
            .collect();
        let raw = self.raw_user_choice_from_dmenu(&choices)?;
        let choice = raw.trim();

        if choice.is_empty() {
            return Ok(MenuMatch::NoMatch);
        }

        let res = choices
            .iter()
            .enumerate()
            .find(|(i, s)| {
                if self.config.show_line_numbers {
                    format!("{i:<3} {s}") == choice
                } else {
                    *s == choice
                }
            })
            .map_or_else(
                || MenuMatch::UserInput(choice.to_owned()),
                |(i, _)| {
                    MenuMatch::Line(
                        i,
                        choices.get(i).expect("Indexing choices panicked").clone(),
                    )
                },
            );

        Ok(res)
    }

    /// Get a vector of choices as bytes
    fn choices_as_input_bytes(&self, choices: &[String]) -> Vec<u8> {
        if self.config.show_line_numbers {
            choices
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{i:<3} {s}"))
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes()
                .to_vec()
        } else {
            choices.join("\n").as_bytes().to_vec()
        }
    }

    /// Launch a shell process with all arguments to dmenu
    fn raw_user_choice_from_dmenu(&self, choices: &[String]) -> PenroseResult<String> {
        let args = self.config.flags(self.prompt, self.screen_index);
        let mut proc = Command::new("dmenu")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(args)
            .spawn()?;

        {
            // Taking stdin here and dropping it when the block scope ends to close it and
            // let dmenu determine the end of input
            let mut stdin = proc
                .stdin
                .take()
                .ok_or_else(|| Error::Custom("unable to open stdin".to_owned()))?;

            stdin.write_all(&self.choices_as_input_bytes(choices))?;
        }

        let mut raw = String::new();
        proc.stdout
            .ok_or_else(|| Error::Custom("failed to spawn dmenu".to_owned()))?
            .read_to_string(&mut raw)?;

        Ok(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::{DMenuConfig, DMenuKind};

    /// Flags [ nb, nf, sb, and nf] need to be modified for the different
    /// versions of dmenu. Classic Suckless dmenu uses a single dash "-", dmenu-rs
    /// uses the more modern cli style of double dashes "--".
    /// This test depends on the order the flags are loaded into the array, so if the order
    /// is changed in the flags function, these tests will fail. There is a better way, but
    /// this works for now.
    #[test]
    fn dmenu_suckless_config_test() {
        let dc = DMenuConfig {
            custom_font: Some("mono"),
            ..DMenuConfig::default()
        };

        // Should default to suckless c-style dmenu
        assert_eq!(dc.kind, DMenuKind::Suckless);
        let flags = dc.flags(None, 0);

        for (i, flag) in flags.into_iter().enumerate() {
            if i == 2 {
                assert_eq!(flag, "-nb".to_owned());
            }
            if i == 4 {
                assert_eq!(flag, "-nf".to_owned());
            }
            if i == 6 {
                assert_eq!(flag, "-sb".to_owned());
            }
            if i == 10 {
                assert_eq!(flag, "-fn".to_owned());
            }
        }
    }

    /// Flags [ nb, nf, sb, and nf] need to be modified for the different
    /// versions of dmenu. Classic Suckless dmenu uses a single dash "-", dmenu-rs
    /// uses the more modern cli style of double dashes "--".
    /// This test depends on the order the flags are loaded into the array, so if the order
    /// is changed in the flags function, these tests will fail. There is a better way, but
    /// this works for now.
    #[test]
    fn dmenu_rs_config_test() {
        let dc = DMenuConfig {
            custom_font: Some("mono"),
            kind: DMenuKind::Rust,
            ..DMenuConfig::default()
        };

        assert_eq!(dc.kind, DMenuKind::Rust);
        let flags = dc.flags(None, 0);

        for (i, flag) in flags.into_iter().enumerate() {
            if i == 2 {
                assert_eq!(flag, "--nb".to_owned());
            }
            if i == 4 {
                assert_eq!(flag, "--nf".to_owned());
            }
            if i == 6 {
                assert_eq!(flag, "--sb".to_owned());
            }
            if i == 10 {
                assert_eq!(flag, "--fn".to_owned());
            }
        }
    }
}
