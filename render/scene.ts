import { IObject } from "../objects/object.interface";
import { Color } from "../types/color";
import { Vector2 } from "../types/vector2";

export class Scene {
	private readonly backgroundColor = Color.BLACK;

	constructor (
		private width: number,
		private height: number
	) {}

	private context: CanvasRenderingContext2D;
	private objects: IObject[] = [];

	addObject(object: IObject) {
		// newer objects get rendered on top of older
		this.objects.unshift(object);
	}

	render() {
		const canvas: HTMLCanvasElement = document.createElement('canvas');
		canvas.width = this.width;
		canvas.height = this.height;
		document.body.appendChild(canvas);

		this.context = canvas.getContext('2d', {
			willReadFrequently: false
		});

		const image = this.context.createImageData(this.width, this.height);

		for (let y = 0; y < this.height; y++) {
			for (let x = 0; x < this.width; x++) {
				const color = this.getPixelColor(new Vector2(x, y));

				// a pixel has 4 values (rgba)
				const pixelDataLength = 4;
				const pixelIndex = pixelDataLength * (this.width * y + x);

				image.data[pixelIndex + 0] = color.red;
				image.data[pixelIndex + 1] = color.green;
				image.data[pixelIndex + 2] = color.blue;
				image.data[pixelIndex + 3] = color.alpha;
			}
		}

		this.context.putImageData(image, 0, 0);
	}

	private getPixelColor(point: Vector2) {
		for (const object of this.objects) {
			if (object.intersects(point)) {
				return object.getColor();
			}
		}

		return this.backgroundColor;
	}
}
