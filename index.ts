import { Scene } from "./render/scene";
import { Color } from "./primitives/color";
import { Sphere } from "./entities/sphere";
import { Vector3 } from "./primitives/vector3";

const scene = new Scene(1000, 1000);

const head = new Sphere(Color.BLUE, new Vector3(500, 500, 500), 300);
const rightEye = new Sphere(Color.WHITE, new Vector3(650, 350, 300), 100);
const leftEye = new Sphere(Color.WHITE, new Vector3(350, 350, 300), 100);
const rightPupil = new Sphere(Color.BLACK, new Vector3(650, 340, 230), 50);
const leftPupil = new Sphere(Color.BLACK, new Vector3(350, 340, 230), 50);
const nose = new Sphere(Color.RED, new Vector3(500, 600, 250), 100);
const leftFoot = new Sphere(Color.GREEN, new Vector3(350, 700, 550), 150);
const rightFoot = new Sphere(Color.GREEN, new Vector3(650, 700, 550), 150);

scene.addEntity(
	head,
	rightEye,
	leftEye,
	rightPupil,
	leftPupil,
	nose,
	leftFoot,
	rightFoot
);

const start = Date.now();
scene.render();

console.debug(`Rendered in ${Date.now() - start}ms`);
