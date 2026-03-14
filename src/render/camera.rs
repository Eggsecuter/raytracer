use crate::primitives::ray::Ray;
use crate::primitives::transform::Transform;
use crate::primitives::vector3::Vector3;
use crate::utilities::angle::Angle;

#[derive(Debug, Clone)]
pub struct Camera {
	pub transform: Transform,
	pub fov: f64,
}

impl Camera {
	pub fn new(transform: Option<Transform>, fov: Option<f64>) -> Self {
		Self {
			transform: transform.unwrap_or(Transform::new(None, None)),
			fov: fov.unwrap_or(Angle::to_radian(60.0)),
		}
	}

	pub fn get_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
		let aspect = width as f64 / height as f64;

		let px =
			(2.0 * (x as f64 + 0.5) / width as f64 - 1.0) * (self.fov / 2.0).tan() * aspect;

		let py =
			(1.0 - 2.0 * (y as f64 + 0.5) / height as f64) * (self.fov / 2.0).tan();

		let direction = Vector3::new(px, py, 1.0).normalize();

		let rotated_dir = self
			.transform
			.rotation
			.rotate_vector(direction)
			.normalize();

		Ray::new(self.transform.position, rotated_dir)
	}
}
