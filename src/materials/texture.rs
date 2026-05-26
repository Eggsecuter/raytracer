use std::io;
use std::path::Path;
use std::sync::Arc;

use crate::primitives::{Color, UV};

// ---------------------------------------------------------------------------
// ImageTexture
// ---------------------------------------------------------------------------
pub struct ImageTexture {
	width: u32,
	height: u32,
	pixels: Vec<Color>,
}

impl ImageTexture {
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
				Color::new(
					p[0] as f32 / 255.0,
					p[1] as f32 / 255.0,
					p[2] as f32 / 255.0,
				).srgb_to_linear()
			})
			.collect();

		Ok(Self {
			width,
			height,
			pixels,
		})
	}

	pub fn sample(&self, uv: UV) -> Color {
		let w = self.width as f32;
		let h = self.height as f32;

		let u = uv.u.fract().abs();
		let v = uv.v.fract().abs();

		// map to pixel space; clamp to avoid 1-pixel overshoot at the far edge.
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

		// bilinear blend: horizontal, then vertical.
		let top = c00 * (1.0 - tx) + c10 * tx;
		let bot = c01 * (1.0 - tx) + c11 * tx;
		top * (1.0 - ty) + bot * ty
	}
}

// ---------------------------------------------------------------------------
// Albedo
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum Albedo {
	Color(Color),
	Texture(Arc<ImageTexture>),
}

impl Albedo {
	pub fn sample(&self, uv: UV) -> Color {
		match self {
			Albedo::Color(c) => *c,
			Albedo::Texture(t) => t.sample(uv),
		}
	}
}

impl std::fmt::Debug for ImageTexture {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ImageTexture")
			.field("width", &self.width)
			.field("height", &self.height)
			.finish_non_exhaustive()
	}
}
