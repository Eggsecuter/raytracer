use std::fmt::{Display, Formatter, Result};

use crate::primitives::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetalMaterial {
	pub specular: Color,
	pub smoothness: f32
}

impl MetalMaterial {
	#[allow(dead_code)]
	pub fn new(specular: Color, smoothness: f32) -> Self {
		Self { specular, smoothness }
	}
}

#[allow(dead_code)]
impl MetalMaterial {
	pub const MIRROR: MetalMaterial = MetalMaterial {
		specular: Color::WHITE,
		smoothness: 1.0
	};

	pub const GOLD: MetalMaterial = MetalMaterial {
		specular: Color { red: 1.0, green: 0.84313, blue: 0.0 },
		smoothness: 0.9
	};

	pub const SILVER: MetalMaterial = MetalMaterial {
		specular: Color { red: 0.7529, green: 0.7529, blue: 0.7529 },
		smoothness: 0.85
	};
}

impl Display for MetalMaterial {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"MetalMaterial[{}, {}]",
			self.specular, self.smoothness
		)
	}
}
