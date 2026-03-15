use crate::primitives::{Color, Ray, RayHit};

pub trait Entity: Sync + Send {
	fn color(&self) -> Color;

	fn intersect(&self, ray: &Ray) -> Option<RayHit>;
}
