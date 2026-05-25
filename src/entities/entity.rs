use crate::primitives::{Aabb, Ray, RayHit};

pub trait Entity: Sync + Send {
	fn intersect(&self, ray: &Ray) -> Option<RayHit>;

	/// returns an axis-aligned bounding box that fully encloses this entity.
	fn bounding_box(&self) -> Aabb;
}
