use std::fmt::{Display, Formatter, Result};

use crate::primitives::{Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
	pub ambient_color: Color,
	pub diffuse_color: Color,
	pub specular_color: Color,
	pub smoothness: f32
}

#[allow(dead_code)]
impl Material {
	pub const MIRROR: Material = Material {
		ambient_color: Color { red: 0.1, green: 0.1, blue: 0.1 },
		diffuse_color: Color { red: 0.8, green: 0.8, blue: 0.8 },
		specular_color: Color { red: 0.6, green: 0.6, blue: 0.6 },
		smoothness: 1.0,
	};

	pub const EMERALD: Material = Material {
		ambient_color: Color { red: 0.0215, green: 0.1745, blue: 0.0215 },
		diffuse_color: Color { red: 0.07568, green: 0.61424, blue: 0.07568 },
		specular_color: Color { red: 0.633, green: 0.727811, blue: 0.633 },
		smoothness: 0.6,
	};

	pub const JADE: Material = Material {
		ambient_color: Color { red: 0.135, green: 0.2225, blue: 0.1575 },
		diffuse_color: Color { red: 0.54, green: 0.89, blue: 0.63 },
		specular_color: Color { red: 0.316228, green: 0.316228, blue: 0.316228 },
		smoothness: 0.1,
	};

	pub const OBSIDIAN: Material = Material {
		ambient_color: Color { red: 0.05375, green: 0.05, blue: 0.06625 },
		diffuse_color: Color { red: 0.18275, green: 0.17, blue: 0.22525 },
		specular_color: Color { red: 0.332741, green: 0.328634, blue: 0.346435 },
		smoothness: 0.3,
	};

	pub const PEARL: Material = Material {
		ambient_color: Color { red: 0.25, green: 0.20725, blue: 0.20725 },
		diffuse_color: Color { red: 1.0, green: 0.829, blue: 0.829 },
		specular_color: Color { red: 0.296648, green: 0.296648, blue: 0.296648 },
		smoothness: 0.088,
	};

	pub const RUBY: Material = Material {
		ambient_color: Color { red: 0.1745, green: 0.01175, blue: 0.01175 },
		diffuse_color: Color { red: 0.61424, green: 0.04136, blue: 0.04136 },
		specular_color: Color { red: 0.727811, green: 0.626959, blue: 0.626959 },
		smoothness: 0.6,
	};

	pub const TURQUOISE: Material = Material {
		ambient_color: Color { red: 0.1, green: 0.18725, blue: 0.1745 },
		diffuse_color: Color { red: 0.396, green: 0.74151, blue: 0.69102 },
		specular_color: Color { red: 0.297254, green: 0.30829, blue: 0.306678 },
		smoothness: 0.1,
	};

	pub const BRASS: Material = Material {
		ambient_color: Color { red: 0.329412, green: 0.223529, blue: 0.027451 },
		diffuse_color: Color { red: 0.780392, green: 0.568627, blue: 0.113725 },
		specular_color: Color { red: 0.992157, green: 0.941176, blue: 0.807843 },
		smoothness: 0.21794872,
	};

	pub const BRONZE: Material = Material {
		ambient_color: Color { red: 0.2125, green: 0.1275, blue: 0.054 },
		diffuse_color: Color { red: 0.714, green: 0.4284, blue: 0.18144 },
		specular_color: Color { red: 0.393548, green: 0.271906, blue: 0.166721 },
		smoothness: 0.2,
	};

	pub const CHROME: Material = Material {
		ambient_color: Color { red: 0.25, green: 0.25, blue: 0.25 },
		diffuse_color: Color { red: 0.4, green: 0.4, blue: 0.4 },
		specular_color: Color { red: 0.774597, green: 0.774597, blue: 0.774597 },
		smoothness: 0.6,
	};

	pub const COPPER: Material = Material {
		ambient_color: Color { red: 0.19125, green: 0.0735, blue: 0.0225 },
		diffuse_color: Color { red: 0.7038, green: 0.27048, blue: 0.0828 },
		specular_color: Color { red: 0.256777, green: 0.137622, blue: 0.086014 },
		smoothness: 0.1,
	};

	pub const GOLD: Material = Material {
		ambient_color: Color { red: 0.24725, green: 0.1995, blue: 0.0745 },
		diffuse_color: Color { red: 0.75164, green: 0.60648, blue: 0.22648 },
		specular_color: Color { red: 0.628281, green: 0.555802, blue: 0.366065 },
		smoothness: 0.4,
	};

	pub const SILVER: Material = Material {
		ambient_color: Color { red: 0.19225, green: 0.19225, blue: 0.19225 },
		diffuse_color: Color { red: 0.50754, green: 0.50754, blue: 0.50754 },
		specular_color: Color { red: 0.508273, green: 0.508273, blue: 0.508273 },
		smoothness: 0.4,
	};

	pub const BLACK_PLASTIC: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.0, blue: 0.0 },
		diffuse_color: Color { red: 0.01, green: 0.01, blue: 0.01 },
		specular_color: Color { red: 0.50, green: 0.50, blue: 0.50 },
		smoothness: 0.25,
	};

	pub const CYAN_PLASTIC: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.1, blue: 0.06 },
		diffuse_color: Color { red: 0.0, green: 0.50980392, blue: 0.50980392 },
		specular_color: Color { red: 0.50196078, green: 0.50196078, blue: 0.50196078 },
		smoothness: 0.25,
	};

	pub const GREEN_PLASTIC: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.0, blue: 0.0 },
		diffuse_color: Color { red: 0.1, green: 0.35, blue: 0.1 },
		specular_color: Color { red: 0.45, green: 0.55, blue: 0.45 },
		smoothness: 0.25,
	};

	pub const RED_PLASTIC: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.0, blue: 0.0 },
		diffuse_color: Color { red: 0.5, green: 0.0, blue: 0.0 },
		specular_color: Color { red: 0.7, green: 0.6, blue: 0.6 },
		smoothness: 0.25,
	};

	pub const WHITE_PLASTIC: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.0, blue: 0.0 },
		diffuse_color: Color { red: 0.55, green: 0.55, blue: 0.55 },
		specular_color: Color { red: 0.70, green: 0.70, blue: 0.70 },
		smoothness: 0.25,
	};

	pub const YELLOW_PLASTIC: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.0, blue: 0.0 },
		diffuse_color: Color { red: 0.5, green: 0.5, blue: 0.0 },
		specular_color: Color { red: 0.60, green: 0.60, blue: 0.50 },
		smoothness: 0.25,
	};

	pub const BLACK_RUBBER: Material = Material {
		ambient_color: Color { red: 0.02, green: 0.02, blue: 0.02 },
		diffuse_color: Color { red: 0.01, green: 0.01, blue: 0.01 },
		specular_color: Color { red: 0.4, green: 0.4, blue: 0.4 },
		smoothness: 0.078125,
	};

	pub const CYAN_RUBBER: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.05, blue: 0.05 },
		diffuse_color: Color { red: 0.4, green: 0.5, blue: 0.5 },
		specular_color: Color { red: 0.04, green: 0.7, blue: 0.7 },
		smoothness: 0.078125,
	};

	pub const GREEN_RUBBER: Material = Material {
		ambient_color: Color { red: 0.0, green: 0.05, blue: 0.0 },
		diffuse_color: Color { red: 0.4, green: 0.5, blue: 0.4 },
		specular_color: Color { red: 0.04, green: 0.7, blue: 0.04 },
		smoothness: 0.078125,
	};

	pub const RED_RUBBER: Material = Material {
		ambient_color: Color { red: 0.05, green: 0.0, blue: 0.0 },
		diffuse_color: Color { red: 0.5, green: 0.4, blue: 0.4 },
		specular_color: Color { red: 0.7, green: 0.04, blue: 0.04 },
		smoothness: 0.078125,
	};

	pub const WHITE_RUBBER: Material = Material {
		ambient_color: Color { red: 0.05, green: 0.05, blue: 0.05 },
		diffuse_color: Color { red: 0.5, green: 0.5, blue: 0.5 },
		specular_color: Color { red: 0.7, green: 0.7, blue: 0.7 },
		smoothness: 0.078125,
	};

	pub const YELLOW_RUBBER: Material = Material {
		ambient_color: Color { red: 0.05, green: 0.05, blue: 0.0 },
		diffuse_color: Color { red: 0.5, green: 0.5, blue: 0.4 },
		specular_color: Color { red: 0.7, green: 0.04, blue: 0.04 },
		smoothness: 0.078125,
	};
}

impl Material {
	#[allow(dead_code)]
	pub fn new(ambient_color: Color, diffuse_color: Color, specular_color: Color, smoothness: f32) -> Self {
		Self { ambient_color, diffuse_color, specular_color, smoothness }
	}

	#[allow(dead_code)]
	pub fn from_color(color: Color) -> Self {
		Self {
			ambient_color: color  * 0.1,
			diffuse_color: color * 0.8,
			specular_color: color * 0.8,
			smoothness: 0.0
		}
	}
}

impl Display for Material {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"Material[{}, {}, {}, {}]",
			self.ambient_color, self.diffuse_color, self.specular_color, self.smoothness
		)
	}
}
