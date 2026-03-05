import { Angle } from "../utilities/angle";
import { Ray } from "../primitives/ray";
import { Transform } from "../primitives/transform";
import { Vector3 } from "../primitives/vector3";

export class Camera {
	constructor (
		public transform: Transform = new Transform(),
		public fov: number = Angle.toRadiant(60),
	) {}

	getRay(x: number, y: number, width: number, height: number): Ray {
		const aspect = width / height;

		const px = (2 * (x + 0.5) / width - 1) * Math.tan(this.fov / 2) * aspect;
		const py = (1 - 2 * (y + 0.5) / height) * Math.tan(this.fov / 2);

		const direction = new Vector3(px, py, 1).normalize();
		const rotatedDir = this.transform.rotation.rotateVector(direction).normalize();

		return new Ray(this.transform.position, rotatedDir);
	}
}
