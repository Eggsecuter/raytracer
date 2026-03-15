use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

#[allow(dead_code)]
impl Vector3 {
	pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
	pub const RIGHT: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
	pub const UP: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
	pub const LEFT: Vector3 = Vector3 { x: -1.0, y: 0.0, z: 0.0 };
	pub const DOWN: Vector3 = Vector3 { x: 0.0, y: -1.0, z: 0.0 };
	pub const FORWARD: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
	pub const BACKWARD: Vector3 = Vector3 { x: 0.0, y: 0.0, z: -1.0 };
}

impl Vector3 {
	pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
		Self { x, y, z }
	}

	pub fn length(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn normalize(&self) -> Vector3 {
		let length = self.length();

		if length == 0.0 {
			Vector3::ZERO
		} else {
			*self / length
		}
	}

	pub fn cross(&self, other: Vector3) -> Vector3 {
		Vector3::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x
		)
	}

	pub fn dot(&self, other: &Vector3) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn angle(&self, other: &Vector3) -> f64 {
		(self.dot(other) / (self.length() * other.length())).acos()
	}
}

impl Add for Vector3 {
	type Output = Vector3;

	fn add(self, other: Vector3) -> Vector3 {
		Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl AddAssign for Vector3 {
	fn add_assign(&mut self, other: Vector3) {
		*self = *self + other;
	}
}

impl Sub for Vector3 {
	type Output = Vector3;

	fn sub(self, other: Vector3) -> Vector3 {
		Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl SubAssign for Vector3 {
	fn sub_assign(&mut self, other: Vector3) {
		*self = *self - other;
	}
}

impl Mul for Vector3 {
	type Output = Vector3;

	fn mul(self, other: Vector3) -> Vector3 {
		Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
	}
}

impl MulAssign for Vector3 {
	fn mul_assign(&mut self, other: Vector3) {
		*self = *self * other;
	}
}

impl Mul<f64> for Vector3 {
	type Output = Vector3;

	fn mul(self, factor: f64) -> Vector3 {
		Vector3::new(self.x * factor, self.y * factor, self.z * factor)
	}
}

impl MulAssign<f64> for Vector3 {
	fn mul_assign(&mut self, other: f64) {
		*self = *self * other;
	}
}

impl Div for Vector3 {
	type Output = Vector3;

	fn div(self, other: Vector3) -> Vector3 {
		Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
	}
}

impl DivAssign for Vector3 {
	fn div_assign(&mut self, other: Vector3) {
		*self = *self / other;
	}
}

impl Div<f64> for Vector3 {
	type Output = Vector3;

	fn div(self, factor: f64) -> Vector3 {
		Vector3::new(self.x / factor, self.y / factor, self.z / factor)
	}
}

impl DivAssign<f64> for Vector3 {
	fn div_assign(&mut self, other: f64) {
		*self = *self / other;
	}
}

impl Neg for Vector3 {
	type Output = Vector3;

	fn neg(self) -> Vector3 {
		Vector3::new(-self.x, -self.y, -self.z)
	}
}

impl Display for Vector3 {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Vector3[{}, {}, {}]", self.x, self.y, self.z)
	}
}
