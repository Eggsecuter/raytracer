import { Entity } from "../entities/entity";
import { Light } from "../light/light.interface";
import { Color } from "../primitives/color";
import { RayHit } from "../primitives/ray-hit";
import { Camera } from "./camera";

export class Scene {
	private readonly backgroundColor = Color.BLACK;
	private readonly ambientLight = Color.fromGrayscale(0.05);

	constructor (
		private camera: Camera,
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

	entities: Entity[] = [];
	globalLights: Light[] = [];

	private context: CanvasRenderingContext2D;

	render() {
		this.context.clearRect(0, 0, this.width, this.height);
		const image = this.context.createImageData(this.width, this.height);

		for (let y = 0; y < this.height; y++) {
			for (let x = 0; x < this.width; x++) {
				const color = this.getPixelColor(x, y);

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

	private getPixelColor(x: number, y: number) {
		const ray = this.camera.getRay(x, y, this.width, this.height);

		let closestEntity: Entity = null;
		let closestIntersection: RayHit = null;

		for (const entity of this.entities) {
			const intersection = entity.intersect(ray);

			// newer entities get rendered on top of older
			if (intersection && (closestIntersection == null || intersection.distance <= closestIntersection.distance)) {
				closestEntity = entity
				closestIntersection = intersection;
			}
		}

		if (!closestEntity || !closestIntersection) {
			return this.backgroundColor;
		}

		let diffuseColor = Color.BLACK;

		for (const globalLight of this.globalLights) {
			diffuseColor = diffuseColor.add(globalLight.calculateColor(closestIntersection));
		}

		return closestEntity.color.multiply(diffuseColor).add(this.ambientLight);
	}
}
