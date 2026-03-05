import { Scene } from "./render/scene";
import { Color } from "./primitives/color";
import { Sphere } from "./entities/sphere";
import { Vector3 } from "./primitives/vector3";
import { Transform } from "./primitives/transform";
import { Camera } from "./render/camera";
import { GlobalLight } from "./light/global-light";

const camera = new Camera();
const scene = new Scene(camera, 1000, 1000);

const head = new Sphere(
	Color.BLUE,
	new Transform(new Vector3(0, 0, 5.0)),
	1.0
);

const rightEye = new Sphere(
	Color.WHITE,
	new Transform(new Vector3(0.3, 0.3, 4.1)),
	0.2
);

const rightPupil = new Sphere(
	Color.GREEN,
	new Transform(new Vector3(0.3, 0.25, 3.9)),
	0.05
);

const leftEye = new Sphere(
	Color.WHITE,
	new Transform(new Vector3(-0.3, 0.3, 4.1)),
	0.2
);

const leftPupil = new Sphere(
	Color.GREEN,
	new Transform(new Vector3(-0.3, 0.25, 3.9)),
	0.05
);

const nose = new Sphere(
	Color.RED,
	new Transform(new Vector3(0, -0.25, 4)),
	0.2
);

const other = new Sphere(
	Color.GREEN,
	new Transform(new Vector3(-2, 3, 7)),
	0.5
)

scene.entities.push(
	head,
	rightEye,
	rightPupil,
	leftEye,
	leftPupil,
	nose,
	other
);

scene.globalLights.push(
	new GlobalLight(
		new Color(0.5, 0.5, 0.4),
		new Vector3(-5, 2, 0)
	)
);

const start = Date.now();
scene.render();

console.debug(`Rendered in ${Date.now() - start}ms`);
