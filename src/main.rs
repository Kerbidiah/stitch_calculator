#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe;
use env_logger;

pub mod app;
pub mod stitch;
pub mod ui_element;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
	env_logger::init();

	let mut native_options = eframe::NativeOptions::default();
	native_options.default_theme = eframe::Theme::Light;

	eframe::run_native(
		"Stitch Calculator App",
		native_options,
		Box::new(|cc| Box::new(app::StitchApp::new(cc))),
	)?;

	Ok(())
}
