import { Color } from "../primitives/color";
import { Ray } from "../primitives/ray";
import { RayHit } from "../primitives/ray-hit";
import { Transform } from "../primitives/transform";
import { Vector } from "../primitives/vector";

export abstract class Entity<TVector extends Vector> {
	constructor (
		public color: Color,
		public transform: Transform<TVector>
	) {}

	abstract intersect(ray: Ray<TVector>): RayHit<TVector> | null;
}
