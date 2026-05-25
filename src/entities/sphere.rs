use crate::entities::Entity;
use crate::materials::Material;
use crate::primitives::{Aabb, Ray, RayHit, Transform, Vector3, UV};

#[derive(Debug)]
pub struct Sphere {
	pub material: Material,
	pub transform: Transform,
	pub radius: f32,
}

impl Sphere {
	pub fn new(material: Material, transform: Transform, radius: f32) -> Self {
		Self {
			material,
			transform,
			radius,
		}
	}
}

impl Entity for Sphere {
	fn bounding_box(&self) -> Aabb {
		let r = Vector3::new(self.radius, self.radius, self.radius);
		Aabb::new(self.transform.position - r, self.transform.position + r)
	}

	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		let origin_to_center = ray.origin - self.transform.position;

		let direction_len_sq = ray.direction.dot(&ray.direction);
		let projection_len = 2.0 * ray.direction.dot(&origin_to_center);
		let center_dist_sq = origin_to_center.dot(&origin_to_center) - self.radius * self.radius;

		let discriminant =
			projection_len * projection_len - 4.0 * direction_len_sq * center_dist_sq;

		let epsilon = 1e-6;

		if discriminant <= epsilon {
			return None;
		}

		let sqrt_discriminant = discriminant.sqrt();
		let denominator = 2.0 * direction_len_sq;

		let first_distance = (-projection_len - sqrt_discriminant) / denominator;
		let second_distance = (-projection_len + sqrt_discriminant) / denominator;

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

		let hit_point = ray.origin + ray.direction * hit_distance;
		let surface_normal = (hit_point - self.transform.position).normalize();

		let front_face = ray.direction.dot(&surface_normal) < 0.0;

		if front_face != ray.check_front {
			return None;
		}

		let normal = if ray.check_front {
			surface_normal
		} else {
			-surface_normal
		};

		let uv = sphere_uv(surface_normal);

		Some(RayHit::new(
			hit_distance,
			hit_point,
			normal,
			self.material.clone(),
			front_face,
			uv,
		))
	}
}

fn sphere_uv(n: Vector3) -> UV {
	use std::f32::consts::{PI, TAU};
	let u = (f32::atan2(-n.z, n.x) + PI) / TAU;
	let v = (n.y.clamp(-1.0, 1.0).asin() + PI / 2.0) / PI;

	UV::new(u, v)
}
