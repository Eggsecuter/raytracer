import { Color } from "../primitives/color";
import { RayHit } from "../primitives/ray-hit";
import { Vector3 } from "../primitives/vector3";
import { Light } from "./light.interface";

export class GlobalLight implements Light {
	constructor (
		private color: Color,
		private position: Vector3
	) {}

	calculateColor(rayHit: RayHit) {
		const intensity = this.position.subtract(rayHit.point).scalarProduct(rayHit.normal);

		return this.color.multiply(Color.fromGrayscale(intensity));
	}
}
