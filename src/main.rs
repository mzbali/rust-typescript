enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Colour {
    fn is_green(&self) -> bool {
        if let Colour::Green = self {
            return true;
        }
        return false;
    }
    fn is_green_parts(&self) -> bool {
        match self {
            Colour::Red => return false,
            Colour::Green => return false,
            Colour::Blue => return true,
            Colour::Yellow => return true,
        }
    }
}
fn print_colour(colour: Colour) {
    match colour {
        Colour::Red => println!("red"),
        Colour::Green => println!("green"),
        Colour::Blue => println!("blue"),
        Colour::Yellow => println!("yellow"),
    }
}

fn main() {
    let foo = Colour::Green;
    foo.is_green();
}
