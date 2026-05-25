#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::entities::{Mesh, Triangle};
use crate::materials::Material;
use crate::primitives::{Vector3, UV};

pub struct ObjReader;

#[derive(Debug, Clone, Copy)]
struct FaceVertex {
	vertex_index: usize,
	uv_index: Option<usize>,
	normal_index: Option<usize>,
}

impl ObjReader {
	pub fn read_mesh<P: AsRef<Path>>(path: P, material: Material) -> io::Result<Mesh> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);

		let mut vertices: Vec<Vector3> = Vec::new();
		let mut tex_coords: Vec<UV> = Vec::new();
		let mut normals: Vec<Vector3> = Vec::new();
		let mut triangles: Vec<Triangle> = Vec::new();

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
				"v" => vertices.push(parse_vec3(parts, line_index)?),
				"vt" => tex_coords.push(parse_uv(parts, line_index)?),
				"vn" => normals.push(parse_vec3(parts, line_index)?.normalize()),
				"f" => {
					let face_vertices = parts
						.map(|part| {
							parse_face_vertex(
								part,
								vertices.len(),
								tex_coords.len(),
								normals.len(),
								line_index,
							)
						})
						.collect::<io::Result<Vec<_>>>()?;

					if face_vertices.len() < 3 {
						return Err(invalid_data(
							line_index,
							"face must contain at least 3 vertices",
						));
					}

					// Fan-triangulate convex polygon faces.
					for i in 1..face_vertices.len() - 1 {
						let a = face_vertices[0];
						let b = face_vertices[i];
						let c = face_vertices[i + 1];

						let geo_normal = face_normal([a, b, c], &normals);

						// Per-vertex UVs — fall back to sensible defaults if the OBJ
						// does not contain texture coordinates.
						let uv_a = resolve_uv(a.uv_index, &tex_coords);
						let uv_b = resolve_uv(b.uv_index, &tex_coords);
						let uv_c = resolve_uv(c.uv_index, &tex_coords);

						triangles.push(Triangle::with_uvs(
							vertices[a.vertex_index],
							vertices[b.vertex_index],
							vertices[c.vertex_index],
							uv_a,
							uv_b,
							uv_c,
							material.clone(),
							geo_normal,
						));
					}
				}
				_ => {}
			}
		}

		Ok(Mesh::new(triangles))
	}
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_vec3<'a>(
	mut parts: impl Iterator<Item = &'a str>,
	line_index: usize,
) -> io::Result<Vector3> {
	let x = parse_f32(parts.next(), line_index, "missing x coordinate")?;
	let y = parse_f32(parts.next(), line_index, "missing y coordinate")?;
	let z = parse_f32(parts.next(), line_index, "missing z coordinate")?;
	Ok(Vector3::new(x, y, z))
}

fn parse_uv<'a>(mut parts: impl Iterator<Item = &'a str>, line_index: usize) -> io::Result<UV> {
	let u = parse_f32(parts.next(), line_index, "missing u coordinate")?;
	let v = parse_f32(parts.next(), line_index, "missing v coordinate")?;
	// OBJ stores V with 0 at the bottom; most renderers treat 0 at the top, so
	// flip V to match the convention used by common image loaders.
	Ok(UV::new(u, 1.0 - v))
}

fn parse_f32(value: Option<&str>, line_index: usize, missing_message: &str) -> io::Result<f32> {
	let value = value.ok_or_else(|| invalid_data(line_index, missing_message))?;
	value
		.parse::<f32>()
		.map_err(|_| invalid_data(line_index, "invalid floating-point value"))
}

/// Parse a single `v/vt/vn` token on a face line.
///
/// All three components are optional in the OBJ spec:
/// - `v` only:       `1`
/// - `v` and `vn`:  `1//2`
/// - all three:      `1/2/3`
fn parse_face_vertex(
	value: &str,
	vertex_count: usize,
	uv_count: usize,
	normal_count: usize,
	line_index: usize,
) -> io::Result<FaceVertex> {
	let mut parts = value.split('/');

	let vertex_index = parse_obj_index(
		parts.next().filter(|p| !p.is_empty()),
		vertex_count,
		line_index,
		"missing vertex index",
	)?;

	let uv_index = match parts.next().filter(|p| !p.is_empty()) {
		Some(idx) => Some(parse_obj_index(
			Some(idx),
			uv_count,
			line_index,
			"invalid texture-coordinate index",
		)?),
		None => None,
	};

	let normal_index = match parts.next().filter(|p| !p.is_empty()) {
		Some(idx) => Some(parse_obj_index(
			Some(idx),
			normal_count,
			line_index,
			"invalid normal index",
		)?),
		None => None,
	};

	Ok(FaceVertex {
		vertex_index,
		uv_index,
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

/// Average the vertex normals to get a smooth face normal; returns `None` when
/// any of the three vertices lack a normal index (geometry normal is used instead).
fn face_normal(face_vertices: [FaceVertex; 3], normals: &[Vector3]) -> Option<Vector3> {
	let mut normal = Vector3::ZERO;
	for fv in face_vertices {
		normal += normals[fv.normal_index?];
	}
	Some(normal.normalize())
}

/// Return the UV for this face-vertex, or fall back to a default when the OBJ
/// does not include texture coordinates for this vertex.
fn resolve_uv(uv_index: Option<usize>, tex_coords: &[UV]) -> UV {
	uv_index
		.and_then(|i| tex_coords.get(i).copied())
		.unwrap_or(UV::ZERO)
}

fn invalid_data(line_index: usize, message: &str) -> io::Error {
	io::Error::new(
		io::ErrorKind::InvalidData,
		format!("line {}: {}", line_index + 1, message),
	)
}
