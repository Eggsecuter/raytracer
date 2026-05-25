use std::io;
use std::path::Path;
use std::sync::Arc;

use crate::primitives::{Color, UV};

// ---------------------------------------------------------------------------
// ImageTexture
// ---------------------------------------------------------------------------

/// A decoded image stored as linear-light RGB pixels, ready to be sampled.
///
/// Pixels are stored row-major with the origin at the top-left corner, which
/// matches the `UV` convention used throughout this renderer (V = 0 at top).
pub struct ImageTexture {
	width: u32,
	height: u32,
	/// Linear-light RGB pixels, row-major.
	pixels: Vec<Color>,
}

impl ImageTexture {
	/// Load an image from disk and decode it into linear-light RGB.
	///
	/// sRGB-encoded images (PNG, JPEG, …) are converted to linear light so
	/// that lighting calculations remain physically correct.
	pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		let img = image::ImageReader::open(path)
			.map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?
			.decode()
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
			.into_rgb8();

		let width = img.width();
		let height = img.height();

		let pixels = img
			.pixels()
			.map(|p| {
				// Convert each channel from sRGB (gamma-encoded, u8) to linear light (f32).
				Color::new(
					srgb_to_linear(p[0]),
					srgb_to_linear(p[1]),
					srgb_to_linear(p[2]),
				)
			})
			.collect();

		Ok(Self {
			width,
			height,
			pixels,
		})
	}

	/// Sample the texture at `uv` using **bilinear interpolation**.
	///
	/// UV coordinates outside `[0, 1]` are wrapped (tiled).
	pub fn sample(&self, uv: UV) -> Color {
		let w = self.width as f32;
		let h = self.height as f32;

		// Wrap UV into [0, 1) — fract() is always non-negative for positive values,
		// so handle negatives explicitly.
		let u = uv.u.fract().abs();
		let v = uv.v.fract().abs();

		// Map to pixel space; clamp to avoid 1-pixel overshoot at the far edge.
		let px = (u * w - 0.5).max(0.0);
		let py = (v * h - 0.5).max(0.0);

		let x0 = px.floor() as usize;
		let y0 = py.floor() as usize;
		let x1 = (x0 + 1).min(self.width as usize - 1);
		let y1 = (y0 + 1).min(self.height as usize - 1);

		let tx = px.fract();
		let ty = py.fract();

		let stride = self.width as usize;
		let c00 = self.pixels[y0 * stride + x0];
		let c10 = self.pixels[y0 * stride + x1];
		let c01 = self.pixels[y1 * stride + x0];
		let c11 = self.pixels[y1 * stride + x1];

		// Bilinear blend: horizontal, then vertical.
		let top = c00 * (1.0 - tx) + c10 * tx;
		let bot = c01 * (1.0 - tx) + c11 * tx;
		top * (1.0 - ty) + bot * ty
	}
}

// ---------------------------------------------------------------------------
// Albedo
// ---------------------------------------------------------------------------

/// The source of albedo colour for a [`LambertMaterial`](super::LambertMaterial).
///
/// - `Color` — a uniform flat colour (zero runtime cost).
/// - `Texture` — a reference-counted [`ImageTexture`] sampled at the hit's UV
///   coordinates.  Cloning is cheap (`Arc` clone) so the texture data is
///   shared rather than duplicated.
#[derive(Debug, Clone)]
pub enum Albedo {
	Color(Color),
	Texture(Arc<ImageTexture>),
}

impl Albedo {
	/// Return the albedo colour at the given UV coordinate.
	///
	/// For `Color` variants the UV is ignored.
	/// For `Texture` variants the image is sampled with bilinear interpolation.
	pub fn sample(&self, uv: UV) -> Color {
		match self {
			Albedo::Color(c) => *c,
			Albedo::Texture(t) => t.sample(uv),
		}
	}
}

// ---------------------------------------------------------------------------
// Debug for ImageTexture (Arc requires it)
// ---------------------------------------------------------------------------

impl std::fmt::Debug for ImageTexture {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ImageTexture")
			.field("width", &self.width)
			.field("height", &self.height)
			.finish_non_exhaustive()
	}
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a single sRGB-encoded byte to a linear-light `f32` value in `[0, 1]`.
fn srgb_to_linear(byte: u8) -> f32 {
	let s = byte as f32 / 255.0;
	if s <= 0.04045 {
		s / 12.92
	} else {
		((s + 0.055) / 1.055).powf(2.4)
	}
}
