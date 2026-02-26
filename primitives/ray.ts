import { Primitive } from "./primitive.interface";
import { Vector } from "./vector";

export class Ray<TVector extends Vector> implements Primitive {
	constructor (
		public origin: TVector,
		public direction: TVector
	) {}

	clone(): Ray<TVector> {
		return new Ray(this.origin.clone(), this.direction.clone()) as Ray<TVector>;
	}

	toString(): string {
		return `Ray[${this.origin.toString()}, ${this.direction.toString()}]`;
	}
}
