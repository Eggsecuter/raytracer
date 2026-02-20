import { Color } from "../types/color";
import { Vector2 } from "../types/vector2";
import { Entity } from "./entity";

export class Circle extends Entity {
	constructor (
		color: Color,
		origin: Vector2,
		private radius: number
	) {
		super(color, origin);
	}

	intersects(point: Vector2): boolean {
		const distance = point.subtract(this.origin).length;

		return distance <= this.radius;
	}
}
