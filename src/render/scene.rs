use crate::entities::entity::Entity;
use crate::lights::light::Light;
use crate::materials::{DielectricMaterial, LambertMaterial, Material, MetalMaterial};
use crate::primitives::color::Color;
use crate::primitives::ray_hit::RayHit;
use crate::primitives::{Ray, Vector3};
use crate::render::Camera;

use rayon::prelude::*;

pub struct Scene {
	pub camera: Camera,
	pub width: usize,
	pub height: usize,

	pub entities: Vec<Box<dyn Entity>>,
	pub global_lights: Vec<Box<dyn Light>>,

	background_color: Color,
	trace_depth: i32,
	smooth_samples: i32,
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
			trace_depth: 4,
			smooth_samples: 32,
		}
	}

	pub fn render(&self, buffer: &mut [u32]) {
		buffer
			.par_chunks_mut(self.width)
			.enumerate()
			.for_each(|(y, row)| {
				for (x, pixel) in row.iter_mut().enumerate().take(self.width) {
					let ray = self.camera.get_ray(
						x as f32,
						y as f32,
						self.width as f32,
						self.height as f32,
					);
					let color = self.trace_ray(ray, self.trace_depth).clamped();

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

		let hit = match self.intersect(ray) {
			Some(intersection) => intersection,
			None => return self.background_color,
		};

		match hit.material {
			Material::Lambert(material) => return self.shade(hit, material),

			Material::Metal(material) => {
				return self.reflect(ray, hit, material, depth) * material.specular;
			}

			Material::Dielectric(material) => return self.refract(ray, hit, material, depth),
		}
	}

	fn intersect(&self, ray: Ray) -> Option<RayHit> {
		// find the closest intersection
		let mut closest_hit: Option<RayHit> = None;

		for entity in &self.entities {
			if let Some(hit) = entity.intersect(&ray)
				&& (closest_hit.is_none() || hit.distance <= closest_hit.as_ref().unwrap().distance)
			{
				closest_hit = Some(hit);
			}
		}

		return closest_hit;
	}

	fn shade(&self, hit: RayHit, material: LambertMaterial) -> Color {
		// calculate diffuse color
		let mut diffuse_color = Color::BLACK;
		let mut in_light_count = 0;

		for light in &self.global_lights {
			let to_light = light.position() - hit.point;
			let distance_to_light = to_light.length();
			let shadow_ray = Ray::new(hit.point + hit.normal * 1e-4, to_light.normalize(), true);

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

		return material.albedo * diffuse_color + material.ambient;
	}

	fn reflect(&self, ray: Ray, hit: RayHit, material: MetalMaterial, depth: i32) -> Color {
		let ideal_reflection = self.get_reflected_direction(ray, hit);
		let reflection_point = self.get_offset_point(hit);

		let roughness = 1.0 - material.smoothness;
		let mut accumulated = Color::BLACK;

		for _ in 0..self.smooth_samples {
			// generate random perturbation in unit sphere
			let mut random_dir;
			loop {
				random_dir = Vector3::new(
					rand::random::<f32>() * 2.0 - 1.0,
					rand::random::<f32>() * 2.0 - 1.0,
					rand::random::<f32>() * 2.0 - 1.0,
				);

				if random_dir.length_squared() <= 1.0 {
					break;
				}
			}

			// blend reflection with noise
			let direction = (ideal_reflection + random_dir * roughness).normalize();

			let reflected_ray = Ray::new(reflection_point, direction, true);
			accumulated = accumulated + self.trace_ray(reflected_ray, depth - 1);
		}

		accumulated * (1.0 / self.smooth_samples as f32)
	}

	fn refract(&self, ray: Ray, hit: RayHit, material: DielectricMaterial, depth: i32) -> Color {
		let (eta1, eta2) = if hit.front_face {
			(1.0, material.refractive_index)
		} else {
			(material.refractive_index, 1.0)
		};

		let reflected_ray = Ray::new(
			self.get_offset_point(hit),
			self.get_reflected_direction(ray, hit),
			hit.front_face,
		);
		let reflected_color = self.trace_ray(reflected_ray, depth - 1);

		let eta = eta1 / eta2;
		let cos_theta = -ray.direction.dot(&hit.normal);

		let k = 1.0 - eta * eta * (1.0 - cos_theta * cos_theta);

		// total reflection
		if k < 0.0 {
			return reflected_color;
		}

		let refracted_direction =
			ray.direction.normalize() * eta + hit.normal.normalize() * (eta * cos_theta - k.sqrt());
		let refracted_ray = Ray::new(
			self.get_negative_offset_point(hit),
			refracted_direction,
			!hit.front_face,
		);
		let mut refracted_color = self.trace_ray(refracted_ray, depth - 1);

		if !hit.front_face {
			refracted_color *= (material.absorption * -hit.distance).exp();
		}

		let fresnel_factor = self.fresnel_schlick(eta1, eta2, cos_theta);

		return reflected_color * fresnel_factor + refracted_color * (1.0 - fresnel_factor);
	}

	fn get_offset_point(&self, hit: RayHit) -> Vector3 {
		hit.point + hit.normal * 1e-3
	}

	fn get_negative_offset_point(&self, hit: RayHit) -> Vector3 {
		hit.point - hit.normal * 1e-3
	}

	fn get_reflected_direction(&self, ray: Ray, hit: RayHit) -> Vector3 {
		ray.direction - hit.normal * 2.0 * ray.direction.dot(&hit.normal)
	}

	fn fresnel_schlick(&self, eta1: f32, eta2: f32, cos_theta: f32) -> f32 {
		let r0 = ((eta1 - eta2) / (eta1 + eta2)).powi(2);

		r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
	}
}
