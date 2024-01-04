fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Need a filename as argument");
    let numbers = std::fs::read_to_string(filename).expect("Failed to read the file to string");

    numbers.lines().for_each(|line| {
        let num = line.parse::<usize>().ok();
        if let Some(value) = num {
            println!("{}", value);
        } else {
            println!("Not a number")
        }
    })
}
