import { Primitive } from "./primitive.interface";

export abstract class Vector implements Primitive {
	abstract get length(): number;

	abstract add(other: Vector): Vector;
	abstract subtract(other: Vector): Vector;
	abstract multiply(factor: number): Vector;
	abstract scalarProduct(other: Vector): number;
	abstract invert(): Vector;
	abstract normalize(): Vector;

	/**
	 * Calculates the angle between two vectors
	 * @param other Other vector
	 * @returns Angle in radiants
	 */
	angle(other: Vector): number {
		return Math.acos(this.scalarProduct(other) / (this.length * other.length));
	}

	abstract clone(): Vector;
	abstract toString(): string;
}
