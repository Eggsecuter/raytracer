use std::f32::consts::PI;

use crate::entities::{AabbBox, Quad, Sphere};
use crate::lights::PointLight;
use crate::materials::{DielectricMaterial, Material, MetalMaterial, ShadedMaterial};
use crate::primitives::{Color, Quaternion, Transform, Vector3};
use crate::render::{Camera, Scene};

#[allow(dead_code)]
pub fn build() -> Scene {
	// PROD SETTINGS
	// let width = 3840;
	// let height = 2160;

	// DEMO SETTINGS
	let width = 1920;
	let height = 1080;

	let camera = Camera::new(Some(Transform::new(
		Some(Vector3::new(-1.5, 0.0, 1.0)),
		Some(Quaternion::from_euler(Vector3::new(0.0, PI * 0.12, 0.0))),
	)), None, Some(width as f32 / height as f32));

	let mut scene = Scene::new(camera, width, height);
	// PROD SETTINGS
	// scene.trace_depth = 4;
	// scene.smooth_samples = 4;
	// scene.aa_samples = 256;

	// DEMO SETTINGS
	scene.trace_depth = 4;
	scene.smooth_samples = 4;
	scene.aa_samples = 1;

	let front_left_bottom = Vector3::new(-3.0, -2.0, 3.0);
	let front_right_bottom = Vector3::new(3.0, -2.0, 3.0);
	let front_left_top = Vector3::new(-3.0, 2.0, 3.0);
	let front_right_top = Vector3::new(3.0, 2.0, 3.0);

	let back_left_bottom = Vector3::new(-3.0, -2.0, 9.0);
	let back_right_bottom = Vector3::new(3.0, -2.0, 9.0);
	let back_left_top = Vector3::new(-3.0, 2.0, 9.0);
	let back_right_top = Vector3::new(3.0, 2.0, 9.0);

	let floor = Quad::new(
		front_right_bottom,
		front_left_bottom,
		back_left_bottom,
		back_right_bottom,
		Material::Shaded(ShadedMaterial::from_texture(
				Color::new(0.05, 0.05, 0.05),
				"models/wood_floor.jpg",
			)
			.expect("failed to load wood floor texture")
		),
		None,
	);
	scene.entities.push(Box::new(floor));
	let ceiling = Quad::new(
		front_left_top,
		front_right_top,
		back_right_top,
		back_left_top,
		Material::Shaded(ShadedMaterial::from_texture(
				Color::new(0.05, 0.05, 0.05),
				"models/plaster_ceiling.jpg",
			)
			.expect("failed to load plaster ceiling texture")
		),
		None,
	);
	scene.entities.push(Box::new(ceiling));
	let back_wall = Quad::new(
		back_left_bottom,
		back_left_top,
		back_right_top,
		back_right_bottom,
		Material::Shaded(ShadedMaterial::from_texture(
				Color::new(0.05, 0.05, 0.05),
				"models/plaster_wall.jpg",
			)
			.expect("failed to load plaster wall texture")
		),
		None,
	);
	scene.entities.push(Box::new(back_wall));
	let left_wall = Quad::new(
		front_left_bottom,
		front_left_top,
		back_left_top,
		back_left_bottom,
		Material::Shaded(ShadedMaterial::from_texture(
				Color::new(0.05, 0.05, 0.05),
				"models/plaster_wall.jpg",
			)
			.expect("failed to load plaster wall texture")
		),
		None,
	);
	scene.entities.push(Box::new(left_wall));
	let right_wall = Quad::new(
		front_right_bottom,
		back_right_bottom,
		back_right_top,
		front_right_top,
		Material::Shaded(ShadedMaterial::from_texture(
				Color::new(0.05, 0.05, 0.05),
				"models/brick_wall.jpg",
			)
			.expect("failed to load brick wall texture")
		),
		None,
	);
	scene.entities.push(Box::new(right_wall));
	let front_wall = Quad::new(
		front_right_bottom,
		front_right_top,
		front_left_top,
		front_left_bottom,
		Material::Shaded(ShadedMaterial::from_texture(
			Color::new(0.05, 0.05, 0.05),
			"models/plaster_wall.jpg",
		)
		.expect("failed to load plaster wall texture")),
		None,
	);
	scene.entities.push(Box::new(front_wall));

	scene
		.add_obj_mesh(
			"models/couch/source/couch.obj",
			Material::Shaded(
				ShadedMaterial::from_texture(
					Color::new(0.05, 0.05, 0.05),
					"models/couch/textures/couch.png",
				)
				.expect("failed to load couch texture"),
			),
			Vector3::new(-1.3, -1.45, 8.0),
			2.7,
			Quaternion::from_euler(Vector3::new(0.0, -PI * 0.75, 0.0)),
		)
		.expect("failed to load couch mesh");

	scene
		.add_obj_mesh(
			"models/statue/statue.obj",
			Material::Metal(MetalMaterial::SILVER),
			Vector3::new(2.1, -2.0, 8.3),
			1.5,
			Quaternion::from_euler(Vector3::new(0.0, PI * 0.15, 0.0)),
		)
		.expect("failed to load statue mesh");

	scene
		.add_obj_mesh(
			"models/plant/plant.obj",
			Material::Shaded(
				ShadedMaterial::from_texture(
					Color::new(0.05, 0.05, 0.05),
					"models/plant/plant.png",
				)
				.expect("failed to load plant texture"),
			),
			Vector3::new(-2.4, -2.0, 3.5),
			0.3,
			Quaternion::from_euler(Vector3::new(0.0, PI * 0.85, 0.0)),
		)
		.expect("failed to load plant mesh");

	scene
		.add_obj_mesh(
			"models/arcade/arcade.obj",
			Material::Shaded(
				ShadedMaterial::from_texture(
					Color::new(0.05, 0.05, 0.05),
					"models/arcade/arcade.png",
				)
				.expect("failed to load arcade texture"),
			),
			Vector3::new(2.5, -2.0, 4.5),
			0.04,
			Quaternion::from_euler(Vector3::new(0.0, -PI * 0.5, 0.0)),
		)
		.expect("failed to load arcade mesh");

	let aabb_box = AabbBox::new(
		Material::Shaded(ShadedMaterial::from_color(Color::WHITE)),
		Transform::new(Some(Vector3::new(-1.8, -1.5, 5.0)), None),
		Vector3::new(2.0, -1.0, 1.25),
	);
	scene.entities.push(Box::new(aabb_box));

	let shaded_ball = Sphere::new(
		Material::Shaded(ShadedMaterial::from_color(Color::RED)),
		Transform::new(Some(Vector3::new(-0.5, -1.3, 5.2)), None),
		0.2,
	);
	scene.entities.push(Box::new(shaded_ball));

	let glass_ball = Sphere::new(
		Material::Dielectric(DielectricMaterial::GLASS_TRANSPARENT),
		Transform::new(Some(Vector3::new(-1.4, -1.1, 5.8)), None),
		0.4,
	);
	scene.entities.push(Box::new(glass_ball));

	let glass_ball = Sphere::new(
		Material::Dielectric(DielectricMaterial::GLASS),
		Transform::new(Some(Vector3::new(-0.6, -1.2, 5.9)), None),
		0.3,
	);
	scene.entities.push(Box::new(glass_ball));

	scene
		.add_obj_mesh(
			"models/diamond/diamond.obj",
			Material::Dielectric(DielectricMaterial::EMERALD),
			Vector3::new(-1.0, -1.5, 5.5),
			0.2,
			Quaternion::from_euler(Vector3::new(-PI * 0.25, -PI * 0.2, 0.0)),
		)
		.expect("failed to load diamond mesh");

	let metal_ball = Sphere::new(
		Material::Metal(MetalMaterial::MIRROR),
		Transform::new(Some(Vector3::new(0.0, -1.25, 5.4)), None),
		0.25,
	);
	scene.entities.push(Box::new(metal_ball));

	scene
		.add_obj_mesh(
			"models/ceiling_lamp/ceiling_lamp.obj",
			Material::Shaded(
				ShadedMaterial::from_texture(
					Color::new(0.05, 0.05, 0.05),
					"models/ceiling_lamp/ceiling_lamp.png",
				)
				.expect("failed to load ceiling lamp texture"),
			),
			Vector3::new(0.0, 1.5, 5.0),
			0.7,
			Quaternion::from_euler(Vector3::new(0.0, 0.0, 0.0)),
		)
		.expect("failed to load ceiling lamp mesh");

	scene
		.add_obj_mesh(
			"models/frame/source/frame.obj",
			Material::Shaded(
				ShadedMaterial::from_texture(
					Color::new(0.05, 0.05, 0.05),
					"models/frame/textures/frame.jpeg",
				)
				.expect("failed to load frame texture"),
			),
			Vector3::new(0.0, 0.8, 8.95),
			0.4,
			Quaternion::from_euler(Vector3::new(0.0, PI * 0.5, 0.0)),
		)
		.expect("failed to load frame mesh");

	let mirror = Quad::new(
		Vector3::new(-0.8, 0.38, 8.95),
		Vector3::new(-0.8, 1.25, 8.95),
		Vector3::new(0.8, 1.25, 8.95),
		Vector3::new(0.8, 0.38, 8.95),
		Material::Metal(MetalMaterial::MIRROR),
		None,
	);
	scene.entities.push(Box::new(mirror));

	let poster = Quad::new(
		Vector3::new(2.0, -0.28, 3.01),
		Vector3::new(2.0, 1.8, 3.01),
		Vector3::new(0.5, 1.8, 3.01),
		Vector3::new(0.5, -0.28, 3.01),
		Material::Shaded(ShadedMaterial::from_texture(
			Color::new(0.05, 0.05, 0.05),
			"models/cyberpunk_poster.jpg",
		).expect("failed to load cyberpunk poster texture")),
		None,
	);
	scene.entities.push(Box::new(poster));

	scene
		.add_obj_mesh(
			"models/clock/clock.obj",
			Material::Shaded(
				ShadedMaterial::from_color(Color::new(1.0, 0.05, 0.1))
			),
			Vector3::new(-3.0, 1.0, 5.0),
			1.0,
			Quaternion::from_euler(Vector3::new(0.0, PI * 0.5, 0.0)),
		)
		.expect("failed to load clock mesh");

	// main light
	scene.global_lights.push(Box::new(
		PointLight::new(Color::new(0.988, 0.976, 0.85), Vector3::new(0.0, 0.0, 5.0), 8.0)
	));
	scene.global_lights.push(Box::new(
		PointLight::new(Color::new(0.988, 0.976, 0.85), Vector3::new(0.0, 1.0, 5.0), 2.0)
	));

	// support lights
	scene.global_lights.push(Box::new(
		PointLight::new(Color::new(0.33, 0.83, 1.0), Vector3::new(-2.6, -1.0, 3.2), 2.0)
	));
	scene.global_lights.push(Box::new(
		PointLight::new(Color::new(0.988, 0.976, 0.85), Vector3::new(-1.8, -1.0, 6.5), 2.0)
	));

	// LED arcade lights
	scene.global_lights.push(Box::new(
		PointLight::new(Color::new(0.0, 1.0, 0.0), Vector3::new(2.5, -1.5, 3.1), 1.0)
	));
	scene.global_lights.push(Box::new(
		PointLight::new(Color::new(0.988, 0.976, 0.85), Vector3::new(2.5, -1.0, 3.1), 1.0)
	));
	scene.global_lights.push(Box::new(
		PointLight::new(Color::new(0.941, 0.235, 1.0), Vector3::new(2.5, 0.0, 4.5), 5.0)
	));

	scene
}
