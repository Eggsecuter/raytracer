import { Color } from "./types/color";

class Main {
	constructor (
		private width: number,
		private height: number
	) {}

	private context: CanvasRenderingContext2D;

	render() {
		const canvas: HTMLCanvasElement = document.createElement('canvas');
		canvas.width = this.width;
		canvas.height = this.height;
		document.body.appendChild(canvas);

		this.context = canvas.getContext('2d', {
			willReadFrequently: false
		});

		const backgroundColor = Color.GREEN;
		const image = this.context.createImageData(this.width, this.height);

		// a pixel has 4 values (rgba)
		for (let pixelIndex = 0; pixelIndex < image.data.length; pixelIndex += 4) {
			image.data[pixelIndex + 0] = backgroundColor.red;
			image.data[pixelIndex + 1] = backgroundColor.green;
			image.data[pixelIndex + 2] = backgroundColor.blue;
			image.data[pixelIndex + 3] = backgroundColor.alpha;
		}

		this.context.putImageData(image, 0, 0);
	}
}

const main = new Main(1080, 1080);
main.render();
