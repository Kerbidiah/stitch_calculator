#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;
pub mod stitch;
pub mod ui_element;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
	env_logger::init();

	let native_options = eframe::NativeOptions {
		default_theme: eframe::Theme::Light,
		..Default::default()
	};

	eframe::run_native(
		"Stitch Calculator App",
		native_options,
		Box::new(|cc| Box::new(app::StitchApp::new(cc))),
	)?;

	Ok(())
}
