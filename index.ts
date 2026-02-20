import { Circle } from "./entities/circle";
import { Rectangle } from "./entities/rectangle";
import { Scene } from "./render/scene";
import { Color } from "./types/color";
import { Vector2 } from "./types/vector2";

const scene = new Scene(1000, 1000);

const blueCircle = new Circle(Color.BLUE, new Vector2(500, 500), 250);
const redRectangle = new Rectangle(Color.RED, new Vector2(250, 400), 300, 450);

scene.addEntity(blueCircle);
scene.addEntity(redRectangle);

const start = Date.now();
scene.render();

console.debug(`Rendered in ${Date.now() - start}ms`);
