import { Primitive } from "./primitive.interface";
import { Vector3 } from "./vector3";

export class RayHit implements Primitive {
	constructor (
		public distance: number,
		public point: Vector3,
		public normal: Vector3
	) {}

	clone(): RayHit {
		return new RayHit(this.distance, this.point.clone(), this.normal.clone());
	}

	toString(): string {
		return `RayHit[${this.distance}, ${this.point.toString()}, ${this.normal.toString()}]`;
	}
}
