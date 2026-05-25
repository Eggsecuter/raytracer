#![allow(dead_code)]

use std::io;
use std::path::Path;

use crate::entities::{Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Quaternion, Ray, RayHit, Vector3};
use crate::utilities::ObjReader;

#[derive(Debug, Clone)]
pub struct Mesh {
	pub triangles: Vec<Triangle>,
}

impl Mesh {
	pub fn new(triangles: Vec<Triangle>) -> Self {
		Self { triangles }
	}

	pub fn from_obj<P: AsRef<Path>>(
		path: P,
		material: Material,
		position: Vector3,
		scale: f32,
		rotation: Quaternion,
	) -> io::Result<Self> {
		Ok(ObjReader::read_mesh(path, material)?
			.scaled(scale)
			.rotated(rotation)
			.translated(position))
	}

	pub fn translated(&self, offset: Vector3) -> Self {
		Self {
			triangles: self
				.triangles
				.iter()
				.map(|triangle| triangle.translated(offset))
				.collect(),
		}
	}

	pub fn rotated(&self, rotation: Quaternion) -> Self {
		Self {
			triangles: self
				.triangles
				.iter()
				.map(|triangle| triangle.rotated(rotation))
				.collect(),
		}
	}

	pub fn scaled(&self, factor: f32) -> Self {
		Self {
			triangles: self
				.triangles
				.iter()
				.map(|triangle| triangle.scaled(factor))
				.collect(),
		}
	}
}

impl Entity for Mesh {
	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		let mut closest_hit = None;

		for triangle in &self.triangles {
			if let Some(hit) = triangle.intersect(ray)
				&& closest_hit.is_none_or(|closest: RayHit| hit.distance < closest.distance)
			{
				closest_hit = Some(hit);
			}
		}

		closest_hit
	}
}
