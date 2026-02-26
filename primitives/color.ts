import { Primitive } from "./primitive.interface";

export class Color implements Primitive {
	static readonly BLACK = new Color(0, 0, 0);
	static readonly WHITE = new Color(1, 1, 1);
	static readonly RED = new Color(1, 0, 0);
	static readonly GREEN = new Color(0, 1, 0);
	static readonly BLUE = new Color(0, 0, 1);

	/**
	 * Create a raytracing color. For further color calculation the range is 0-1.
	 * @param red Red amount range 0-1
	 * @param green Green amount range 0-1
	 * @param blue Blue amount range 0-1
	 */
	constructor (
		public readonly red: number,
		public readonly green: number,
		public readonly blue: number
	) {
		red = this.clamp(red);
		green = this.clamp(green);
		blue = this.clamp(blue);
	}

	add(other: Color): Color {
		return new Color(
			this.red + other.red,
			this.green + other.green,
			this.blue + other.blue
		);
	}

	multiply(other: Color): Color {
		return new Color(
			this.red * other.red,
			this.green * other.green,
			this.blue * other.blue
		);
	}

	clone(): Color {
		return new Color(this.red, this.green, this.blue);
	}

	toString(): string {
		return `Color[${this.red}, ${this.green}, ${this.blue}]`;
	}

	private clamp(amount: number): number {
		if (amount > 1) {
			return 1;
		} else if (amount < 0) {
			return 0;
		}

		return amount;
	}
}
