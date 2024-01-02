import * as fs from 'node:fs';

fs.readFileSync("lines").
    toString().
    split("\n").
    filter((_, i) => i % 2 == 0).
    filter((_,i) => i >= 2 && i < 4).
    forEach((line) => {
        console.log(line);
    });
