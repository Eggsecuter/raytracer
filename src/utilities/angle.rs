pub struct Angle;

impl Angle {
	pub const FULL_RADIANT: f32 = std::f32::consts::PI * 2.0;
	pub const FULL_DEGREE: f32 = 360.0;

	/// Convert radians to degrees
	pub fn to_degree(radian: f32) -> f32 {
		radian / Self::FULL_RADIANT * Self::FULL_DEGREE
	}

	/// Convert degrees to radians
	pub fn to_radian(degree: f32) -> f32 {
		degree / Self::FULL_DEGREE * Self::FULL_RADIANT
	}
}
