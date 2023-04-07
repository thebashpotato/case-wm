//! Configure and customize the baked in status bar

use crate::config::{BarConfig, ColorScheme};
use penrose::{x::XConn, Color};
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
pub struct CaseWindowManagerStatusBar {
    /// Users bar config
    bar_config: BarConfig,
    /// Users color scheme
    color_scheme: ColorScheme,
    /// Style of the bar, uses the built in `penrose_ui::core::TextStyle` widget
    style: TextStyle,
}

impl CaseWindowManagerStatusBar {
    /// Builds the status bar
    pub fn new() -> Self {
        let bar_config = BarConfig::new();
        let color_scheme = ColorScheme::new();
        let style = TextStyle {
            font: bar_config.font().to_owned(),
            point_size: 8,
            fg: color_scheme.white().into(),
            bg: Some(color_scheme.black().into()),
            padding: (2.0, 2.0),
        };
        Self {
            bar_config,
            color_scheme,
            style,
        }
    }

    /// Builds a DWM style baked in status bar
    pub fn build_status_bar<X: XConn>(&self) -> penrose_ui::Result<StatusBar<X>> {
        let highlight: Color = self.color_scheme.blue().into();
        let empty_ws: Color = self.color_scheme.grey().into();

        let padded_style = TextStyle {
            padding: (4.0, 2.0),
            ..self.style.clone()
        };

        StatusBar::try_new(
            self.bar_config.position(),
            self.bar_config.bar_height_pixel(),
            self.style.bg.unwrap_or_else(|| 0x00000.into()),
            &[&self.style.font],
            vec![
                Box::new(Workspaces::new(&self.style, highlight, empty_ws)),
                Box::new(CurrentLayout::new(&self.style)),
                // Box::new(penrose_bar::widgets::debug::StateSummary::new(style)),
                Box::new(ActiveWindowName::new(
                    self.bar_config.max_active_window_chars(),
                    &TextStyle {
                        bg: Some(highlight),
                        padding: (6.0, 4.0),
                        ..self.style.clone()
                    },
                    true,
                    false,
                )),
                Box::new(wifi_network(&padded_style)),
                Box::new(battery_summary("BAT1", &padded_style)),
                Box::new(battery_summary("BAT0", &padded_style)),
                Box::new(amixer_volume("Master", &padded_style)),
                Box::new(current_date_and_time(&padded_style)),
            ],
        )
    }
}
