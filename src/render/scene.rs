use std::io;
use std::path::Path;

use crate::entities::{BvhNode, Entity, Mesh};
use crate::lights::light::Light;
use crate::materials::{DielectricMaterial, LambertMaterial, Material, MetalMaterial};
use crate::primitives::color::Color;
use crate::primitives::ray_hit::RayHit;
use crate::primitives::{Quaternion, Ray, Vector3};
use crate::render::Camera;
use crate::utilities::halton_2d;

use rayon::prelude::*;

pub struct Scene {
	pub camera: Camera,
	pub width: usize,
	pub height: usize,

	pub entities: Vec<Box<dyn Entity>>,
	pub global_lights: Vec<Box<dyn Light>>,

	pub background_color: Color,
	pub trace_depth: i32,
	pub smooth_samples: i32,
	pub aa_samples: u32,

	bvh: Option<Box<dyn Entity>>,
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
			smooth_samples: 8,
			aa_samples: 16,
			bvh: None,
		}
	}

	pub fn add_obj_mesh<P: AsRef<Path>>(
		&mut self,
		path: P,
		material: Material,
		position: Vector3,
		scale: f32,
		rotation: Quaternion,
	) -> io::Result<()> {
		self.entities.push(Box::new(Mesh::from_obj(
			path, material, position, scale, rotation,
		)?));

		Ok(())
	}

	pub fn build_bvh(&mut self) {
		if !self.entities.is_empty() {
			let entities = std::mem::take(&mut self.entities);
			self.bvh = Some(BvhNode::build(entities));
		}
	}

	pub fn render(&mut self, buffer: &mut [u32]) {
		self.build_bvh();

		let scene: &Scene = self;

		buffer
			.par_chunks_mut(scene.width)
			.enumerate()
			.for_each(|(y, row)| {
				for (x, pixel) in row.iter_mut().enumerate().take(scene.width) {
					let hdr_color = if scene.aa_samples <= 1 {
						let ray = scene.camera.get_ray(
							x as f32,
							y as f32,
							scene.width as f32,
							scene.height as f32,
							0.5,
							0.5,
						);
						scene.trace_ray(ray, scene.trace_depth)
					} else {
						let mut accumulated = Color::BLACK;
						for s in 1..=scene.aa_samples {
							let (dx, dy) = halton_2d(s);
							let ray = scene.camera.get_ray(
								x as f32,
								y as f32,
								scene.width as f32,
								scene.height as f32,
								dx,
								dy,
							);
							accumulated += scene.trace_ray(ray, scene.trace_depth);
						}
						accumulated * (1.0 / scene.aa_samples as f32)
					};

					let lrd_color = hdr_color.hdr_to_ldr(None);
					let pixel_color = lrd_color.linear_to_srgb();

					let r = (pixel_color.red * 255.0) as u32;
					let g = (pixel_color.green * 255.0) as u32;
					let b = (pixel_color.blue * 255.0) as u32;

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

		// clone the material before matching so `hit` is not partially moved
		// and can still be forwarded to the shading functions intact.
		match hit.material.clone() {
			Material::Lambert(material) => return self.shade(hit, material),

			Material::Metal(material) => {
				return self.reflect(ray, hit, material, depth) * material.specular;
			}

			Material::Dielectric(material) => return self.refract(ray, hit, material, depth),
		}
	}

	fn intersect(&self, ray: Ray) -> Option<RayHit> {
		if let Some(bvh) = &self.bvh {
			return bvh.intersect(&ray);
		}

		// fallback linear search
		let mut closest_hit: Option<RayHit> = None;
		for entity in &self.entities {
			if let Some(hit) = entity.intersect(&ray)
				&& (closest_hit.is_none() || hit.distance <= closest_hit.as_ref().unwrap().distance)
			{
				closest_hit = Some(hit);
			}
		}
		closest_hit
	}

	fn shade(&self, hit: RayHit, material: LambertMaterial) -> Color {
		let mut diffuse_color = Color::BLACK;
		let mut in_light_count = 0;

		for light in &self.global_lights {
			let to_light = light.position() - hit.point;
			let distance_to_light = to_light.length();
			let shadow_ray = Ray::new(hit.point + hit.normal * 1e-4, to_light.normalize(), true);

			// A hit closer than the light means this point is in shadow.
			let in_light = self
				.intersect(shadow_ray)
				.map_or(true, |shadow_hit| shadow_hit.distance >= distance_to_light);

			if in_light {
				diffuse_color += light.calculate_color(&hit);
				in_light_count += 1;
			}
		}

		diffuse_color *= in_light_count as f32 / self.global_lights.len().max(1) as f32;

		material.albedo_at(hit.uv) * diffuse_color + material.ambient
	}

	fn reflect(&self, ray: Ray, hit: RayHit, material: MetalMaterial, depth: i32) -> Color {
		let ideal_reflection = self.get_reflected_direction(ray, &hit);
		let reflection_point = self.get_offset_point(&hit);

		let roughness = 1.0 - material.smoothness;
		let mut accumulated = Color::BLACK;

		for _ in 0..self.smooth_samples {
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
			self.get_offset_point(&hit),
			self.get_reflected_direction(ray, &hit),
			hit.front_face,
		);
		let reflected_color = self.trace_ray(reflected_ray, depth - 1);

		let eta = eta1 / eta2;
		let cos_theta = -ray.direction.dot(&hit.normal);

		let k = 1.0 - eta * eta * (1.0 - cos_theta * cos_theta);

		if k < 0.0 {
			return reflected_color;
		}

		let refracted_direction =
			ray.direction.normalize() * eta + hit.normal.normalize() * (eta * cos_theta - k.sqrt());
		let refracted_ray = Ray::new(
			self.get_negative_offset_point(&hit),
			refracted_direction,
			!hit.front_face,
		);
		let mut refracted_color = self.trace_ray(refracted_ray, depth - 1);

		if !hit.front_face {
			refracted_color *= (material.absorption * -hit.distance).exp();
		}

		let fresnel_factor = self.fresnel_schlick(eta1, eta2, cos_theta);

		reflected_color * fresnel_factor + refracted_color * (1.0 - fresnel_factor)
	}

	fn get_offset_point(&self, hit: &RayHit) -> Vector3 {
		hit.point + hit.normal * 1e-3
	}

	fn get_negative_offset_point(&self, hit: &RayHit) -> Vector3 {
		hit.point - hit.normal * 1e-3
	}

	fn get_reflected_direction(&self, ray: Ray, hit: &RayHit) -> Vector3 {
		ray.direction - hit.normal * 2.0 * ray.direction.dot(&hit.normal)
	}

	fn fresnel_schlick(&self, eta1: f32, eta2: f32, cos_theta: f32) -> f32 {
		let r0 = ((eta1 - eta2) / (eta1 + eta2)).powi(2);
		r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
	}
}
