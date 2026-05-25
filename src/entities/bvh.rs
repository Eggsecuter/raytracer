use crate::entities::Entity;
use crate::primitives::{Aabb, Ray, RayHit};

/// A node in a Bounding Volume Hierarchy (BVH) tree.
///
/// Each node caches its own AABB and two child sub-trees.  `intersect` first
/// tests the ray against the node's AABB; only on a hit does it recurse into
/// the children, returning the closer of the two results.
pub struct BvhNode {
	left: Box<dyn Entity>,
	right: Box<dyn Entity>,
	aabb: Aabb,
}

impl BvhNode {
	/// Recursively build a BVH tree from the given list of entities and return
	/// its root as a `Box<dyn Entity>`.
	///
	/// The algorithm:
	/// 1. Compute the bounding box of all entity centroids.
	/// 2. Sort by the longest axis of that centroid box.
	/// 3. Split at the median and recurse on each half.
	///
	/// A list of exactly one entity is returned unwrapped (no wrapper node).
	pub fn build(mut entities: Vec<Box<dyn Entity>>) -> Box<dyn Entity> {
		assert!(
			!entities.is_empty(),
			"BvhNode::build: cannot build from an empty entity list"
		);

		if entities.len() == 1 {
			return entities.remove(0);
		}

		// Determine the best split axis from the spread of centroids.
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
	/// Test the ray against the AABB first; skip both children on a miss.
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
}
