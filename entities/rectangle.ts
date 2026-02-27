import { Color } from "../primitives/color";
import { Ray } from "../primitives/ray";
import { RayHit } from "../primitives/ray-hit";
import { Transform } from "../primitives/transform";
import { Vector2 } from "../primitives/vector2";
import { Entity } from "./entity";

export class Rectangle extends Entity<Vector2> {
	constructor(
		color: Color,
		transform: Transform<Vector2>,
		private width: number,
		private height: number
	) {
		super(color, transform);
	}

	intersect(ray: Ray<Vector2>): RayHit<Vector2> | null {
		const halfWidth = this.width / 2;
		const halfHeight = this.height / 2;

		const left = this.transform.position.x - halfWidth;
		const right = this.transform.position.x + halfWidth;
		const top = this.transform.position.y - halfHeight;
		const bottom = this.transform.position.y + halfHeight;

		if (
			ray.origin.x >= left &&
			ray.origin.x <= right &&
			ray.origin.y >= top &&
			ray.origin.y <= bottom
		) {
			return new RayHit(
				0,
				ray.origin.clone(),
				ray.direction.invert()
			);
		}

		return null;
	}
}
