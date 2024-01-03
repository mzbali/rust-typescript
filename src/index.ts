function practice(num: number | undefined): number | undefined {
    return num === undefined ? undefined: num * 5 ;
}
console.log(practice(5));
console.log(practice(undefined));

