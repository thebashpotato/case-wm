//! Custom and built in layouts
//! use crate::{BAR_HEIGHT_PX, INNER_PX, MAX_MAIN, OUTER_PX, RATIO, RATIO_STEP};

use crate::config::WindowConfig;
use penrose::{
    builtin::layout::{
        transformers::{Gaps, ReserveTop},
        MainAndStack, Monocle,
    },
    core::layout::LayoutStack,
    stack,
};

/// Holds all available layours
#[derive(Debug)]
pub struct Layout {
    /// window configuration options, inner and outer gaps etc..
    window_config: WindowConfig,
}

impl Layout {
    /// Constructs the object with a window config.
    pub const fn new() -> Self {
        Self {
            window_config: WindowConfig::new(),
        }
    }

    /// Layouts are added to this stack. Currently
    /// only Main stack and Monocle are supported.
    pub fn layouts(&self) -> LayoutStack {
        stack!(
            MainAndStack::side(
                self.window_config.max_main(),
                self.window_config.ratio(),
                self.window_config.ratio_step()
            ),
            MainAndStack::side_mirrored(
                self.window_config.max_main(),
                self.window_config.ratio(),
                self.window_config.ratio_step()
            ),
            MainAndStack::bottom(
                self.window_config.max_main(),
                self.window_config.ratio(),
                self.window_config.ratio_step()
            ),
            Monocle::boxed()
        )
        .map(|layout| {
            ReserveTop::wrap(
                Gaps::wrap(
                    layout,
                    self.window_config.outer_px(),
                    self.window_config.inner_px(),
                ),
                self.window_config.bar_height_px(),
            )
        })
    }
}
