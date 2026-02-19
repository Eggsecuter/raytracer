export class Vector2 {
	get length() {
		return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2))
	}

	constructor (
		public x: number,
		public y: number
	) {}

	add(other: Vector2): Vector2 {
		return new Vector2(this.x + other.x, this.y + other.y);
	}

	subtract(other: Vector2): Vector2 {
		return new Vector2(this.x - other.x, this.y - other.y);
	}

	multiplyScalar(factor: number): Vector2 {
		return new Vector2(this.x * factor, this.y * factor);
	}

	clone(): Vector2 {
		return new Vector2(this.x, this.y);
	}

	toString(): string {
		return `[${this.x}, ${this.y}]`;
	}
}
