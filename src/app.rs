use crate::stitch;
use crate::ui_element::*;

#[derive(Debug)]
pub struct StitchApp {
	specs: stitch::Specs,
	specs_hash: u64,
	lengths: Vec<u64>
}

impl Default for StitchApp {
	fn default() -> Self {
		let specs = stitch::Specs::default();
		let specs_hash = specs.hash();
		let lengths = specs.get_lengths();

		Self {
			specs,
			specs_hash,
			lengths,
		}
	}
}

impl StitchApp {
	/// Called once before the first frame.
	#[allow(unused_variables)]
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is where you can customize the look at feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		Self::default()
	}
}

impl eframe::App for StitchApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		menu_bar::menu_bar(ctx, frame);
		left_panel::left_panel(ctx, &mut self.specs);

		// update lengths and hash if need be
		if !self.specs.check_matching_hash(self.specs_hash) {
			self.specs_hash = self.specs.hash();
			self.lengths = self.specs.get_lengths();
		}

		center_panel::center_panel(ctx, &self.lengths);
	}
}
