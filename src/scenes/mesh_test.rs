use crate::entities::Quad;
use crate::lights::OmniLight;
use crate::materials::{LambertMaterial, Material, MetalMaterial};
use crate::primitives::{Color, Quaternion, Vector3};
use crate::render::{Camera, Scene};

#[allow(dead_code)]
pub fn build(width: usize, height: usize) -> Scene {
	let camera = Camera::new(None, None, Some(width as f32 / height as f32));

	let mut scene = Scene::new(camera, width, height);
	scene.aa_samples = 4;

	let front_left_bottom = Vector3::new(-3.0, -2.0, 3.0);
	let front_right_bottom = Vector3::new(3.0, -2.0, 3.0);
	let front_left_top = Vector3::new(-3.0, 2.0, 3.0);
	let front_right_top = Vector3::new(3.0, 2.0, 3.0);

	let back_left_bottom = Vector3::new(-3.0, -2.0, 9.0);
	let back_right_bottom = Vector3::new(3.0, -2.0, 9.0);
	let back_left_top = Vector3::new(-3.0, 2.0, 9.0);
	let back_right_top = Vector3::new(3.0, 2.0, 9.0);

	let floor = Quad::new(
		front_right_bottom,
		front_left_bottom,
		back_left_bottom,
		back_right_bottom,
		Material::Lambert(LambertMaterial::from_color(Color::WHITE)),
		None,
	);
	scene.entities.push(Box::new(floor));
	let ceiling = Quad::new(
		front_left_top,
		front_right_top,
		back_right_top,
		back_left_top,
		Material::Lambert(LambertMaterial::from_color(Color::WHITE)),
		None,
	);
	scene.entities.push(Box::new(ceiling));
	let back_wall = Quad::new(
		back_left_bottom,
		back_left_top,
		back_right_top,
		back_right_bottom,
		Material::Lambert(LambertMaterial::from_color(Color::WHITE)),
		None,
	);
	scene.entities.push(Box::new(back_wall));
	let front_wall = Quad::new(
		front_left_bottom,
		front_right_bottom,
		front_right_top,
		front_left_top,
		Material::Lambert(LambertMaterial::from_color(Color::WHITE)),
		None,
	);
	scene.entities.push(Box::new(front_wall));
	let left_wall = Quad::new(
		front_left_bottom,
		front_left_top,
		back_left_top,
		back_left_bottom,
		Material::Lambert(LambertMaterial::from_color(Color::RED)),
		None,
	);
	scene.entities.push(Box::new(left_wall));
	let right_wall = Quad::new(
		front_right_bottom,
		back_right_bottom,
		back_right_top,
		front_right_top,
		Material::Lambert(LambertMaterial::from_color(Color::GREEN)),
		None,
	);
	scene.entities.push(Box::new(right_wall));

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
			Vector3::new(2.0, -1.2, 7.0),
			0.5,
			Quaternion::from_euler(Vector3::new(0.0, 0.5, 0.0)),
		)
		.expect("failed to load penguin mesh");

	scene
		.add_obj_mesh(
			"models/moai/source/moai.obj",
			Material::Metal(MetalMaterial::GOLD),
			Vector3::new(-1.0, -2.0, 5.0),
			0.08,
			Quaternion::from_euler(Vector3::new(0.0, -0.5, 0.0)),
		)
		.expect("failed to load penguin mesh");

	let ceiling_light =
		OmniLight::new(Color::new(0.9, 0.9, 0.9), Vector3::new(1.0, 1.85, 3.5));
	scene.global_lights.push(Box::new(ceiling_light));

	scene
}
