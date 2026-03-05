import { Scene } from "./render/scene";
import { Color } from "./primitives/color";
import { Sphere } from "./entities/sphere";
import { Vector3 } from "./primitives/vector3";
import { Transform } from "./primitives/transform";
import { Camera } from "./render/camera";
import { Angle } from "./utilities/angle";
import { Quaternion } from "./primitives/quaternion";

const camera = new Camera();
const scene = new Scene(camera, 1000, 1000);

const head = new Sphere(
	Color.BLUE,
	new Transform(new Vector3(0, 0, 5.0)),
	1.0
);

const rightEye = new Sphere(
	Color.WHITE,
	new Transform(new Vector3(0.3, 0.3, 3.0)),
	0.2
);

const rightPupil = new Sphere(
	Color.GREEN,
	new Transform(new Vector3(0.25, 0.25, 2.5)),
	0.05
);

const leftEye = new Sphere(
	Color.WHITE,
	new Transform(new Vector3(-0.3, 0.3, 3.0)),
	0.2
);

const leftPupil = new Sphere(
	Color.GREEN,
	new Transform(new Vector3(-0.25, 0.25, 2.5)),
	0.05
);

const nose = new Sphere(
	Color.RED,
	new Transform(new Vector3(0, -0.25, 3)),
	0.2
);

scene.addEntity(
	head,
	rightEye,
	rightPupil,
	leftEye,
	leftPupil,
	nose
);

const start = Date.now();
scene.render();

console.debug(`Rendered in ${Date.now() - start}ms`);

let yaw = 0;
let pitch = 0;

document.onkeydown = (event: KeyboardEvent) => {
	const turnAmount = Angle.toRadiant(5);

	switch (event.key) {
		case 'ArrowUp':
			pitch += turnAmount;
			break;

		case 'ArrowDown':
			pitch -= turnAmount;
			break;

		case 'ArrowRight':
			yaw += turnAmount;
			break;

		case 'ArrowLeft':
			yaw -= turnAmount;
			break;
	}

	const qYaw = Quaternion.fromEuler(new Vector3(0, yaw, 0));
	const qPitch = Quaternion.fromEuler(new Vector3(pitch, 0, 0));

	camera.transform.rotation = qYaw.multiply(qPitch).normalize();

	scene.render();
}
