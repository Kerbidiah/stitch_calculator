pub fn menu_bar(ctx: &egui::Context, frame: &mut eframe::Frame) {
	egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
		egui::menu::bar(ui, |ui| {
			ui.menu_button("File", |ui| {
				if ui.button("Quit").clicked() {
					frame.close();
				}
			});
		});
	});
}