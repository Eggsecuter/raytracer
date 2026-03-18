use crate::primitives::Ray;
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

		// find the closest intersection
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

		// unwrap intersection or return background
		let intersection = match closest_intersection {
			Some(hit) => hit,
			None => return self.background_color,
		};

		// calculate diffuse color
		let mut diffuse_color = Color::BLACK;
		let mut in_light_count = 0;

		for light in &self.global_lights {
			let to_light = light.position() - intersection.point;
			let distance_to_light = to_light.length();
			let shadow_ray = Ray::new(intersection.point + intersection.normal * 1e-4, to_light.normalize());

			// check if point is in shadow
			let mut in_light = true;

			for other_entity in &self.entities {
				if let Some(hit) = other_entity.intersect(&shadow_ray) {
					if hit.distance < distance_to_light {
						in_light = false;
						break;
					}
				}
			}

			if in_light {
				diffuse_color += light.calculate_color(&intersection);
				in_light_count += 1;
			}
		}

		// final shadow factor
		diffuse_color *= in_light_count as f64 / self.global_lights.iter().count().max(1) as f64;

		closest_entity.unwrap().color() * diffuse_color + self.ambient_light
	}
}
