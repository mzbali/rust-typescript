fn practice(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}
fn main() {
    let value = practice(vec![1, 2, 3], 1);

    println!("{}", value);
}
