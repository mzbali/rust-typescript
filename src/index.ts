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

    toString(){
        return `(${this.x}, ${this.y}): ${this.width} x ${this.height}`
    }
}
class Circle {
    constructor(
        public x: number,
        public y: number,
        public radius: number,
    ) { }
    area() {
        return this.radius * this.radius * Math.PI;
    }
}
console.log(`Rectangle ${new Rectangle(0,0,10,10)}`)
