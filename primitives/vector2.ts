import { Vector } from "./vector";

export class Vector2 extends Vector {
	static readonly ZERO = new Vector2(0, 0);
	static readonly RIGHT = new Vector2(1, 0);
	static readonly UP = new Vector2(0, 1);
	static readonly LEFT = new Vector2(-1, 0);
	static readonly DOWN = new Vector2(0, -1);

	get length() {
		return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2));
	}

	constructor (
		public x: number,
		public y: number
	) {
		super();
	}

	add(other: Vector2): Vector2 {
		return new Vector2(this.x + other.x, this.y + other.y);
	}

	subtract(other: Vector2): Vector2 {
		return new Vector2(this.x - other.x, this.y - other.y);
	}

	multiply(factor: number): Vector2 {
		return new Vector2(this.x * factor, this.y * factor);
	}

	scalarProduct(other: Vector2): number {
		return this.x * other.x + this.y * other.y;
	}

	invert(): Vector2 {
		return new Vector2(-this.x, -this.y);
	}

	normalize(): Vector2 {
		const length = this.length;

		if (length == 0) {
			return new Vector2(0, 0);
		}

		return new Vector2(this.x / length, this.y / length);
	}

	clone(): Vector2 {
		return new Vector2(this.x, this.y);
	}

	toString(): string {
		return `Vector2[${this.x}, ${this.y}]`;
	}
}
