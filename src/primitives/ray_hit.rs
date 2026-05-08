use crate::{materials::Material, primitives::{Vector3}};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayHit {
	pub distance: f32,
	pub point: Vector3,
	pub normal: Vector3,
	pub material: Material
}

impl RayHit {
	pub fn new(distance: f32, point: Vector3, normal: Vector3, material: Material) -> Self {
		Self {
			distance,
			point,
			normal,
			material
		}
	}
}

impl Display for RayHit {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"RayHit[{}, {}, {}, {}]",
			self.distance, self.point, self.normal, self.material
		)
	}
}
