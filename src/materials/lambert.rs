use std::fmt::{Display, Formatter, Result};
use std::io;
use std::path::Path;

use crate::materials::texture::{Albedo, ImageTexture};
use crate::primitives::{Color, UV};

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LambertMaterial {
	pub ambient: Color,
	pub albedo: Albedo,
}

impl LambertMaterial {
	#[allow(dead_code)]
	pub fn new(ambient: Color, albedo: Color) -> Self {
		Self {
			ambient,
			albedo: Albedo::Color(albedo),
		}
	}

	#[allow(dead_code)]
	pub fn from_color(color: Color) -> Self {
		Self {
			ambient: color * 0.1,
			albedo: Albedo::Color(color),
		}
	}

	#[allow(dead_code)]
	pub fn from_texture<P: AsRef<Path>>(ambient: Color, path: P) -> io::Result<Self> {
		let texture = ImageTexture::load(path)?;
		Ok(Self {
			ambient,
			albedo: Albedo::Texture(Arc::new(texture)),
		})
	}

	pub fn albedo_at(&self, uv: UV) -> Color {
		self.albedo.sample(uv)
	}
}

impl Display for LambertMaterial {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match &self.albedo {
			Albedo::Color(c) => write!(f, "LambertMaterial[ambient={}, albedo={}]", self.ambient, c),
			Albedo::Texture(_) => write!(f, "LambertMaterial[ambient={}, albedo=<texture>]", self.ambient),
		}
	}
}
