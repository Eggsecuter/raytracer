use std::fmt::{Display, Formatter, Result};

use crate::primitives::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DielectricMaterial {
	pub absorption: Color,
	pub refractive_index: f32,
}

impl DielectricMaterial {
	#[allow(dead_code)]
	pub fn new(absorption: Color, refractive_index: f32) -> Self {
		Self {
			absorption,
			refractive_index,
		}
	}
}

#[allow(dead_code)]
impl DielectricMaterial {
	pub const WATER: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 0.15,
			green: 0.06,
			blue: 0.02,
		},
		refractive_index: 1.333,
	};

	pub const GLASS: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 0.0,
			green: 0.0,
			blue: 0.0,
		},
		refractive_index: 1.4,
	};

	pub const GLASS_TRANSPARENT: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 0.0,
			green: 0.0,
			blue: 0.0,
		},
		refractive_index: 1.03,
	};

	pub const GREEN_GLASS: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 0.6,
			green: 0.1,
			blue: 0.5,
		},
		refractive_index: 1.4,
	};

	pub const BROWN_GLASS: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 0.1,
			green: 0.4,
			blue: 1.2,
		},
		refractive_index: 1.4,
	};

	pub const DIAMOND: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 0.0,
			green: 0.0,
			blue: 0.0,
		},
		refractive_index: 2.417,
	};

	pub const EMERALD: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 10.0,
			green: 0.2,
			blue: 8.0,
		},
		refractive_index: 1.57,
	};

	pub const SAPPHIRE: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 1.0,
			green: 0.7,
			blue: 0.05,
		},
		refractive_index: 1.76,
	};

	pub const RUBY: DielectricMaterial = DielectricMaterial {
		absorption: Color {
			red: 0.05,
			green: 0.9,
			blue: 1.1,
		},
		refractive_index: 1.77,
	};
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
