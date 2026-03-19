use crate::Color;
use crate::Transform;
use crate::entities::Entity;
use crate::primitives::Ray;
use crate::primitives::RayHit;

#[derive(Debug, PartialEq)]
pub struct Sphere {
	pub color: Color,
	pub transform: Transform,
	pub radius: f32,
}

impl Sphere {
	pub fn new(color: Color, transform: Transform, radius: f32) -> Self {
		Self { color, transform, radius }
	}
}

impl Entity for Sphere {
	fn color(&self) -> Color {
		self.color
	}

	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		// Vector from sphere center to ray origin
		let origin_to_center = ray.origin - self.transform.position;

		// Quadratic coefficients
		let direction_len_sq = ray.direction.dot(&ray.direction);
		let projection_len = 2.0 * ray.direction.dot(&origin_to_center);
		let center_dist_sq = origin_to_center.dot(&origin_to_center) - self.radius * self.radius;

		let discriminant = projection_len * projection_len - 4.0 * direction_len_sq * center_dist_sq;

		// No intersection
		if discriminant < 0.0 {
			return None;
		}

		let sqrt_discriminant = discriminant.sqrt();
		let denominator = 2.0 * direction_len_sq;

		let first_distance = (-projection_len - sqrt_discriminant) / denominator;
		let second_distance = (-projection_len + sqrt_discriminant) / denominator;

		// Choose nearest valid hit in front of ray
		let mut hit_distance = f32::INFINITY;

		if first_distance > 0.0 {
			hit_distance = first_distance;
		}

		if second_distance > 0.0 && second_distance < hit_distance {
			hit_distance = second_distance;
		}

		if !hit_distance.is_finite() {
			return None;
		}

		// Compute hit point and surface normal
		let hit_point = ray.origin + ray.direction * hit_distance;
		let surface_normal = (hit_point - self.transform.position).normalize();

		Some(RayHit::new(hit_distance, hit_point, surface_normal))
	}
}
