use crate::primitives::{Ray, RayHit};

pub trait Entity: Sync + Send {
	fn intersect(&self, ray: &Ray) -> Option<RayHit>;
}
