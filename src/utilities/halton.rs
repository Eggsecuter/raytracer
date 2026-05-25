pub fn halton(mut i: u32, base: u32) -> f32 {
	let mut fraction = 1.0_f32;
	let mut result = 0.0_f32;

	while i > 0 {
		fraction /= base as f32;
		result += fraction * (i % base) as f32;
		i /= base;
	}

	result
}

pub fn halton_2d(i: u32) -> (f32, f32) {
	(halton(i, 2), halton(i, 3))
}
