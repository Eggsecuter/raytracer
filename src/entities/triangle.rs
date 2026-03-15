use crate::entities::Entity;
use crate::primitives::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
	pub color: Color,
	pub v0: Vector3,
	pub edge1: Vector3,
	pub edge2: Vector3,
	pub normal: Vector3
}

impl Triangle {
	pub fn new(color: Color, v0: Vector3, v1: Vector3, v2: Vector3) -> Self {
		let edge1 = v1 - v0;
		let edge2 = v2 - v0;
		let normal = edge1.cross(edge2).normalize();

		Self { color, v0, edge1, edge2, normal }
	}
}

impl Entity for Triangle {
	fn color(&self) -> Color {
		self.color
	}

	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		let perpendicular_vector = ray.direction.cross(self.edge2);
		let determinant = self.edge1.dot(&perpendicular_vector);

		if determinant < 0.0 {
			return None;
		}

		let inverse_determinant = 1.0 / determinant;
		let origin_to_vertex = ray.origin - self.v0;
		let barycentric_u = origin_to_vertex.dot(&perpendicular_vector) * inverse_determinant;

		if barycentric_u < 0.0 || barycentric_u > 1.0 {
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

		Some(RayHit::new(
			distance_along_ray,
			intersection_point,
			self.normal
		))
	}
}
