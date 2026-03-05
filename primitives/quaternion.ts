import { Primitive } from "./primitive.interface";
import { Vector3 } from "./vector3";

/**
 * Quaternion is useful for describing and working with orientation within 3d space.
 * It prevents gimbal lock and allows smooth interpolation between orientations.
 */
export class Quaternion implements Primitive {
	static readonly IDENTITY = new Quaternion(0, 0, 0, 1);

	constructor(
		public x: number,
		public y: number,
		public z: number,
		public w: number
	) {}

	static fromEuler(euler: Vector3): Quaternion {
		const cx = Math.cos(euler.x * 0.5);
		const sx = Math.sin(euler.x * 0.5);
		const cy = Math.cos(euler.y * 0.5);
		const sy = Math.sin(euler.y * 0.5);
		const cz = Math.cos(euler.z * 0.5);
		const sz = Math.sin(euler.z * 0.5);

		return new Quaternion(
			sx * cy * cz + cx * sy * sz,
			cx * sy * cz - sx * cy * sz,
			cx * cy * sz + sx * sy * cz,
			cx * cy * cz - sx * sy * sz
		);
	}

	multiply(q: Quaternion): Quaternion {
		return new Quaternion(
			this.w*q.x + this.x*q.w + this.y*q.z - this.z*q.y,
			this.w*q.y - this.x*q.z + this.y*q.w + this.z*q.x,
			this.w*q.z + this.x*q.y - this.y*q.x + this.z*q.w,
			this.w*q.w - this.x*q.x - this.y*q.y - this.z*q.z
		);
	}

	conjugate(): Quaternion {
		return new Quaternion(-this.x, -this.y, -this.z, this.w);
	}

	normalize(): Quaternion {
		const length = Math.hypot(this.x, this.y, this.z, this.w);

		return new Quaternion(
			this.x / length,
			this.y / length,
			this.z / length,
			this.w / length
		);
	}

	rotateVector(delta: Vector3): Vector3 {
		const deltaQuaternion = new Quaternion(delta.x, delta.y, delta.z, 0);
		const result = this.multiply(deltaQuaternion).multiply(this.conjugate());

		return new Vector3(result.x, result.y, result.z);
	}

	clone(): Quaternion {
		return new Quaternion(this.x, this.y, this.z, this.w);
	}

	toString(): string {
		return `Quaternion[${this.x}, ${this.y}, ${this.z}, ${this.w}]`;
	}
}
