mod entities;
mod lights;
mod materials;
mod primitives;
mod render;
mod scenes;
mod utilities;

use image::{ImageBuffer, Rgb};
use minifb::{Key, Window, WindowOptions};
use std::fs;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

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

	save_buffer_as_jpg(&buffer, WIDTH, HEIGHT);

	while window.is_open()
		&& !window.is_key_down(Key::Escape)
		&& !window.is_key_down(Key::Space)
		&& !window.is_key_down(Key::Enter)
	{
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
	}
}

fn save_buffer_as_jpg(buffer: &[u32], width: usize, height: usize) {
	fs::create_dir_all("output").unwrap();

	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	let path = format!("output/{}.jpg", timestamp);

	let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(
		width as u32,
		height as u32,
	);

	for y in 0..height {
		for x in 0..width {
			let pixel = buffer[y * width + x];

			let r = ((pixel >> 16) & 0xFF) as u8;
			let g = ((pixel >> 8) & 0xFF) as u8;
			let b = (pixel & 0xFF) as u8;

			img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
		}
	}

	img.save(&path).unwrap();

	println!("Saved image to {}", path);
}
