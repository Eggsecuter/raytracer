#![allow(dead_code)]

use std::io;
use std::path::Path;

use crate::entities::{BvhNode, Entity, Triangle};
use crate::materials::Material;
use crate::primitives::{Aabb, Quaternion, Ray, RayHit, Vector3};
use crate::utilities::ObjReader;

/// A triangle mesh that uses an internal BVH tree to accelerate intersection.
///
/// The raw triangles are retained so that the transform helpers (`translated`,
/// `rotated`, `scaled`) can create new, correctly-transformed meshes whose BVH
/// is rebuilt automatically.
pub struct Mesh {
	pub triangles: Vec<Triangle>,
	/// BVH over the triangles; `None` only for a degenerate empty mesh.
	bvh: Option<Box<dyn Entity>>,
	aabb: Aabb,
}

impl Mesh {
	pub fn new(triangles: Vec<Triangle>) -> Self {
		if triangles.is_empty() {
			return Self {
				triangles: Vec::new(),
				bvh: None,
				aabb: Aabb::empty(),
			};
		}

		// Compute overall AABB and build the BVH in one pass.
		let mut aabb = Aabb::empty();
		let entities: Vec<Box<dyn Entity>> = triangles
			.iter()
			.map(|t| {
				aabb = Aabb::surrounding(aabb, t.bounding_box());
				Box::new(t.clone()) as Box<dyn Entity>
			})
			.collect();

		let bvh = BvhNode::build(entities);

		Self {
			triangles,
			bvh: Some(bvh),
			aabb,
		}
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
		Self::new(
			self.triangles
				.iter()
				.map(|t| t.translated(offset))
				.collect(),
		)
	}

	pub fn rotated(&self, rotation: Quaternion) -> Self {
		Self::new(
			self.triangles
				.iter()
				.map(|t| t.rotated(rotation))
				.collect(),
		)
	}

	pub fn scaled(&self, factor: f32) -> Self {
		Self::new(
			self.triangles
				.iter()
				.map(|t| t.scaled(factor))
				.collect(),
		)
	}
}

impl std::fmt::Debug for Mesh {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Mesh")
			.field("triangles", &self.triangles)
			.field("aabb", &self.aabb)
			.finish_non_exhaustive()
	}
}

impl Entity for Mesh {
	fn bounding_box(&self) -> Aabb {
		self.aabb
	}

	fn intersect(&self, ray: &Ray) -> Option<RayHit> {
		self.bvh.as_ref()?.intersect(ray)
	}
}
