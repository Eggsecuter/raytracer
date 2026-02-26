import { Circle } from "./entities/circle";
import { Scene } from "./render/scene";
import { Color } from "./primitives/color";
import { Vector2 } from "./primitives/vector2";

const scene = new Scene(1000, 1000);

const redCircle = new Circle(Color.RED, new Vector2(350, 650), 250);
const blueCircle = new Circle(Color.BLUE, new Vector2(500, 350), 250);
const greenCircle = new Circle(Color.GREEN, new Vector2(650, 650), 250);

scene.addEntity(redCircle);
scene.addEntity(blueCircle);
scene.addEntity(greenCircle);

const start = Date.now();
scene.render();

console.debug(`Rendered in ${Date.now() - start}ms`);
