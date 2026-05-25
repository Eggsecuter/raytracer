use crate::lights::OmniLight;
use crate::materials::{LambertMaterial, Material};
use crate::primitives::{Color, Quaternion, Vector3};
use crate::render::{Camera, Scene};

#[allow(dead_code)]
pub fn build(width: usize, height: usize) -> Scene {
	let camera = Camera::new(None, None, Some(width as f32 / height as f32));

	let mut scene = Scene::new(camera, width, height);
	scene.aa_samples = 4;

	scene
		.add_obj_mesh(
			"models/penguin/source/lowpolyPinguin.obj",
			Material::Lambert(
				LambertMaterial::from_texture(
					Color::new(0.05, 0.05, 0.05),
					"models/penguin/textures/ColorTexturePinguin.png",
				)
				.expect("failed to load penguin texture"),
			),
			Vector3::new(0.0, 0.0, 5.0),
			0.5,
			Quaternion::from_euler(Vector3::new(0.0, 0.0, 0.0)),
		)
		.expect("failed to load penguin mesh");

	let ceiling_light =
		OmniLight::new(Color::new(0.9, 0.9, 0.9), Vector3::new(1.0, 1.85, 3.5));
	scene.global_lights.push(Box::new(ceiling_light));

	scene
}
