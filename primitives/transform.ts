import { Primitive } from "./primitive.interface";
import { Quaternion } from "./quaternion";
import { Vector3 } from "./vector3";

export class Transform implements Primitive {
	constructor (
		public position: Vector3 = Vector3.ZERO,
		public rotation: Quaternion = Quaternion.IDENTITY
	) {}

	move(delta: Vector3) {
		this.position = this.position.add(delta);
	}

	rotate(delta: Vector3 | Quaternion) {
		if (delta instanceof Vector3) {
			delta = Quaternion.fromEuler(delta);
		}

		this.rotation = this.rotation.multiply(delta).normalize();
	}

	clone(): Transform {
		return new Transform(this.position.clone(), this.rotation.clone());
	}

	toString(): string {
		return `Transform[${this.position.toString()}, ${this.rotation.toString()}]`;
	}
}
