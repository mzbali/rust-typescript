type Custom = {
    age: number,
    name: string
}
type item = number | string | Custom

function append(item: Item[]) {
    item.push("Hello, fem");
}

const items = [];

console.log(items)
append(items);
console.log(items)

const numbers: number[] = [];

console.log(numbers);
append(numbers);
console.log(numbers);
