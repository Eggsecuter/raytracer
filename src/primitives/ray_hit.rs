use std::fmt::{Display, Formatter, Result};
use crate::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayHit {
	pub distance: f32,
	pub point: Vector3,
	pub normal: Vector3
}

impl RayHit {
	pub fn new(distance: f32, point: Vector3, normal: Vector3) -> Self {
		Self { distance, point, normal }
	}
}

impl Display for RayHit {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "RayHit[{}, {}, {}]", self.distance, self.point, self.normal)
	}
}
