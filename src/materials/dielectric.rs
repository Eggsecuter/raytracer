use std::fmt::{Display, Formatter, Result};

use crate::primitives::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DielectricMaterial {
	pub absorption: Color,
	pub refractive_index: f32
}

impl DielectricMaterial {
	#[allow(dead_code)]
	pub fn new(absorption: Color, refractive_index: f32) -> Self {
		Self { absorption, refractive_index }
	}
}

impl Display for DielectricMaterial {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"DielectricMaterial[{}, {}]",
			self.absorption, self.refractive_index
		)
	}
}
