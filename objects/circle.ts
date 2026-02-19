import { Color } from "../types/color";
import { Vector2 } from "../types/vector2";
import { IObject } from "./object.interface";

export class Circle implements IObject {
	constructor (
		private origin: Vector2,
		private radius: number,
		private color: Color
	) {}

	getColor(): Color {
		return this.color;
	}

	intersects(point: Vector2): boolean {
		const distance = point.subtract(this.origin).length;

		return distance <= this.radius;
	}
}
