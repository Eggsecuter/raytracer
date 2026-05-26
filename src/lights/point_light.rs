use crate::lights::Light;
use crate::primitives::{Color, RayHit, Vector3};

#[derive(Debug, PartialEq)]
pub struct PointLight {
	pub color: Color,
	pub position: Vector3,
	pub intensity: f32,
}

impl PointLight {
	pub fn new(color: Color, position: Vector3, intensity: f32) -> Self {
		Self { color, position, intensity }
	}
}

impl Light for PointLight {
	fn position(&self) -> Vector3 {
		self.position
	}

	fn calculate_color(&self, ray_hit: &RayHit) -> Color {
		let to_light = self.position - ray_hit.point;
		let distance_squared = to_light.length_squared();

		let light_dir = to_light.normalize();
		let n_dot_length = light_dir.dot(&ray_hit.normal).max(0.0);

		// quadratic attenuation
		// artistic control 1.0 avoids extreme brightness at small distances
		let attenuation = self.intensity / (distance_squared + 1.0);

		self.color * (n_dot_length * attenuation)
	}
}
