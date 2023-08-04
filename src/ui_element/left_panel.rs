use crate::stitch::Specs;

pub fn left_panel(ctx: &egui::Context, frame: &mut eframe::Frame, specs: &mut Specs) {
	egui::SidePanel::left("left panel").show(ctx, |ui| {
		ui.heading("Parameters");
		ui.separator();
		
		specs.editing_ui(ui);

		ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
			egui::warn_if_debug_build(ui);
			ui.weak(format!("version: {}", env!("CARGO_PKG_VERSION")));

			if ui.button("reset").clicked() {
				*specs = Specs::default();
			}
		});
	});
}


