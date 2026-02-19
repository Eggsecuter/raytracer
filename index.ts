import { Circle } from "./objects/circle";
import { Rectangle } from "./objects/rectangle";
import { Scene } from "./render/scene";
import { Color } from "./types/color";
import { Vector2 } from "./types/vector2";

const scene = new Scene(1000, 1000);

const blueCircle = new Circle(new Vector2(500, 500), 250, Color.BLUE);
const redRectangle = new Rectangle(new Vector2(250, 400), 300, 450, Color.RED);

scene.addObject(blueCircle);
scene.addObject(redRectangle);

scene.render();
