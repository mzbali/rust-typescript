struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}
fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello, fem!".to_string()));
}
fn main() {
    let mut items = vec![];
    append(&mut items);
    let foo = Item::Number(5);

    match &foo {
        Item::Number(number) => println!("number: {}", number),
        Item::String(str) => println!("string: {}", str),
        Item::MyCustom(custom) if custom.name == "Ricky" => {
            println!("name:{}, age:{}", custom.name, custom.age)
        }
        _ => {}
    }
}
