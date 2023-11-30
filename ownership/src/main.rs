fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 5;
    assert_eq!(x, y);
    printing();
}

fn printing() {
    let mut s: String = String::from("hello");
    s.push_str(", world!");
    let v = s;
    println!("{}", v);
}
