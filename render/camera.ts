import { Transform } from "../primitives/transform";
import { Vector3 } from "../primitives/vector3";

export class Camera {
	constructor (
		public transform: Transform<Vector3>
	) {}
}
