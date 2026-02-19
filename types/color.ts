export class Color {
	static readonly BLACK = new Color(0, 0, 0, 255);
	static readonly WHITE = new Color(255, 255, 255, 255);
	static readonly RED = new Color(255, 0, 0, 255);
	static readonly GREEN = new Color(0, 255, 0, 255);
	static readonly BLUE = new Color(0, 0, 255, 255);

	constructor (
		public readonly red: number,
		public readonly green: number,
		public readonly blue: number,
		public readonly alpha: number
	) {}
}
