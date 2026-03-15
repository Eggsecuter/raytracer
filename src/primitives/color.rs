use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
	pub red: f64,
	pub green: f64,
	pub blue: f64,
}

#[allow(dead_code)]
impl Color {
	pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0 };
	pub const WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0 };
	pub const RED: Color = Color { red: 1.0, green: 0.0, blue: 0.0 };
	pub const GREEN: Color = Color { red: 0.0, green: 1.0, blue: 0.0 };
	pub const BLUE: Color = Color { red: 0.0, green: 0.0, blue: 1.0 };

	pub const YELLOW: Color = Color { red: 1.0, green: 1.0, blue: 0.0 };
	pub const MAGENTA: Color = Color { red: 1.0, green: 0.0, blue: 1.0 };
	pub const CYAN: Color = Color { red: 0.0, green: 1.0, blue: 1.0 };
}

impl Color {
	pub fn new(red: f64, green: f64, blue: f64) -> Self {
		Self {
			red: Self::clamp(red),
			green: Self::clamp(green),
			blue: Self::clamp(blue),
		}
	}

	pub fn from_grayscale(intensity: f64) -> Self {
		Self::new(intensity, intensity, intensity)
	}

	fn clamp(amount: f64) -> f64 {
		if amount > 1.0 {
			1.0
		} else if amount < 0.0 {
			0.0
		} else {
			amount
		}
	}
}

impl Add for Color {
	type Output = Color;

	fn add(self, other: Color) -> Color {
		Color::new(
			self.red + other.red,
			self.green + other.green,
			self.blue + other.blue
		)
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, other: Color) {
		*self = *self + other
	}
}

impl Mul for Color {
	type Output = Color;

	fn mul(self, other: Color) -> Color {
		Color::new(
			self.red * other.red,
			self.green * other.green,
			self.blue * other.blue
		)
	}
}

impl MulAssign for Color {
	fn mul_assign(&mut self, other: Color) {
		*self = *self * other
	}
}

impl Mul<f64> for Color {
	type Output = Color;

	fn mul(self, scalar: f64) -> Color {
		Color::new(
			self.red * scalar,
			self.green * scalar,
			self.blue * scalar
		)
	}
}

impl MulAssign<f64> for Color {
	fn mul_assign(&mut self, scalar: f64) {
		*self = *self * scalar
	}
}

impl Display for Color {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Color[{}, {}, {}]", self.red, self.green, self.blue)
	}
}
