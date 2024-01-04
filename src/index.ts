import fs from "node:fs";

const filename = process.argv[2]
fs.
    readFileSync(filename)
    .toString()
    .split("\n")
    .forEach(line => console.log(line));
