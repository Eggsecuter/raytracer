use crate::entities::{AabbBox, Quad, Sphere};
use crate::lights::PointLight;
use crate::materials::{DielectricMaterial, LambertMaterial, Material, MetalMaterial};
use crate::primitives::{Color, Transform, Vector3};
use crate::render::{Camera, Scene};

#[allow(dead_code)]
pub fn build() -> Scene {
	let width = 1500;
	let height = 1000;

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

	let first_ball = Sphere::new(
		Material::Dielectric(DielectricMaterial::SAPPHIRE),
		Transform::new(Some(Vector3::new(-0.5, -1.0, 4.5)), None),
		1.0,
	);
	scene.entities.push(Box::new(first_ball));
	let second_ball = Sphere::new(
		Material::Metal(MetalMaterial::SILVER),
		Transform::new(Some(Vector3::new(1.05, -1.35, 7.2)), None),
		0.65,
	);
	scene.entities.push(Box::new(second_ball));

	let aabb_box = AabbBox::new(
		Material::Dielectric(DielectricMaterial::GLASS),
		Transform::new(Some(Vector3::new(1.0, 1.0, 5.0)), None),
		Vector3::new(1.0, -3.0, 1.5),
	);
	scene.entities.push(Box::new(aabb_box));

	let ceiling_light = PointLight::new(Color::new(0.9, 0.9, 0.9), Vector3::new(0.0, 1.85, 5.8), 3.0);
	scene.global_lights.push(Box::new(ceiling_light));
	let second_ceiling_light =
		PointLight::new(Color::new(0.9, 0.9, 0.9), Vector3::new(-1.0, 1.85, 8.0), 1.0);
	scene.global_lights.push(Box::new(second_ceiling_light));

	scene
}
