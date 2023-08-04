pub fn center_panel(ctx: &egui::Context, lengths: &Vec<u64>) {
	egui::CentralPanel::default().show(ctx, |ui| {
		ui.heading("Stitches");
		ui.separator();

		egui::ScrollArea::vertical()
		.auto_shrink([false, false])
		.show(ui, |ui| {
			for each in lengths {
				ui.label(format!("{}", each));
			}
		});
	});
}