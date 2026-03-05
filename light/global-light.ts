import { Color } from "../primitives/color";
import { RayHit } from "../primitives/ray-hit";
import { Vector3 } from "../primitives/vector3";
import { Light } from "./light.interface";

export class GlobalLight implements Light {
	constructor (
		private color: Color,
		private direction: Vector3
	) {}

	// TODO distance matters!
	calculateColor(rayHit: RayHit) {
		const intensity = this.direction.scalarProduct(rayHit.normal);

		return this.color.multiply(Color.fromGrayscale(intensity));
	}
}
