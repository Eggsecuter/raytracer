import { Color } from "../primitives/color";
import { RayHit } from "../primitives/ray-hit";

export interface Light {
	calculateColor(rayHit: RayHit): Color;
}
