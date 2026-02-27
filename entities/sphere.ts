import { Color } from "../primitives/color";
import { Ray } from "../primitives/ray";
import { RayHit } from "../primitives/ray-hit";
import { Transform } from "../primitives/transform";
import { Vector3 } from "../primitives/vector3";
import { Entity } from "./entity";

export class Sphere extends Entity<Vector3> {
	constructor(
		color: Color,
		transform: Transform<Vector3>,
		public radius: number
	) {
		super(color, transform);
	}

	intersect(ray: Ray<Vector3>): RayHit<Vector3> | null {
		// vector from sphere center to ray origin
		const originToCenter = ray.origin.subtract(this.transform.position);

		// quadratic coefficients
		const directionLengthSquared = ray.direction.scalarProduct(ray.direction);
		const projectionLength = 2 * ray.direction.scalarProduct(originToCenter);
		const centerDistanceSquared = originToCenter.scalarProduct(originToCenter) - this.radius * this.radius;

		const discriminant = projectionLength * projectionLength - 4 * directionLengthSquared * centerDistanceSquared;

		// no intersection
		if (discriminant < 0) {
			return null;
		}

		const sqrtDiscriminant = Math.sqrt(discriminant);
		const denominator = 2 * directionLengthSquared;

		const firstDistance = (-projectionLength - sqrtDiscriminant) / denominator;

		const secondDistance = (-projectionLength + sqrtDiscriminant) / denominator;

		// choose nearest valid hit in front of ray
		let hitDistance = Number.POSITIVE_INFINITY;

		if (firstDistance > 0) {
			hitDistance = firstDistance;
		}

		if (secondDistance > 0 && secondDistance < hitDistance) {
			hitDistance = secondDistance;
		}

		if (!Number.isFinite(hitDistance)) {
			return null;
		}

		const hitPoint = ray.origin.add(
			ray.direction.multiply(hitDistance)
		);

		const surfaceNormal =
			hitPoint.subtract(this.transform.position).normalize();

		return new RayHit(hitDistance, hitPoint, surfaceNormal);
	}
}
