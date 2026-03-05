import { Primitive } from "./primitive.interface";
import { Vector3 } from "./vector3";

export class Ray implements Primitive {
	constructor (
		public origin: Vector3,
		public direction: Vector3
	) {}

	clone(): Ray {
		return new Ray(this.origin.clone(), this.direction.clone());
	}

	toString(): string {
		return `Ray[${this.origin.toString()}, ${this.direction.toString()}]`;
	}
}
