use std::fmt::{Display, Formatter, Result};

use crate::materials::{DielectricMaterial, LambertMaterial, MetalMaterial};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Material {
	Lambert(LambertMaterial),
	Metal(MetalMaterial),
	Dielectric(DielectricMaterial),
}

impl Display for Material {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Material::Lambert(material) => write!(f, "{material}"),
			Material::Metal(material) => write!(f, "{material}"),
			Material::Dielectric(material) => write!(f, "{material}"),
		}
	}
}
