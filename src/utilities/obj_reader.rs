#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::entities::{Mesh, Triangle};
use crate::materials::Material;
use crate::primitives::Vector3;

pub struct ObjReader;

#[derive(Debug, Clone, Copy)]
struct FaceVertex {
	vertex_index: usize,
	normal_index: Option<usize>,
}

impl ObjReader {
	pub fn read_mesh<P: AsRef<Path>>(path: P, material: Material) -> io::Result<Mesh> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let mut vertices = Vec::new();
		let mut normals = Vec::new();
		let mut triangles = Vec::new();

		for (line_index, line) in reader.lines().enumerate() {
			let line = line?;
			let line = line.split('#').next().unwrap_or("").trim();

			if line.is_empty() {
				continue;
			}

			let mut parts = line.split_whitespace();
			let Some(record_type) = parts.next() else {
				continue;
			};

			match record_type {
				"v" => vertices.push(parse_vector(parts, line_index)?),
				"vn" => normals.push(parse_vector(parts, line_index)?.normalize()),
				"f" => {
					let face_vertices = parts
						.map(|part| {
							parse_face_vertex(part, vertices.len(), normals.len(), line_index)
						})
						.collect::<io::Result<Vec<_>>>()?;

					if face_vertices.len() < 3 {
						return Err(invalid_data(
							line_index,
							"face must contain at least 3 vertices",
						));
					}

					for i in 1..face_vertices.len() - 1 {
						let a = face_vertices[0];
						let b = face_vertices[i];
						let c = face_vertices[i + 1];
						let normal = face_normal([a, b, c], &normals);

						triangles.push(Triangle::new(
							vertices[a.vertex_index],
							vertices[b.vertex_index],
							vertices[c.vertex_index],
							material,
							normal,
						));
					}
				}
				_ => {}
			}
		}

		Ok(Mesh::new(triangles))
	}
}

fn parse_vector<'a>(
	mut parts: impl Iterator<Item = &'a str>,
	line_index: usize,
) -> io::Result<Vector3> {
	let x = parse_f32(parts.next(), line_index, "missing x coordinate")?;
	let y = parse_f32(parts.next(), line_index, "missing y coordinate")?;
	let z = parse_f32(parts.next(), line_index, "missing z coordinate")?;

	Ok(Vector3::new(x, y, z))
}

fn parse_f32(value: Option<&str>, line_index: usize, missing_message: &str) -> io::Result<f32> {
	let value = value.ok_or_else(|| invalid_data(line_index, missing_message))?;

	value
		.parse::<f32>()
		.map_err(|_| invalid_data(line_index, "invalid floating-point value"))
}

fn parse_face_vertex(
	value: &str,
	vertex_count: usize,
	normal_count: usize,
	line_index: usize,
) -> io::Result<FaceVertex> {
	let mut parts = value.split('/');
	let vertex_index = parse_obj_index(
		parts.next().filter(|part| !part.is_empty()),
		vertex_count,
		line_index,
		"missing vertex index",
	)?;

	let _texture_index = parts.next();
	let normal_index = match parts.next().filter(|part| !part.is_empty()) {
		Some(index) => Some(parse_obj_index(
			Some(index),
			normal_count,
			line_index,
			"missing normal index",
		)?),
		None => None,
	};

	Ok(FaceVertex {
		vertex_index,
		normal_index,
	})
}

fn parse_obj_index(
	value: Option<&str>,
	item_count: usize,
	line_index: usize,
	missing_message: &str,
) -> io::Result<usize> {
	let value = value.ok_or_else(|| invalid_data(line_index, missing_message))?;
	let index = value
		.parse::<isize>()
		.map_err(|_| invalid_data(line_index, "invalid face index"))?;

	if index == 0 {
		return Err(invalid_data(line_index, "obj indices are 1-based"));
	}

	let resolved = if index > 0 {
		index - 1
	} else {
		item_count as isize + index
	};

	if resolved < 0 || resolved >= item_count as isize {
		return Err(invalid_data(line_index, "face index out of bounds"));
	}

	Ok(resolved as usize)
}

fn face_normal(face_vertices: [FaceVertex; 3], normals: &[Vector3]) -> Option<Vector3> {
	let mut normal = Vector3::ZERO;

	for face_vertex in face_vertices {
		let normal_index = face_vertex.normal_index?;
		normal += normals[normal_index];
	}

	Some(normal.normalize())
}

fn invalid_data(line_index: usize, message: &str) -> io::Error {
	io::Error::new(
		io::ErrorKind::InvalidData,
		format!("line {}: {}", line_index + 1, message),
	)
}
