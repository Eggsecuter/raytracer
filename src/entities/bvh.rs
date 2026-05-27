use crate::entities::Entity;
use crate::primitives::{Aabb, Ray, RayHit};

pub struct BvhNode {
	left: Box<dyn Entity>,
	right: Box<dyn Entity>,
	aabb: Aabb,
}

impl BvhNode {
	pub fn build(mut entities: Vec<Box<dyn Entity>>) -> Box<dyn Entity> {
		assert!(
			!entities.is_empty(),
			"BvhNode::build: cannot build from an empty entity list"
		);

		if entities.len() == 1 {
			return entities.remove(0);
		}

		let centroid_bounds = entities.iter().fold(Aabb::empty(), |acc, e| {
			let c = e.bounding_box().centroid();
			Aabb::surrounding(acc, Aabb::new(c, c))
		});
		let axis = centroid_bounds.longest_axis();

		entities.sort_unstable_by(|a, b| {
			let ca = a.bounding_box().centroid();
			let cb = b.bounding_box().centroid();
			let va = [ca.x, ca.y, ca.z][axis];
			let vb = [cb.x, cb.y, cb.z][axis];
			va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
		});

		let right_half = entities.split_off(entities.len() / 2);
		let left = Self::build(entities);
		let right = Self::build(right_half);
		let aabb = Aabb::surrounding(left.bounding_box(), right.bounding_box());

		Box::new(BvhNode { left, right, aabb })
	}
}

impl Entity for BvhNode {
	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		if !self.aabb.hit(ray) {
			return None;
		}

		let left_hit = self.left.intersect(ray);
		let right_hit = self.right.intersect(ray);

		match (left_hit, right_hit) {
			(Some(l), Some(r)) => Some(if l.distance <= r.distance { l } else { r }),
			(Some(l), None) => Some(l),
			(None, r) => r,
		}
	}

	fn bounding_box(&self) -> Aabb {
		self.aabb
	}

	fn get_triangle_count(&self) -> i32 {
		self.left.get_triangle_count() + self.right.get_triangle_count()
	}
}
