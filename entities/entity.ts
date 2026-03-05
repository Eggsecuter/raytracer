import { Color } from "../primitives/color";
import { Ray } from "../primitives/ray";
import { RayHit } from "../primitives/ray-hit";
import { Transform } from "../primitives/transform";

export abstract class Entity {
	constructor (
		public color: Color,
		public transform: Transform
	) {}

	abstract intersect(ray: Ray): RayHit | null;
}
