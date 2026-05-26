use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
}

#[allow(dead_code)]
impl Color {
	pub const BLACK: Color = Color {
		red: 0.0,
		green: 0.0,
		blue: 0.0,
	};
	pub const WHITE: Color = Color {
		red: 1.0,
		green: 1.0,
		blue: 1.0,
	};
	pub const RED: Color = Color {
		red: 1.0,
		green: 0.0,
		blue: 0.0,
	};
	pub const GREEN: Color = Color {
		red: 0.0,
		green: 1.0,
		blue: 0.0,
	};
	pub const BLUE: Color = Color {
		red: 0.0,
		green: 0.0,
		blue: 1.0,
	};

	pub const YELLOW: Color = Color {
		red: 1.0,
		green: 1.0,
		blue: 0.0,
	};
	pub const MAGENTA: Color = Color {
		red: 1.0,
		green: 0.0,
		blue: 1.0,
	};
	pub const CYAN: Color = Color {
		red: 0.0,
		green: 1.0,
		blue: 1.0,
	};
}

impl Color {
	pub fn new(red: f32, green: f32, blue: f32) -> Self {
		Self {
			red: red,
			green: green,
			blue: blue,
		}
	}

	pub fn srgb_to_linear(self) -> Self {
		self.powf(2.2)
	}

	pub fn linear_to_srgb(self) -> Self {
		self.powf(1.0 / 2.2)
	}

	pub fn hdr_to_ldr(self, exposure: Option<f32>) -> Self {
		let exposure = exposure.unwrap_or(0.0);
		let hdr_color = self * 2.0_f32.powf(exposure);

		hdr_color / (hdr_color + 1.0)
	}

	pub fn exp(self) -> Self {
		Self {
			red: self.red.exp(),
			green: self.green.exp(),
			blue: self.blue.exp(),
		}
	}

	fn powf(self, exp: f32) -> Self {
		Self {
			red: self.red.powf(exp),
			green: self.green.powf(exp),
			blue: self.blue.powf(exp),
		}
	}
}

impl Add for Color {
	type Output = Color;

	fn add(self, other: Color) -> Color {
		Color::new(
			self.red + other.red,
			self.green + other.green,
			self.blue + other.blue,
		)
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, other: Color) {
		*self = *self + other
	}
}

impl Add<f32> for Color {
	type Output = Color;

	fn add(self, rhs: f32) -> Self::Output {
		Color {
			red: self.red + rhs,
			green: self.green + rhs,
			blue: self.blue + rhs,
		}
	}
}

impl Mul for Color {
	type Output = Color;

	fn mul(self, other: Color) -> Color {
		Color::new(
			self.red * other.red,
			self.green * other.green,
			self.blue * other.blue,
		)
	}
}

impl MulAssign for Color {
	fn mul_assign(&mut self, other: Color) {
		*self = *self * other
	}
}

impl Mul<f32> for Color {
	type Output = Color;

	fn mul(self, scalar: f32) -> Color {
		Color::new(self.red * scalar, self.green * scalar, self.blue * scalar)
	}
}

impl MulAssign<f32> for Color {
	fn mul_assign(&mut self, scalar: f32) {
		*self = *self * scalar
	}
}

impl Div for Color {
	type Output = Color;

	fn div(self, other: Color) -> Self::Output {
		Color {
			red: self.red / other.red,
			green: self.green / other.green,
			blue: self.blue / other.blue,
		}
	}
}

impl Display for Color {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Color[{}, {}, {}]", self.red, self.green, self.blue)
	}
}
