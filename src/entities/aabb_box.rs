use crate::entities::{Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Aabb, Ray, RayHit, Vector3, UV};

#[derive(Debug, Clone)]
pub struct AabbBox {
	pub triangles: [Triangle; 12],
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

		let (u00, u10, u11, u01) = (
			UV::new(0.0, 0.0),
			UV::new(1.0, 0.0),
			UV::new(1.0, 1.0),
			UV::new(0.0, 1.0),
		);

		let t = |a, b, c, uv_a, uv_b, uv_c| {
			Triangle::with_uvs(a, b, c, uv_a, uv_b, uv_c, material.clone(), None)
		};

		Self {
			min,
			max,
			triangles: [
				t(v000, v010, v011, u00, u01, u11),
				t(v000, v011, v001, u00, u11, u10),
				t(v100, v101, v111, u00, u10, u11),
				t(v100, v111, v110, u00, u11, u01),
				t(v000, v001, v101, u00, u01, u11),
				t(v000, v101, v100, u00, u11, u10),
				t(v010, v110, v111, u00, u10, u11),
				t(v010, v111, v011, u00, u11, u01),
				t(v000, v100, v110, u00, u10, u11),
				t(v000, v110, v010, u00, u11, u01),
				t(v001, v011, v111, u00, u01, u11),
				t(v001, v111, v101, u00, u11, u10),
			],
		}
	}
}

impl Entity for AabbBox {
	fn bounding_box(&self) -> Aabb {
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
