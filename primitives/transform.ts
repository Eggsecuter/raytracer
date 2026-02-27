import { Primitive } from "./primitive.interface";
import { Vector } from "./vector";

export class Transform<TVector extends Vector> implements Primitive {
	constructor (
		public position: TVector,
		public rotation: TVector
	) {}

	move(delta: TVector) {
		this.position = this.position.add(delta) as TVector;
	}

	rotate(delta: TVector) {
		this.rotation = this.rotation.add(delta) as TVector;
	}

	clone(): Transform<TVector> {
		return new Transform(
			this.position.clone(),
			this.rotation.clone()
		) as Transform<TVector>;
	}

	toString(): string {
		return `Transform[${this.position.toString()}, ${this.rotation.toString()}]`;
	}
}
