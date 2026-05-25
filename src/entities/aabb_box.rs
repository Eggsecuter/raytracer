use crate::entities::{Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Aabb, Ray, RayHit, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct AabbBox {
	pub triangles: [Triangle; 12],
	/// The two corner points as supplied to [`AabbBox::new`].
	pub min: Vector3,
	pub max: Vector3,
}

impl AabbBox {
	pub fn new(material: Material, min: Vector3, max: Vector3) -> Self {
		let Vector3 {
			x: x0,
			y: y0,
			z: z0,
		} = min;
		let Vector3 {
			x: x1,
			y: y1,
			z: z1,
		} = max;

		let v000 = Vector3::new(x0, y0, z0);
		let v001 = Vector3::new(x0, y0, z1);
		let v010 = Vector3::new(x0, y1, z0);
		let v011 = Vector3::new(x0, y1, z1);
		let v100 = Vector3::new(x1, y0, z0);
		let v101 = Vector3::new(x1, y0, z1);
		let v110 = Vector3::new(x1, y1, z0);
		let v111 = Vector3::new(x1, y1, z1);

		let m = material;

		Self {
			min,
			max,
			triangles: [
				// -X
				Triangle::new(v000, v010, v011, m, None),
				Triangle::new(v000, v011, v001, m, None),
				// +X
				Triangle::new(v100, v101, v111, m, None),
				Triangle::new(v100, v111, v110, m, None),
				// -Y
				Triangle::new(v000, v001, v101, m, None),
				Triangle::new(v000, v101, v100, m, None),
				// +Y
				Triangle::new(v010, v110, v111, m, None),
				Triangle::new(v010, v111, v011, m, None),
				// -Z
				Triangle::new(v000, v100, v110, m, None),
				Triangle::new(v000, v110, v010, m, None),
				// +Z
				Triangle::new(v001, v011, v111, m, None),
				Triangle::new(v001, v111, v101, m, None),
			],
		}
	}
}

impl Entity for AabbBox {
	fn bounding_box(&self) -> Aabb {
		// Use the actual geometric min/max regardless of the order the corners were supplied.
		Aabb::new(
			Vector3::new(
				self.min.x.min(self.max.x),
				self.min.y.min(self.max.y),
				self.min.z.min(self.max.z),
			),
			Vector3::new(
				self.min.x.max(self.max.x),
				self.min.y.max(self.max.y),
				self.min.z.max(self.max.z),
			),
		)
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
