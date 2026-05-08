mod entities;
mod lights;
mod primitives;
mod render;
mod scenes;
mod utilities;

use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use scenes::cornell_box;

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
	let scene = cornell_box::build(WIDTH, HEIGHT);

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
