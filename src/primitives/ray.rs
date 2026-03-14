use std::fmt::{Display, Formatter, Result};
use crate::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
	pub origin: Vector3,
	pub direction: Vector3
}

impl Ray {
	pub fn new(origin: Vector3, direction: Vector3) -> Self {
		Self { origin, direction }
	}
}

impl Display for Ray {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Ray[{}, {}]", self.origin, self.direction)
	}
}
