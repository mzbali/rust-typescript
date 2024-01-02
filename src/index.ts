import fs from "fs";

let list = [1,2,3];

list = list.map(l=> l+1)
console.log(list);

const data = fs.readFileSync("../lines") ;

console.log(data)
