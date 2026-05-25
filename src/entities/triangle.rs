use crate::entities::Entity;
use crate::materials::Material;
use crate::primitives::*;
use crate::primitives::{Aabb, Quaternion, UV};

#[derive(Debug, Clone)]
pub struct Triangle {
	pub material: Material,
	pub v0: Vector3,
	pub edge1: Vector3,
	pub edge2: Vector3,
	pub normal0: Vector3,
	pub normal1: Vector3,
	pub normal2: Vector3,

	pub uv0: UV,
	pub uv1: UV,
	pub uv2: UV,
}

impl Triangle {
	#[allow(dead_code)]
	pub fn new(
		v0: Vector3,
		v1: Vector3,
		v2: Vector3,
		material: Material,
		normal: Option<Vector3>,
	) -> Self {
		Self::with_uvs(
			v0,
			v1,
			v2,
			UV::new(0.0, 0.0),
			UV::new(1.0, 0.0),
			UV::new(0.0, 1.0),
			material,
			normal,
		)
	}

	pub fn with_uvs(
		v0: Vector3,
		v1: Vector3,
		v2: Vector3,
		uv0: UV,
		uv1: UV,
		uv2: UV,
		material: Material,
		normal: Option<Vector3>,
	) -> Self {
		let edge1 = v1 - v0;
		let edge2 = v2 - v0;
		let normal = normal.unwrap_or_else(|| edge1.cross(edge2)).normalize();

		// use the same normal for all vertices (flat shading)
		Self {
			material,
			v0,
			edge1,
			edge2,
			normal0: normal,
			normal1: normal,
			normal2: normal,
			uv0,
			uv1,
			uv2,
		}
	}

	/// creates a triangle with per-vertex normals for smooth shading
	pub fn with_normals(
		v0: Vector3,
		v1: Vector3,
		v2: Vector3,
		uv0: UV,
		uv1: UV,
		uv2: UV,
		material: Material,
		normal0: Vector3,
		normal1: Vector3,
		normal2: Vector3,
	) -> Self {
		let edge1 = v1 - v0;
		let edge2 = v2 - v0;

		Self {
			material,
			v0,
			edge1,
			edge2,
			normal0: normal0.normalize(),
			normal1: normal1.normalize(),
			normal2: normal2.normalize(),
			uv0,
			uv1,
			uv2,
		}
	}

	pub fn translated(&self, offset: Vector3) -> Self {
		Self {
			v0: self.v0 + offset,
			edge1: self.edge1,
			edge2: self.edge2,
			normal0: self.normal0,
			normal1: self.normal1,
			normal2: self.normal2,
			material: self.material.clone(),
			uv0: self.uv0,
			uv1: self.uv1,
			uv2: self.uv2,
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
			normal0: rotation.rotate_vector(self.normal0),
			normal1: rotation.rotate_vector(self.normal1),
			normal2: rotation.rotate_vector(self.normal2),
			material: self.material.clone(),
			uv0: self.uv0,
			uv1: self.uv1,
			uv2: self.uv2,
		}
	}

	pub fn scaled(&self, factor: f32) -> Self {
		Self {
			v0: self.v0 * factor,
			edge1: self.edge1 * factor,
			edge2: self.edge2 * factor,
			normal0: self.normal0,
			normal1: self.normal1,
			normal2: self.normal2,
			material: self.material.clone(),
			uv0: self.uv0,
			uv1: self.uv1,
			uv2: self.uv2,
		}
	}
}

impl Entity for Triangle {
	fn bounding_box(&self) -> Aabb {
		let v1 = self.v0 + self.edge1;
		let v2 = self.v0 + self.edge2;
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

		// interpolate the normal using barycentric coordinates
		let barycentric_w = 1.0 - barycentric_u - barycentric_v;
		let interpolated_normal = (self.normal0 * barycentric_w
			+ self.normal1 * barycentric_u
			+ self.normal2 * barycentric_v)
			.normalize();

		let normal = if ray.check_front {
			interpolated_normal
		} else {
			-interpolated_normal
		};

		let uv = UV::barycentric(self.uv0, self.uv1, self.uv2, barycentric_u, barycentric_v);

		Some(RayHit::new(
			distance_along_ray,
			intersection_point,
			normal,
			self.material.clone(),
			ray.check_front,
			uv,
		))
	}
}
