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
	fn calculate_color(&self, rayHit: &RayHit) -> Color {
		let intensity = (self.position - rayHit.point).dot(&rayHit.normal);

		self.color * intensity
	}
}
