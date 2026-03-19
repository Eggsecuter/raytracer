mod utilities;
mod primitives;
mod entities;
mod lights;
mod render;

use std::time::Instant;

use primitives::color::Color;
use primitives::vector3::Vector3;
use primitives::transform::Transform;

use entities::sphere::Sphere;

use render::camera::Camera;
use render::scene::Scene;

use lights::omni_light::OmniLight;

use minifb::{Key, Window, WindowOptions};

use crate::entities::Triangle;

const WIDTH: usize = 1500;
const HEIGHT: usize = 1000;

fn main() {
	let mut window = Window::new(
		"Eggsecuter Raytracer",
		WIDTH,
		HEIGHT,
		WindowOptions::default(),
	)
	.unwrap();

	let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	let camera = Camera::new(None, None, Some(WIDTH as f32 / HEIGHT as f32));
	let mut scene = Scene::new(camera, WIDTH, HEIGHT);

	let head = Sphere::new(
		Color::BLUE,
		Transform::new(Some(Vector3::new(0.0, 0.0, 5.0)), None),
		1.0,
	);

	let right_eye = Sphere::new(
		Color::WHITE,
		Transform::new(Some(Vector3::new(0.3, 0.3, 4.1)), None),
		0.2,
	);

	let right_pupil = Sphere::new(
		Color::GREEN,
		Transform::new(Some(Vector3::new(0.3, 0.25, 3.9)), None),
		0.05,
	);

	let left_eye = Sphere::new(
		Color::WHITE,
		Transform::new(Some(Vector3::new(-0.3, 0.3, 4.1)), None),
		0.2,
	);

	let left_pupil = Sphere::new(
		Color::GREEN,
		Transform::new(Some(Vector3::new(-0.3, 0.25, 3.9)), None),
		0.05,
	);

	let nose = Sphere::new(
		Color::RED,
		Transform::new(Some(Vector3::new(0.0, -0.25, 4.0)), None),
		0.2,
	);

	let other = Sphere::new(
		Color::GREEN,
		Transform::new(Some(Vector3::new(-2.0, 3.0, 7.0)), None),
		0.5,
	);

	let triangle = Triangle::new(
		Color::MAGENTA,
		Vector3::new(3.0, -1.0, 2.0),
		Vector3::new(-3.0, -1.0, 2.0),
		Vector3::new(-3.0, -1.0, 7.0)
	);

	let triangle2 = Triangle::new(
		Color::CYAN,
		Vector3::new(3.0, -1.0, 2.0),
		Vector3::new(-3.0, -1.0, 7.0),
		Vector3::new(3.0, -1.0, 7.0),
	);

	let triangle3 = Triangle::new(
		Color::MAGENTA,
		Vector3::new(3.0, -1.0, 2.0),
		Vector3::new(3.0, -1.0, 7.0),
		Vector3::new(3.0, 3.0, 2.0)
	);

	let triangle4 = Triangle::new(
		Color::CYAN,
		Vector3::new(3.0, 3.0, 2.0),
		Vector3::new(3.0, -1.0, 7.0),
		Vector3::new(3.0, 3.0, 7.0)
	);

	scene.entities.push(Box::new(head));
	scene.entities.push(Box::new(right_eye));
	scene.entities.push(Box::new(right_pupil));
	scene.entities.push(Box::new(left_eye));
	scene.entities.push(Box::new(left_pupil));
	scene.entities.push(Box::new(nose));
	scene.entities.push(Box::new(other));
	scene.entities.push(Box::new(triangle));
	scene.entities.push(Box::new(triangle2));
	scene.entities.push(Box::new(triangle3));
	scene.entities.push(Box::new(triangle4));

	let first_light = OmniLight::new(
		Color::new(0.5, 0.5, 0.4),
		Vector3::new(-5.0, 2.0, 5.0),
	);

	let second_light = OmniLight::new(
		Color::new(0.5, 0.5, 0.4),
		Vector3::new(-5.0, 2.0, 1.0),
	);

	scene.global_lights.push(Box::new(first_light));
	scene.global_lights.push(Box::new(second_light));

	let start = Instant::now();

	scene.render(&mut buffer);

	println!("Rendered in {}ms", start.elapsed().as_millis());

	while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Space) && !window.is_key_down(Key::Enter) {
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
}
