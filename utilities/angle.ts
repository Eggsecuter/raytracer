export class Angle {
	static readonly FULL_RADIANT = Math.PI * 2;
	static readonly FULL_DEGREE = 360;

	static toDegree(radiant: number) {
		return radiant / this.FULL_RADIANT * this.FULL_DEGREE;
	}

	static toRadiant(degree: number) {
		return degree / this.FULL_DEGREE * this.FULL_RADIANT;
	}
}
