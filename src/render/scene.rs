use crate::entities::entity::Entity;
use crate::lights::light::Light;
use crate::primitives::Ray;
use crate::primitives::color::Color;
use crate::primitives::ray_hit::RayHit;
use crate::render::Camera;

use rayon::prelude::*;

pub struct Scene {
	pub camera: Camera,
	pub width: usize,
	pub height: usize,

	pub entities: Vec<Box<dyn Entity>>,
	pub global_lights: Vec<Box<dyn Light>>,

	background_color: Color,
	trace_depth: i32
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
			trace_depth: 10
		}
	}

	pub fn render(&self, buffer: &mut [u32]) {
		buffer
			.par_chunks_mut(self.width)
			.enumerate()
			.for_each(|(y, row)| {
				for (x, pixel) in row.iter_mut().enumerate().take(self.width) {
					let ray = self.camera.get_ray(x as f32, y as f32, self.width as f32, self.height as f32);
					let color = self.trace_ray(ray, self.trace_depth);

					let r = (color.red * 255.0) as u32;
					let g = (color.green * 255.0) as u32;
					let b = (color.blue * 255.0) as u32;

					*pixel = (r << 16) | (g << 8) | b;
				}
			});
	}

	fn trace_ray(&self, ray: Ray, depth: i32) -> Color {
		if depth == 0 {
			return self.background_color;
		}

		let intersection = self.intersect(ray);

		let hit = match intersection {
			Some(intersection) => intersection,
			None => return self.background_color
		};

		let shaded_color = self.shade(hit);

		if hit.material.smoothness <= 0.0 {
			return shaded_color;
		}

		let reflected_ray = self.reflect(ray, hit);
		let reflected_color = self.trace_ray(reflected_ray, depth - 1);

		// blend rays
		return shaded_color * (1.0 - hit.material.smoothness)
			+ reflected_color * hit.material.smoothness;
	}

	fn intersect(&self, ray: Ray) -> Option<RayHit> {
		// find the closest intersection
		let mut closest_hit: Option<RayHit> = None;

		for entity in &self.entities {
			if let Some(hit) = entity.intersect(&ray)
				&& (closest_hit.is_none()
					|| hit.distance <= closest_hit.as_ref().unwrap().distance)
			{
				closest_hit = Some(hit);
			}
		}

		return closest_hit
	}

	fn reflect(&self, ray: Ray, hit: RayHit) -> Ray {
		let point = hit.point + hit.normal * 1e-2;
		let direction = ray.direction - hit.normal * 2.0 * ray.direction.dot(&hit.normal);

		return Ray::new(point, direction);
	}

	fn shade(&self, hit: RayHit) -> Color {
		// calculate diffuse color
		let mut diffuse_color = Color::BLACK;
		let mut in_light_count = 0;

		for light in &self.global_lights {
			let to_light = light.position() - hit.point;
			let distance_to_light = to_light.length();
			let shadow_ray = Ray::new(
				hit.point + hit.normal * 1e-4,
				to_light.normalize(),
			);

			// check if point is in shadow
			let mut in_light = true;

			for other_entity in &self.entities {
				if let Some(hit) = other_entity.intersect(&shadow_ray)
					&& hit.distance < distance_to_light
				{
					in_light = false;
					break;
				}
			}

			if in_light {
				diffuse_color += light.calculate_color(&hit);
				in_light_count += 1;
			}
		}

		// final shadow factor
		diffuse_color *= in_light_count as f32 / self.global_lights.len().max(1) as f32;

		return hit.material.diffuse_color * diffuse_color + hit.material.ambient_color
	}
}
