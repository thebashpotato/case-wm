//! Configure and customize the baked in status bar

use crate::config::{BarConfig, ColorSchemeConfig};
use penrose::x::XConn;
use penrose_ui::{
    bar::{
        widgets::{
            amixer_volume, battery_summary, current_date_and_time, wifi_network, ActiveWindowName,
            CurrentLayout, Workspaces,
        },
        StatusBar,
    },
    core::TextStyle,
};

/// Holds all subsidary structures that make up the status bar,
/// the user should not need to edit this file.
#[derive(Debug)]
pub struct CaseWmStatusBar {
    /// Users bar config
    bar_config: BarConfig,
    /// Users color scheme
    color_scheme: ColorSchemeConfig,
    /// Style of the bar,
    style: TextStyle,
}

impl CaseWmStatusBar {
    /// Builds the status bar
    pub fn new() -> Self {
        let bar_config = BarConfig::new();
        let color_scheme = ColorSchemeConfig::new();
        let style = TextStyle {
            font: bar_config.font().to_owned(),
            point_size: bar_config.font_point_size(),
            fg: color_scheme.foreground(),
            bg: Some(color_scheme.background()),
            padding: bar_config.padding(),
        };
        Self {
            bar_config,
            color_scheme,
            style,
        }
    }

    /// Builds a DWM style baked in status bar
    pub fn build_status_bar<X: XConn>(&self) -> penrose_ui::Result<StatusBar<X>> {
        let widget_padding = TextStyle {
            padding: (4.0, 2.0),
            ..self.style.clone()
        };

        StatusBar::try_new(
            self.bar_config.position(),
            self.bar_config.bar_height(),
            self.style.bg.unwrap_or_else(|| 0x0000.into()),
            &[&self.style.font],
            vec![
                Box::new(Workspaces::new(
                    &self.style,
                    self.color_scheme.primary(),
                    self.color_scheme.inactive(),
                )),
                Box::new(CurrentLayout::new(&self.style)),
                //TODO: Load this dependant on debug cli flag
                //Box::new(StateSummary::new(&self.style)),
                Box::new(ActiveWindowName::new(
                    self.bar_config.max_active_window_chars(),
                    &TextStyle {
                        bg: Some(self.color_scheme.primary()),
                        padding: (6.0, 4.0),
                        ..self.style.clone()
                    },
                    true,
                    false,
                )),
                Box::new(wifi_network(&widget_padding)),
                Box::new(battery_summary("BAT1", &widget_padding)),
                Box::new(battery_summary("BAT0", &widget_padding)),
                Box::new(amixer_volume("Master", &widget_padding)),
                Box::new(current_date_and_time(&widget_padding)),
            ],
        )
    }
}
