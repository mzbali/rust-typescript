function practice(nums: number[], index: number): number | undefined {
    return nums[index] ? nums[index] * 5 : index * 5;
}
console.log(practice([1, 2, 3], 0));
console.log(practice([1, 2, 3], 4));

