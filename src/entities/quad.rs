use crate::entities::{Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Ray, RayHit, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Quad {
	pub triangles: [Triangle; 2],
}

impl Quad {
	pub fn new(
		a: Vector3,
		b: Vector3,
		c: Vector3,
		d: Vector3,
		material: Material,
		normal: Option<Vector3>,
	) -> Self {
		Self {
			triangles: [
				Triangle::new(a, b, c, material, normal),
				Triangle::new(a, c, d, material, normal),
			],
		}
	}
}

impl Entity for Quad {
	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		let mut closest_hit = None;

		for triangle in self.triangles {
			if let Some(hit) = triangle.intersect(ray)
				&& closest_hit.is_none_or(|closest: RayHit| hit.distance < closest.distance)
			{
				closest_hit = Some(hit);
			}
		}

		closest_hit
	}
}
