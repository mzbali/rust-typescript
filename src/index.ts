interface Area {
    area: () => number
}
class Rectangle implements Area {
    constructor(
        public x: number,
        public y: number,
        public width: number,
        public height: number,
    ) { }
    area() {
        return this.width * this.height;
    }
}
class Circle {
    constructor(
        public x: number,
        public y: number,
        public radius: number,
    ) { }
    area() {
        return this.radius * this.radius * Math.PI
    }
}
