import { Color } from "../types/color";
import { Vector2 } from "../types/vector2";

export interface IObject {
	getColor(): Color;
	intersects(point: Vector2): boolean;
}
