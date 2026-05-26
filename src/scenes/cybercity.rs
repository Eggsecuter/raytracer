use crate::entities::{Quad, Sphere};
use crate::lights::PointLight;
use crate::materials::{LambertMaterial, Material};
use crate::primitives::{Color, Transform, Vector3};
use crate::render::{Camera, Scene};

#[allow(dead_code)]
pub fn build() -> Scene {
	let width = 2378 / 4;
	let height = 1682 / 4;

	let camera = Camera::new(None, None, Some(width as f32 / height as f32));

	let mut scene = Scene::new(camera, width, height);

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

	let moon = Sphere::new(
		Material::Lambert(LambertMaterial::from_texture(
				Color::new(0.05, 0.05, 0.05),
				"models/moon/lroc_color_poles_1k.jpg",
			)
			.expect("failed to load moon texture")
		),
		Transform::new(Some(Vector3::new(0.0, 0.0, 3.5)), None),
		1.0,
	);
	scene.entities.push(Box::new(moon));

	let light =
		PointLight::new(Color::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 1.0), 1.0);
	scene.global_lights.push(Box::new(light));

	let light2 =
		PointLight::new(Color::new(1.0, 1.0, 1.0), Vector3::new(-1.0, 0.0, 1.0), 10.0);
	scene.global_lights.push(Box::new(light2));

	scene
}
