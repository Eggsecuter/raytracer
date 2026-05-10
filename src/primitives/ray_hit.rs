use crate::{materials::Material, primitives::{Vector3}};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayHit {
	pub distance: f32,
	pub point: Vector3,
	pub normal: Vector3,
	pub material: Material,
	pub front_face: bool
}

impl RayHit {
	pub fn new(distance: f32, point: Vector3, normal: Vector3, material: Material, front_face: bool) -> Self {
		Self {
			distance,
			point,
			normal,
			material,
			front_face
		}
	}
}

impl Display for RayHit {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"RayHit[{}, {}, {}, {}, {}]",
			self.distance, self.point, self.normal, self.material, self.front_face
		)
	}
}
