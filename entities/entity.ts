import { Color } from "../primitives/color";
import { Ray } from "../primitives/ray";
import { RayHit } from "../primitives/ray-hit";
import { Vector } from "../primitives/vector";

export abstract class Entity<TVector extends Vector> {
	constructor (
		public color: Color,
		public origin: TVector
	) {}

	abstract intersect(ray: Ray<TVector>): RayHit<TVector> | null;

	move(delta: TVector) {
		this.origin = this.origin.add(delta) as TVector;
	}
}
