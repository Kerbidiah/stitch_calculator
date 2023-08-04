use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use egui::Ui;

#[derive(Debug, Hash)]
pub struct Specs {
	pub pattern_width: u64,
	pub border: u64,
	pub min_stiches: u64,
	pub max_stitches: u64,
	pub foundation_chain: u64,
}

impl Default for Specs {
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

impl Specs {
	pub fn hash(&self) -> u64 {
		calculate_hash(&self)
	}

	#[inline]
	pub fn check_matching_hash(&self, hash_b: u64) -> bool {
		self.hash() == hash_b
	}

	// TODO: optimize this code
	pub fn get_lengths(&self) -> Vec<u64> {
		let base = self.border * 2 + self.foundation_chain;

		let mut arr: Vec<u64> = (0..=u64::MAX)
			.map(|x| {
				self.pattern_width * x + base
			}).take_while(|x| {
				x <= &(self.max_stitches + self.foundation_chain)
			}).collect();

		arr.retain(|x| x >= &(self.min_stiches + self.foundation_chain));

		arr
	}

	pub fn editing_ui(&mut self, ui: &mut Ui) {
		// let mut d_spacing = egui::style::Spacing::default().item_spacing;
		// d_spacing.y = 6.0;

		egui::Grid::new("Settings Grid")
			.num_columns(2)
			// .spacing(d_spacing)
			.show(ui, |ui| {
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

				ui.separator();
				ui.end_row();

				ui.label("foundation chain: ");
				ui.add(
					egui::DragValue::new(&mut self.foundation_chain)
						.speed(0.0)
						.clamp_range(0..=u64::MAX)
						.suffix(" sts")
				);
				ui.end_row();

				ui.separator();
				ui.end_row();

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
			});
	}
}

#[inline]
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
