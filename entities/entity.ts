import { Color } from "../primitives/color";
import { Vector2 } from "../primitives/vector2";

export abstract class Entity {
	constructor (
		public color: Color,
		protected origin: Vector2
	) {}

	abstract intersects(point: Vector2): boolean;

	move(delta: Vector2) {
		this.origin = this.origin.add(delta);
	}
}
