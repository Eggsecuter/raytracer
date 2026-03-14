use crate::primitives::{Color, RayHit};

pub trait Light: Sync + Send {
	fn calculate_color(&self, rayHit: &RayHit) -> Color;
}
