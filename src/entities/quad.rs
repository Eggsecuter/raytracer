use crate::entities::{Entity, Triangle};
use crate::primitives::{Color, Ray, RayHit, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Quad {
	pub color: Color,
	pub triangles: [Triangle; 2],
}

impl Quad {
	pub fn new(color: Color, a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> Self {
		Self {
			color,
			triangles: [Triangle::new(color, a, b, c), Triangle::new(color, a, c, d)],
		}
	}
}

impl Entity for Quad {
	fn color(&self) -> Color {
		self.color
	}

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
