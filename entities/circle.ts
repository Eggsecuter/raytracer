import { Color } from "../primitives/color";
import { Ray } from "../primitives/ray";
import { RayHit } from "../primitives/ray-hit";
import { Vector2 } from "../primitives/vector2";
import { Entity } from "./entity";

export class Circle extends Entity<Vector2> {
	constructor (
		color: Color,
		origin: Vector2,
		private radius: number
	) {
		super(color, origin);
	}

	intersect(ray: Ray<Vector2>): RayHit<Vector2> | null {
		const distance = ray.origin.subtract(this.origin).length;

		if (distance <= this.radius) {
			return new RayHit(
				0,
				ray.origin.clone(),
				ray.direction.invert()
			);
		}
	}
}
