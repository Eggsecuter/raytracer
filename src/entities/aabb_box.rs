use crate::entities::{Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Aabb, Ray, RayHit, Transform, Vector3, UV};

#[derive(Debug, Clone)]
pub struct AabbBox {
	pub triangles: [Triangle; 12],
	pub transform: Transform,
	pub size: Vector3,
}

impl AabbBox {
	pub fn new(material: Material, transform: Transform, size: Vector3) -> Self {
		// Size is (max - min) in local space, starting from origin
		let min = Vector3::new(0.0, 0.0, 0.0);
		let max = size;

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

		let transform_pos = transform.position;
		let transform_rot = transform.rotation;

		let t = |a: Vector3, b: Vector3, c: Vector3, uv_a, uv_b, uv_c| {
			// Apply transform to vertices
			let a_transformed = transform_rot.rotate_vector(a) + transform_pos;
			let b_transformed = transform_rot.rotate_vector(b) + transform_pos;
			let c_transformed = transform_rot.rotate_vector(c) + transform_pos;

			Triangle::with_uvs(a_transformed, b_transformed, c_transformed, uv_a, uv_b, uv_c, material.clone(), None)
		};

		Self {
			transform,
			size,
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
		// Calculate bounding box in local space (size is max - min, starting from origin)
		let local_min = Vector3::new(0.0, 0.0, 0.0);
		let local_max = self.size;

		// Transform the 8 corners of the box to world space and find bounds
		let corners = [
			Vector3::new(local_min.x, local_min.y, local_min.z),
			Vector3::new(local_max.x, local_min.y, local_min.z),
			Vector3::new(local_min.x, local_max.y, local_min.z),
			Vector3::new(local_max.x, local_max.y, local_min.z),
			Vector3::new(local_min.x, local_min.y, local_max.z),
			Vector3::new(local_max.x, local_min.y, local_max.z),
			Vector3::new(local_min.x, local_max.y, local_max.z),
			Vector3::new(local_max.x, local_max.y, local_max.z),
		];

		let mut aabb_min = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
		let mut aabb_max = Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

		for corner in corners.iter() {
			// Rotate corner by transform rotation, then translate
			let rotated = self.transform.rotation.rotate_vector(*corner);
			let world_corner = rotated + self.transform.position;

			aabb_min.x = aabb_min.x.min(world_corner.x);
			aabb_min.y = aabb_min.y.min(world_corner.y);
			aabb_min.z = aabb_min.z.min(world_corner.z);

			aabb_max.x = aabb_max.x.max(world_corner.x);
			aabb_max.y = aabb_max.y.max(world_corner.y);
			aabb_max.z = aabb_max.z.max(world_corner.z);
		}

		Aabb::new(aabb_min, aabb_max)
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

	fn get_triangle_count(&self) -> i32 {
		12
	}
}
