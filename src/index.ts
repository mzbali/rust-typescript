import fs from "node:fs";

const filename = process.argv[2]
fs.
    readFileSync(filename)
    .toString()
    .split("\n")
    .forEach((line) => {
        const num = parseInt(line);
        if (isNaN(num)) {
            console.log("Not a number")
        } else {
            console.log(line)
        }
    });
