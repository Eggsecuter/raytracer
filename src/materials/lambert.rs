use std::fmt::{Display, Formatter, Result};
use std::io;
use std::path::Path;

use crate::materials::texture::{Albedo, ImageTexture};
use crate::primitives::{Color, UV};

use std::sync::Arc;

/// A Lambertian (diffuse) material whose albedo is either a flat colour or a
/// texture sampled at the hit point's UV coordinates.
#[derive(Debug, Clone)]
pub struct LambertMaterial {
	pub ambient: Color,
	pub albedo: Albedo,
}

impl LambertMaterial {
	/// Create a Lambert material with a flat albedo colour.
	#[allow(dead_code)]
	pub fn new(ambient: Color, albedo: Color) -> Self {
		Self {
			ambient,
			albedo: Albedo::Color(albedo),
		}
	}

	/// Convenience constructor: derive a dim ambient from the single colour.
	#[allow(dead_code)]
	pub fn from_color(color: Color) -> Self {
		Self {
			ambient: color * 0.1,
			albedo: Albedo::Color(color),
		}
	}

	/// Create a Lambert material whose albedo is driven by an image texture
	/// loaded from `path`.
	///
	/// The image is decoded once and stored behind an `Arc` so cloning the
	/// material is cheap.
	///
	/// # Errors
	/// Returns an `io::Error` if the file cannot be opened or decoded.
	#[allow(dead_code)]
	pub fn from_texture<P: AsRef<Path>>(ambient: Color, path: P) -> io::Result<Self> {
		let texture = ImageTexture::load(path)?;
		Ok(Self {
			ambient,
			albedo: Albedo::Texture(Arc::new(texture)),
		})
	}

	/// Sample the albedo at the given UV coordinate.
	///
	/// For flat-colour materials the UV is ignored; for texture materials the
	/// image is sampled with bilinear interpolation and UV tiling.
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
