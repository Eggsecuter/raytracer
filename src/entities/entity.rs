use crate::primitives::{Color, Ray, RayHit, Transform};

pub trait Entity: Sync + Send {
	fn color(&self) -> Color;
	fn transform(&self) -> &Transform;

	fn intersect(&self, ray: &Ray) -> Option<RayHit>;
}
