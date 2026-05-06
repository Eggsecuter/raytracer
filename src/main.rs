mod entities;
mod lights;
mod primitives;
mod render;
mod utilities;

use std::time::Instant;

use primitives::{Color, Transform, Vector3};

use entities::{Quad, Sphere};

use render::{Camera, Scene};

use lights::OmniLight;

use minifb::{Key, Window, WindowOptions};

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

	let front_left_bottom = Vector3::new(-3.0, -2.0, 3.0);
	let front_right_bottom = Vector3::new(3.0, -2.0, 3.0);
	let front_left_top = Vector3::new(-3.0, 2.0, 3.0);
	let front_right_top = Vector3::new(3.0, 2.0, 3.0);

	let back_left_bottom = Vector3::new(-3.0, -2.0, 9.0);
	let back_right_bottom = Vector3::new(3.0, -2.0, 9.0);
	let back_left_top = Vector3::new(-3.0, 2.0, 9.0);
	let back_right_top = Vector3::new(3.0, 2.0, 9.0);

	scene.entities.push(Box::new(Quad::new(
		Color::WHITE,
		front_right_bottom,
		front_left_bottom,
		back_left_bottom,
		back_right_bottom,
	)));
	scene.entities.push(Box::new(Quad::new(
		Color::WHITE,
		front_left_top,
		front_right_top,
		back_right_top,
		back_left_top,
	)));
	scene.entities.push(Box::new(Quad::new(
		Color::WHITE,
		back_left_bottom,
		back_left_top,
		back_right_top,
		back_right_bottom,
	)));
	scene.entities.push(Box::new(Quad::new(
		Color::RED,
		front_left_bottom,
		front_left_top,
		back_left_top,
		back_left_bottom,
	)));
	scene.entities.push(Box::new(Quad::new(
		Color::GREEN,
		front_right_bottom,
		back_right_bottom,
		back_right_top,
		front_right_top,
	)));

	scene.entities.push(Box::new(Sphere::new(
		Color::new(0.7, 0.7, 0.85),
		Transform::new(Some(Vector3::new(-0.85, -1.15, 6.0)), None),
		0.85,
	)));
	scene.entities.push(Box::new(Sphere::new(
		Color::new(0.85, 0.75, 0.55),
		Transform::new(Some(Vector3::new(1.05, -1.35, 7.2)), None),
		0.65,
	)));

	let ceiling_light = OmniLight::new(Color::new(0.45, 0.43, 0.38), Vector3::new(0.0, 1.85, 5.8));

	scene.global_lights.push(Box::new(ceiling_light));

	let start = Instant::now();
	scene.render(&mut buffer);
	println!("Rendered in {}ms", start.elapsed().as_millis());

	while window.is_open()
		&& !window.is_key_down(Key::Escape)
		&& !window.is_key_down(Key::Space)
		&& !window.is_key_down(Key::Enter)
	{
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
}
