import { Entity } from "../entities/entity";
import { Color } from "../primitives/color";
import { Ray } from "../primitives/ray";
import { Vector } from "../primitives/vector";
import { Vector2 } from "../primitives/vector2";
import { Vector3 } from "../primitives/vector3";

export class Scene {
	private readonly backgroundColor = Color.BLACK;

	constructor (
		private width: number,
		private height: number
	) {
		const canvas: HTMLCanvasElement = document.createElement('canvas');
		canvas.width = this.width;
		canvas.height = this.height;
		document.body.appendChild(canvas);

		this.context = canvas.getContext('2d', {
			willReadFrequently: false
		});
	}

	private context: CanvasRenderingContext2D;
	private entities: Entity<Vector>[] = [];

	addEntity(...entity: Entity<Vector>[]) {
		this.entities.push(...entity);
	}

	render() {
		this.context.clearRect(0, 0, this.width, this.height);
		const image = this.context.createImageData(this.width, this.height);

		for (let y = 0; y < this.height; y++) {
			for (let x = 0; x < this.width; x++) {
				const color = this.getPixelColor(new Vector3(x, y, 0));

				// a pixel has 4 values (rgba)
				const pixelDataLength = 4;
				const pixelIndex = pixelDataLength * (this.width * y + x);

				image.data[pixelIndex + 0] = 255 * color.red;
				image.data[pixelIndex + 1] = 255 * color.green;
				image.data[pixelIndex + 2] = 255 * color.blue;
				image.data[pixelIndex + 3] = 255;
			}
		}

		this.context.putImageData(image, 0, 0);
	}

	renderContinuously(onUpdate: (deltaTime: number) => void) {
		let lastRender = 0;

		function update(currentTime: number) {
			const deltaTime = (currentTime - lastRender) / 1000;
			lastRender = currentTime;

			onUpdate(deltaTime);

			this.render();

			requestAnimationFrame(update.bind(this));
		}

		requestAnimationFrame(update.bind(this));
	}

	private getPixelColor(point: Vector3) {
		const ray3d = new Ray(point, Vector3.FORWARD);
		const ray2d = new Ray(new Vector2(point.x, point.y), Vector2.ZERO);

		let smallestDistance: number = null;
		let currentColor = this.backgroundColor;

		for (const entity of this.entities) {
			const intersection = entity.intersect(
				entity.origin instanceof Vector2 ? ray2d : ray3d
			);

			// newer entities get rendered on top of older
			if (intersection && (smallestDistance == null || intersection.distance <= smallestDistance)) {
				smallestDistance = intersection.distance;
				currentColor = entity.color;
			}
		}

		return currentColor;
	}
}
