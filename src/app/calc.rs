use crate::app;

impl app::StitchApp {
	pub fn get_lengths(&self) -> Vec<u64> {
		let base = self.border * 2 + self.foundation_chain;

		let mut arr: Vec<u64> = (0..=u64::MAX)
			.map(|x| {
				self.pattern_width * x + base
			})
			.take_while(|x| {
				x <= &(self.max_stitches + self.foundation_chain)
			}).collect();

		arr.retain(|x| x >= &(self.min_stiches + self.foundation_chain));

		arr
	}
}