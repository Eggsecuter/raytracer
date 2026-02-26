import { Vector } from "./vector";

export class Vector3 extends Vector {
	static readonly ZERO = new Vector3(0, 0, 0);
	static readonly RIGHT = new Vector3(1, 0, 0);
	static readonly UP = new Vector3(0, 1, 0);
	static readonly LEFT = new Vector3(-1, 0, 0);
	static readonly DOWN = new Vector3(0, -1, 0);
	static readonly FORWARD = new Vector3(0, 0, 1);
	static readonly BACKWARD = new Vector3(0, 0, -1);

	get length() {
		return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2) + Math.pow(this.z, 2));
	}

	constructor (
		public x: number,
		public y: number,
		public z: number
	) {
		super();
	}

	add(other: Vector3): Vector3 {
		return new Vector3(this.x + other.x, this.y + other.y, this.z + other.z);
	}

	subtract(other: Vector3): Vector3 {
		return new Vector3(this.x - other.x, this.y - other.y, this.z - other.z);
	}

	multiply(factor: number): Vector3 {
		return new Vector3(this.x * factor, this.y * factor, this.z * factor);
	}

	scalarProduct(other: Vector3): number {
		return this.x * other.x + this.y * other.y + this.z * other.z;
	}

	invert(): Vector3 {
		return new Vector3(-this.x, -this.y, -this.z);
	}

	normalize(): Vector3 {
		const length = this.length;

		if (length == 0) {
			return new Vector3(0, 0, 0);
		}

		return new Vector3(this.x / length, this.y / length, this.z / length);
	}

	clone(): Vector3 {
		return new Vector3(this.x, this.y, this.z);
	}

	toString(): string {
		return `Vector3[${this.x}, ${this.y}, ${this.z}]`;
	}
}
