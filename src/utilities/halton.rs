/// Evaluate the Halton sequence at index `i` for the given `base`.
///
/// The Halton sequence is a low-discrepancy quasi-random sequence that
/// produces well-distributed samples in [0, 1).  Using base 2 for one axis
/// and base 3 for the other is the standard 2-D Halton construction for
/// anti-aliasing — the two sequences are maximally independent because 2 and
/// 3 are coprime.
///
/// # Examples
/// ```
/// // First eight base-2 Halton values: 0.5, 0.25, 0.75, 0.125, 0.625, …
/// assert!((halton(1, 2) - 0.5).abs() < 1e-6);
/// assert!((halton(2, 2) - 0.25).abs() < 1e-6);
/// ```
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

/// Return the `i`-th 2-D Halton sample as `(dx, dy)` offsets in `[0, 1)`.
///
/// - `dx` uses base 2
/// - `dy` uses base 3
///
/// Pass these directly as the sub-pixel jitter when constructing a camera ray.
pub fn halton_2d(i: u32) -> (f32, f32) {
	(halton(i, 2), halton(i, 3))
}
