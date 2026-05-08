use crate::entities::{Quad, Sphere};
use crate::lights::OmniLight;
use crate::materials::Material;
use crate::primitives::{Color, Quaternion, Transform, Vector3};
use crate::render::{Camera, Scene};

pub fn build(width: usize, height: usize) -> Scene {
	let camera = Camera::new(
		Some(Transform::new(
			Some(Vector3::new(1.0, 0.0, 3.1)),
			Some(Quaternion::from_euler(Vector3::new(0.0, -0.5, 0.0)))
		)),
		None,
		Some(width as f32 / height as f32)
	);

	let mut scene = Scene::new(camera, width, height);

	let front_left_bottom = Vector3::new(-3.0, -2.0, 3.0);
	let front_right_bottom = Vector3::new(3.0, -2.0, 3.0);
	let front_left_top = Vector3::new(-3.0, 2.0, 3.0);
	let front_right_top = Vector3::new(3.0, 2.0, 3.0);

	let back_left_bottom = Vector3::new(-3.0, -2.0, 9.0);
	let back_right_bottom = Vector3::new(3.0, -2.0, 9.0);
	let back_left_top = Vector3::new(-3.0, 2.0, 9.0);
	let back_right_top = Vector3::new(3.0, 2.0, 9.0);

	let floor = Quad::new(Material::from_color(Color::WHITE), front_right_bottom, front_left_bottom, back_left_bottom, back_right_bottom);
	scene.entities.push(Box::new(floor));
	let ceiling = Quad::new(Material::from_color(Color::WHITE), front_left_top, front_right_top, back_right_top, back_left_top);
	scene.entities.push(Box::new(ceiling));
	let back_wall = Quad::new(Material::from_color(Color::WHITE), back_left_bottom, back_left_top, back_right_top, back_right_bottom);
	scene.entities.push(Box::new(back_wall));
	let front_wall = Quad::new(Material::from_color(Color::WHITE), front_left_bottom, front_right_bottom, front_right_top, front_left_top);
	scene.entities.push(Box::new(front_wall));
	let left_wall = Quad::new(Material::from_color(Color::RED), front_left_bottom, front_left_top, back_left_top, back_left_bottom);
	scene.entities.push(Box::new(left_wall));
	let right_wall = Quad::new(Material::from_color(Color::GREEN), front_right_bottom, back_right_bottom, back_right_top, front_right_top);
	scene.entities.push(Box::new(right_wall));

	let first_ball = Sphere::new(
		Material::BRASS,
		Transform::new(Some(Vector3::new(-0.85, -1.15, 6.0)), None),
		0.85
	);
	scene.entities.push(Box::new(first_ball));
	let second_ball = Sphere::new(
		Material::CHROME,
		Transform::new(Some(Vector3::new(1.05, -1.35, 7.2)), None),
		0.65
	);
	scene.entities.push(Box::new(second_ball));

	let ceiling_light = OmniLight::new(Color::new(0.45, 0.43, 0.38), Vector3::new(0.0, 1.85, 5.8));
	scene.global_lights.push(Box::new(ceiling_light));

	scene
}
