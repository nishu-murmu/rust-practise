fn main() {
    println!("Hello, world!");
    printing();
}

fn printing() {
    println!("printing engineering!");
    let mut s: String = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}
