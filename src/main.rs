#[derive(Debug)]
struct Item {
    count: isize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn main() {
    let mut item = Item { count: 3 };
    println!("{:?}", item);
    add_one(&mut item);
    println!("{:?}", item);
}
