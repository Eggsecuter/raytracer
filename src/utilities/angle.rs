pub struct Angle;

impl Angle {
	pub const FULL_RADIANT: f64 = std::f64::consts::PI * 2.0;
	pub const FULL_DEGREE: f64 = 360.0;

	/// Convert radians to degrees
	pub fn to_degree(radian: f64) -> f64 {
		radian / Self::FULL_RADIANT * Self::FULL_DEGREE
	}

	/// Convert degrees to radians
	pub fn to_radian(degree: f64) -> f64 {
		degree / Self::FULL_DEGREE * Self::FULL_RADIANT
	}
}
