use std::fmt::format;

use serde::{Deserialize, Serialize};

mod length;
mod calc;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct StitchApp {
	// Example stuff:
	pattern_width: u64,
	border: u64,
	min_stiches: u64,
	max_stitches: u64,
	foundation_chain: u64,
	// units: String,

	// this how you opt-out of serialization of a member
	// #[serde(skip)]
}

impl Default for StitchApp {
	fn default() -> Self {
		Self {
			pattern_width: 2,
			border: 0,
			min_stiches: 10,
			max_stitches: 100,
			foundation_chain: 1,
			// units: "sts".to_string(),
		}
	}
}

const APP_KEY: &str = "stitch app";

impl StitchApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customized the look at feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, APP_KEY).unwrap_or_default();
		} else {
			Self::default()
		}

	}
}

impl eframe::App for StitchApp {
	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, APP_KEY, self);
	}

	/// Called each time the UI needs repainting, which may be many times per second.
	/// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
	#[allow(unused_variables)]
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

		// // menu bar
		// #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
		// egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
		// 	egui::menu::bar(ui, |ui| {
		// 		ui.menu_button("File", |ui| {
		// 			if ui.button("Quit").clicked() {
		// 				frame.close();
		// 			}
		// 		});
		// 	});
		// });

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			ui.heading("Parameters");
			
			let mut d_spacing = egui::style::Spacing::default().item_spacing;
			d_spacing.y = 6.0;

			egui::Grid::new("Settings Grid")
			.num_columns(2)
			.spacing(d_spacing)
			.show(ui, |ui| {
				ui.label("pattern repeat: ");
				ui.add(
					egui::DragValue::new(&mut self.pattern_width)
						.speed(0.0)
						.clamp_range(1..=u64::MAX)
						.suffix(" sts")
				);
				ui.end_row();

				ui.label("border width: ");
				ui.add(
					egui::DragValue::new(&mut self.border)
						.speed(0.0)
						.clamp_range(0..=u64::MAX)
						.suffix(" sts")
				);
				ui.end_row();

				ui.label("min stitches: ");
				ui.add(
					egui::DragValue::new(&mut self.min_stiches)
						.speed(0.0)
						.clamp_range(0..=self.max_stitches)
						.suffix(" sts")
				);
				ui.end_row();

				ui.label("max stitches: ");
				ui.add(
					egui::DragValue::new(&mut self.max_stitches)
						.speed(0.0)
						.clamp_range(self.min_stiches..=u64::MAX)
						.suffix(" sts")
				);
				ui.end_row();

				ui.label("foundation chain: ");
				ui.add(
					egui::DragValue::new(&mut self.foundation_chain)
						.speed(0.0)
						.clamp_range(0..=u64::MAX)
						.suffix(" sts")
				);
				ui.end_row();
			});

			// put debug warning on the bottom and version number
			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				egui::warn_if_debug_build(ui);
				ui.weak(format!("version: {}", env!("CARGO_PKG_VERSION")));
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's

			ui.heading("Stitches");

			for each in self.get_lengths() {
				ui.label(format!("{}", each));
			}
		});
	}
}
