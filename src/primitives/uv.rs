use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UV {
	pub u: f32,
	pub v: f32,
}

impl UV {
	pub const ZERO: UV = UV { u: 0.0, v: 0.0 };

	pub fn new(u: f32, v: f32) -> Self {
		Self { u, v }
	}

	pub fn barycentric(a: UV, b: UV, c: UV, bu: f32, bv: f32) -> UV {
		let w = 1.0 - bu - bv;
		UV::new(
			w * a.u + bu * b.u + bv * c.u,
			w * a.v + bu * b.v + bv * c.v,
		)
	}
}

impl Add for UV {
	type Output = UV;
	fn add(self, other: UV) -> UV {
		UV::new(self.u + other.u, self.v + other.v)
	}
}

impl Mul<f32> for UV {
	type Output = UV;
	fn mul(self, s: f32) -> UV {
		UV::new(self.u * s, self.v * s)
	}
}

impl Display for UV {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "UV[{}, {}]", self.u, self.v)
	}
}
