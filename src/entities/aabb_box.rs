use crate::entities::{Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Ray, RayHit, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct AabbBox {
	pub triangles: [Triangle; 12],
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
