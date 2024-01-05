#[derive(Debug)]
struct Item {
    count: isize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    let mut items = vec![{ Item { count: 1 } }];
    let first = items.first_mut();
    println!("{:?}", first);
    println!("{:?}", first);
}
