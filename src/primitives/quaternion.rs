use std::fmt::{Display, Formatter, Result};
use crate::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}

#[allow(dead_code)]
impl Quaternion {
	pub const IDENTITY: Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
}

impl Quaternion {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self { x, y, z, w }
	}

	pub fn from_euler(euler: Vector3) -> Self {
		let (cx, sx) = ((euler.x * 0.5).cos(), (euler.x * 0.5).sin());
		let (cy, sy) = ((euler.y * 0.5).cos(), (euler.y * 0.5).sin());
		let (cz, sz) = ((euler.z * 0.5).cos(), (euler.z * 0.5).sin());

		Self::new(
			sx * cy * cz + cx * sy * sz,
			cx * sy * cz - sx * cy * sz,
			cx * cy * sz + sx * sy * cz,
			cx * cy * cz - sx * sy * sz
		)
	}

	pub fn multiply(&self, other: Quaternion) -> Quaternion {
		Quaternion::new(
			self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
			self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
			self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
			self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z
		)
	}

	pub fn conjugate(&self) -> Quaternion {
		Quaternion::new(-self.x, -self.y, -self.z, self.w)
	}

	pub fn normalize(&self) -> Quaternion {
		let length = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();

		Quaternion::new(self.x / length, self.y / length, self.z / length, self.w / length)
	}

	pub fn rotate_vector(&self, delta: Vector3) -> Vector3 {
		let delta_q = Quaternion::new(delta.x, delta.y, delta.z, 0.0);
		let result = self.multiply(delta_q).multiply(self.conjugate());

		Vector3::new(result.x, result.y, result.z)
	}
}

impl Display for Quaternion {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Quaternion[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
	}
}
