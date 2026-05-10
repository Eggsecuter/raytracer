use std::fmt::{Display, Formatter, Result};

use crate::primitives::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LambertMaterial {
	pub ambient: Color,
	pub albedo: Color
}

impl LambertMaterial {
	#[allow(dead_code)]
	pub fn new(ambient: Color, albedo: Color) -> Self {
		Self { ambient, albedo }
	}

	#[allow(dead_code)]
	pub fn from_color(color: Color) -> Self {
		Self {
			ambient: color  * 0.1,
			albedo: color
		}
	}
}

impl Display for LambertMaterial {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"LambertMaterial[{}, {}]",
			self.ambient, self.albedo
		)
	}
}
