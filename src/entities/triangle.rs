use crate::entities::Entity;
use crate::materials::Material;
use crate::primitives::*;
use crate::primitives::{Aabb, Quaternion};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
	pub material: Material,
	pub v0: Vector3,
	pub edge1: Vector3,
	pub edge2: Vector3,
	pub normal: Vector3,
}

impl Triangle {
	pub fn new(
		v0: Vector3,
		v1: Vector3,
		v2: Vector3,
		material: Material,
		normal: Option<Vector3>,
	) -> Self {
		let edge1 = v1 - v0;
		let edge2 = v2 - v0;
		let normal = normal.unwrap_or_else(|| edge1.cross(edge2)).normalize();

		Self {
			material,
			v0,
			edge1,
			edge2,
			normal,
		}
	}

	pub fn translated(&self, offset: Vector3) -> Self {
		Self {
			v0: self.v0 + offset,
			edge1: self.edge1,
			edge2: self.edge2,
			normal: self.normal,
			material: self.material,
		}
	}

	pub fn rotated(&self, rotation: Quaternion) -> Self {
		let v1 = self.v0 + self.edge1;
		let v2 = self.v0 + self.edge2;
		let new_v0 = rotation.rotate_vector(self.v0);
		let new_v1 = rotation.rotate_vector(v1);
		let new_v2 = rotation.rotate_vector(v2);
		Self {
			v0: new_v0,
			edge1: new_v1 - new_v0,
			edge2: new_v2 - new_v0,
			normal: rotation.rotate_vector(self.normal),
			material: self.material,
		}
	}

	pub fn scaled(&self, factor: f32) -> Self {
		Self {
			v0: self.v0 * factor,
			edge1: self.edge1 * factor,
			edge2: self.edge2 * factor,
			normal: self.normal,
			material: self.material,
		}
	}
}

impl Entity for Triangle {
	fn bounding_box(&self) -> Aabb {
		let v1 = self.v0 + self.edge1;
		let v2 = self.v0 + self.edge2;
		// Small epsilon padding prevents degenerate flat AABBs (e.g. axis-aligned triangles).
		let eps = Vector3::new(1e-4, 1e-4, 1e-4);
		let min = Vector3::new(
			self.v0.x.min(v1.x).min(v2.x),
			self.v0.y.min(v1.y).min(v2.y),
			self.v0.z.min(v1.z).min(v2.z),
		) - eps;
		let max = Vector3::new(
			self.v0.x.max(v1.x).max(v2.x),
			self.v0.y.max(v1.y).max(v2.y),
			self.v0.z.max(v1.z).max(v2.z),
		) + eps;
		Aabb::new(min, max)
	}

	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		let perpendicular_vector = ray.direction.cross(self.edge2);
		let determinant = self.edge1.dot(&perpendicular_vector);

		let epsilon = 1e-6;

		if ray.check_front && determinant <= epsilon || !ray.check_front && determinant >= -epsilon
		{
			return None;
		}

		let inverse_determinant = 1.0 / determinant;
		let origin_to_vertex = ray.origin - self.v0;
		let barycentric_u = origin_to_vertex.dot(&perpendicular_vector) * inverse_determinant;

		if !(0.0..=1.0).contains(&barycentric_u) {
			return None;
		}

		let cross_vector = origin_to_vertex.cross(self.edge1);
		let barycentric_v = ray.direction.dot(&cross_vector) * inverse_determinant;

		if barycentric_v < 0.0 || barycentric_u + barycentric_v > 1.0 {
			return None;
		}

		let distance_along_ray = self.edge2.dot(&cross_vector) * inverse_determinant;

		if distance_along_ray < 0.0 {
			return None;
		}

		let intersection_point = ray.origin + ray.direction * distance_along_ray;

		let normal = if ray.check_front {
			self.normal
		} else {
			-self.normal
		};

		Some(RayHit::new(
			distance_along_ray,
			intersection_point,
			normal,
			self.material,
			ray.check_front,
		))
	}
}
