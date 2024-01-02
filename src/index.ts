import * as fs from 'node:fs';

const data = fs.readFileSync("lines").toString().split("\n");
data.forEach((line) => {
    console.log(line);
});
