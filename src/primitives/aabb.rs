use crate::primitives::{Ray, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
	pub min: Vector3,
	pub max: Vector3,
}

impl Aabb {
	pub fn new(min: Vector3, max: Vector3) -> Self {
		Self { min, max }
	}

	pub fn empty() -> Self {
		Self {
			min: Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
			max: Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
		}
	}

	pub fn surrounding(a: Aabb, b: Aabb) -> Self {
		Self {
			min: Vector3::new(
				a.min.x.min(b.min.x),
				a.min.y.min(b.min.y),
				a.min.z.min(b.min.z),
			),
			max: Vector3::new(
				a.max.x.max(b.max.x),
				a.max.y.max(b.max.y),
				a.max.z.max(b.max.z),
			),
		}
	}

	pub fn hit(&self, ray: &Ray) -> bool {
		let inv_dx = 1.0 / ray.direction.x;
		let inv_dy = 1.0 / ray.direction.y;
		let inv_dz = 1.0 / ray.direction.z;

		let tx0 = (self.min.x - ray.origin.x) * inv_dx;
		let tx1 = (self.max.x - ray.origin.x) * inv_dx;
		let ty0 = (self.min.y - ray.origin.y) * inv_dy;
		let ty1 = (self.max.y - ray.origin.y) * inv_dy;
		let tz0 = (self.min.z - ray.origin.z) * inv_dz;
		let tz1 = (self.max.z - ray.origin.z) * inv_dz;

		let t_enter = tx0.min(tx1).max(ty0.min(ty1)).max(tz0.min(tz1));
		let t_exit = tx0.max(tx1).min(ty0.max(ty1)).min(tz0.max(tz1));

		t_exit > t_enter.max(0.0)
	}

	pub fn centroid(&self) -> Vector3 {
		Vector3::new(
			(self.min.x + self.max.x) * 0.5,
			(self.min.y + self.max.y) * 0.5,
			(self.min.z + self.max.z) * 0.5,
		)
	}

	pub fn longest_axis(&self) -> usize {
		let ex = self.max.x - self.min.x;
		let ey = self.max.y - self.min.y;
		let ez = self.max.z - self.min.z;
		if ex >= ey && ex >= ez {
			0
		} else if ey >= ez {
			1
		} else {
			2
		}
	}
}
