import { Color } from "../types/color";
import { Vector2 } from "../types/vector2";
import { IObject } from "./object.interface";

export class Rectangle implements IObject {
	constructor(
		private origin: Vector2,
		private width: number,
		private height: number,
		private color: Color
	) {}

	getColor(): Color {
		return this.color;
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
