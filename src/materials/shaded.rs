use std::fmt::{Display, Formatter, Result};
use std::io;
use std::path::Path;

use crate::materials::texture::{Albedo, ImageTexture};
use crate::primitives::{Color, UV};

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ShadedMaterial {
	pub ambient: Color,
	pub albedo: Albedo,

	pub shininess: f32,
	pub kd: f32, // diffuse coefficient
	pub ks: f32, // specular coefficient
	pub ka: f32, // ambient coefficient
}

impl ShadedMaterial {
	#[allow(dead_code)]
	pub fn new(ambient: Color, albedo: Color, shininess: f32, kd: f32, ks: f32, ka: f32) -> Self {
		Self {
			ambient,
			albedo: Albedo::Color(albedo),
			shininess,
			kd,
			ks,
			ka,
		}
	}

	#[allow(dead_code)]
	pub fn from_color(color: Color) -> Self {
		Self {
			ambient: color * 0.1,
			albedo: Albedo::Color(color),
			shininess: 32.0,
			kd: 0.8,
			ks: 0.2,
			ka: 0.05,
		}
	}

	#[allow(dead_code)]
	pub fn from_texture<P: AsRef<Path>>(ambient: Color, path: P) -> io::Result<Self> {
		let texture = ImageTexture::load(path)?;
		Ok(Self {
			ambient,
			albedo: Albedo::Texture(Arc::new(texture)),
			shininess: 128.0,
			kd: 0.8,
			ks: 0.2,
			ka: 0.05,
		})
	}

	pub fn albedo_at(&self, uv: UV) -> Color {
		self.albedo.sample(uv)
	}
}

impl Display for ShadedMaterial {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match &self.albedo {
			Albedo::Color(c) => write!(f, "LambertMaterial[ambient={}, albedo={}]", self.ambient, c),
			Albedo::Texture(_) => write!(f, "LambertMaterial[ambient={}, albedo=<texture>]", self.ambient),
		}
	}
}
