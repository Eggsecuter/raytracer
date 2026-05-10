use crate::primitives::Vector3;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
	pub origin: Vector3,
	pub direction: Vector3,
	pub check_front: bool
}

impl Ray {
	pub fn new(origin: Vector3, direction: Vector3, check_front: bool) -> Self {
		Self { origin, direction, check_front }
	}
}

impl Display for Ray {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Ray[{}, {}, {}]", self.origin, self.direction, self.check_front)
	}
}
