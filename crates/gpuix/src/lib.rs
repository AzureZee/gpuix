//! GPUI umbrella crate — unified entry point for the GPUI framework and platform abstractions.
//!
//! This crate re-exports everything from `gpui` and `gpui_platform`.

pub use gpui::*;

// Re-export from gpui_platform, but exclude Platform to avoid conflict with gpui::* re-export
pub use gpui_platform::{application, background_executor, current_platform, headless};

#[cfg(target_family = "wasm")]
pub use gpui_platform::{single_threaded_web, web_init};

#[cfg(feature = "test-support")]
pub use gpui_platform::current_headless_renderer;
