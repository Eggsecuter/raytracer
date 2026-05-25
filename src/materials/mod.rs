pub mod material;
pub use material::Material;

pub mod lambert;
pub use lambert::LambertMaterial;

pub mod metal;
pub use metal::MetalMaterial;

pub mod dielectric;
pub use dielectric::DielectricMaterial;

pub mod texture;
#[allow(unused_imports)]
pub use texture::{Albedo, ImageTexture};
