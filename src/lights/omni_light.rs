use crate::Color;
use crate::Vector3;
use crate::lights::Light;
use crate::primitives::RayHit;

#[derive(Debug, PartialEq)]
pub struct OmniLight {
	pub color: Color,
	pub position: Vector3
}

impl OmniLight {
	pub fn new(color: Color, position: Vector3) -> Self {
		Self { color, position }
	}
}

impl Light for OmniLight {
	fn position(&self) -> Vector3 {
		self.position
	}

	fn calculate_color(&self, ray_hit: &RayHit) -> Color {
		let intensity = (self.position - ray_hit.point).dot(&ray_hit.normal);

		self.color * intensity
	}
}
