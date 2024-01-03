fn multiply(num: Option<usize>) -> Option<usize> {
    return Some(num? * 5);
}
fn main() {
    let value = multiply(Some(5));
    if let Some(result) = value {
        println!("{}", result)
    } else {
        println!("Result is none")
    }
}
