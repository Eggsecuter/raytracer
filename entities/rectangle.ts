import { Color } from "../primitives/color";
import { Vector2 } from "../primitives/vector2";
import { Entity } from "./entity";

export class Rectangle extends Entity {
	constructor(
		color: Color,
		origin: Vector2,
		private width: number,
		private height: number
	) {
		super(color, origin);
	}

	intersects(point: Vector2): boolean {
		return (
			point.x >= this.origin.x &&
			point.x <= this.origin.x + this.width &&
			point.y >= this.origin.y &&
			point.y <= this.origin.y + this.height
		);
	}
}
