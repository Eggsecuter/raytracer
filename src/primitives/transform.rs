use std::fmt::{Display, Formatter, Result};
use crate::{Vector3, primitives::Quaternion};

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
	pub position: Vector3,
	pub rotation: Quaternion
}

impl Transform {
	pub fn new(position: Option<Vector3>, rotation: Option<Quaternion>) -> Self {
		Self {
			position: position.unwrap_or(Vector3::ZERO),
			rotation: rotation.unwrap_or(Quaternion::IDENTITY),
		}
	}
}

impl Display for Transform {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Transform[{}, {}]", self.position, self.rotation)
	}
}
