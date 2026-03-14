use crate::render::Camera;
use crate::entities::entity::Entity;
use crate::lights::light::Light;
use crate::primitives::color::Color;
use crate::primitives::ray_hit::RayHit;

use rayon::prelude::*;

pub struct Scene {
	pub camera: Camera,
	pub width: usize,
	pub height: usize,

	pub entities: Vec<Box<dyn Entity>>,
	pub global_lights: Vec<Box<dyn Light>>,

	background_color: Color,
	ambient_light: Color,
}

impl Scene {
	pub fn new(camera: Camera, width: usize, height: usize) -> Self {
		Self {
			camera,
			width,
			height,
			entities: Vec::new(),
			global_lights: Vec::new(),
			background_color: Color::BLACK,
			ambient_light: Color::from_grayscale(0.05),
		}
	}

	pub fn render(&self, buffer: &mut [u32]) {
		buffer
			.par_chunks_mut(self.width)
			.enumerate()
			.for_each(|(y, row)| {
				for x in 0..self.width {
					let color = self.get_pixel_color(x, y);

					let r = (color.red * 255.0) as u32;
					let g = (color.green * 255.0) as u32;
					let b = (color.blue * 255.0) as u32;

					row[x] = (r << 16) | (g << 8) | b;
				}
			});
	}

	fn get_pixel_color(&self, x: usize, y: usize) -> Color {
		let ray = self.camera.get_ray(x, y, self.width, self.height);

		let mut closest_entity: Option<&Box<dyn Entity>> = None;
		let mut closest_intersection: Option<RayHit> = None;

		for entity in &self.entities {
			if let Some(hit) = entity.intersect(&ray) {
				if closest_intersection.is_none()
					|| hit.distance <= closest_intersection.as_ref().unwrap().distance
				{
					closest_entity = Some(entity);
					closest_intersection = Some(hit);
				}
			}
		}

		if closest_entity.is_none() {
			return self.background_color;
		}

		let intersection = closest_intersection.unwrap();

		let mut diffuse_color = Color::BLACK;

		for light in &self.global_lights {
			diffuse_color += light.calculate_color(&intersection);
		}

		closest_entity
			.unwrap()
			.color()
			* diffuse_color
			+ self.ambient_light
	}
}
