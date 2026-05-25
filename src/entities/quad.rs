use crate::entities::{Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Aabb, Ray, RayHit, Vector3, UV};

#[derive(Debug, Clone)]
pub struct Quad {
	pub triangles: [Triangle; 2],
}

impl Quad {
	/// Build a planar quad from four corners in order A → B → C → D.
	///
	/// UV assignment:
	/// ```text
	///  D(0,1) --- C(1,1)
	///    |    \    |
	///  A(0,0) --- B(1,0)
	/// ```
	/// The quad is split into triangles ABC and ACD, preserving the UV mapping
	/// seamlessly across both halves.
	pub fn new(
		a: Vector3,
		b: Vector3,
		c: Vector3,
		d: Vector3,
		material: Material,
		normal: Option<Vector3>,
	) -> Self {
		let uv_a = UV::new(0.0, 0.0);
		let uv_b = UV::new(1.0, 0.0);
		let uv_c = UV::new(1.0, 1.0);
		let uv_d = UV::new(0.0, 1.0);

		Self {
			triangles: [
				Triangle::with_uvs(a, b, c, uv_a, uv_b, uv_c, material.clone(), normal),
				Triangle::with_uvs(a, c, d, uv_a, uv_c, uv_d, material, normal),
			],
		}
	}
}

impl Entity for Quad {
	fn bounding_box(&self) -> Aabb {
		Aabb::surrounding(
			self.triangles[0].bounding_box(),
			self.triangles[1].bounding_box(),
		)
	}

	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		let mut closest_hit: Option<RayHit> = None;

		for triangle in &self.triangles {
			if let Some(hit) = triangle.intersect(ray) {
				let is_closer = closest_hit.as_ref().map_or(true, |c| hit.distance < c.distance);
				if is_closer {
					closest_hit = Some(hit);
				}
			}
		}

		closest_hit
	}
}
