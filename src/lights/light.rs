use crate::primitives::{Color, RayHit, Vector3};

pub trait Light: Sync + Send {
	fn position(&self) -> Vector3;
	fn calculate_color(&self, ray_hit: &RayHit) -> Color;
}
