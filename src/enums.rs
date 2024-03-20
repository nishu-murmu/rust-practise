#![allow(unused_variables)]
#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}
impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    };
}

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
    items.push(Item::String("hello world".into()));
}

pub fn enums_functionality() {
    let foo = Color::Green;
    let bar = foo.is_green();
    println!("{}", bar);
    print_color(Color::Red);

    // enums core functionality which makes it different from typescript
    let mut items: Vec<Item> = vec![];
    append(&mut items);
}
