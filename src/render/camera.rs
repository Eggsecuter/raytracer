use crate::primitives::ray::Ray;
use crate::primitives::transform::Transform;
use crate::primitives::vector3::Vector3;
use crate::utilities::angle::Angle;

#[derive(Debug, Clone)]
pub struct Camera {
	pub transform: Transform,
	pub half_height: f32,
	pub half_width: f32
}

impl Camera {
	pub fn new(transform: Option<Transform>, y_fov: Option<f32>, aspect_ratio: Option<f32>) -> Self {
		let y_fov = y_fov.unwrap_or(Angle::to_radian(60.0));
		let half_height = (y_fov / 2.0).tan();

		Self {
			transform: transform.unwrap_or(Transform::new(None, None)),
			half_height,
			half_width: half_height * aspect_ratio.unwrap_or(1.5)
		}
	}

	pub fn get_ray(&self, x: f32, y: f32, width: f32, height: f32) -> Ray {
		let pixel_center_offset = 0.5;

		// calculate normalized coordinates
		let u = 2.0 * (x + pixel_center_offset) / width - 1.0;
		let v = 1.0 - 2.0 * (y + pixel_center_offset) / height;

		let direction = self.transform.rotation.rotate_vector(Vector3::FORWARD)
			+ Vector3::RIGHT * u * self.half_width
			+ Vector3::UP * v * self.half_height;

		Ray::new(self.transform.position, direction.normalize())
	}
}
