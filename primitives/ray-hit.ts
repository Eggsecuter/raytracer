import { Primitive } from "./primitive.interface";
import { Vector } from "./vector";

export class RayHit<TVector extends Vector> implements Primitive {
	constructor (
		public distance: number,
		public point: TVector,
		public normal: TVector
	) {}

	clone(): RayHit<TVector> {
		return new RayHit(this.distance, this.point.clone(), this.normal.clone()) as RayHit<TVector>;
	}

	toString(): string {
		throw new Error("Method not implemented.");
	}
}
