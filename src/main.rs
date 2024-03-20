mod enums;


fn main() {
    let data = vec![1, 2, 3];
    let mut arr = data.iter().map(|x| x + 1);

    // This is what collect does behind the scenes
    // creates a new vector
    let mut new_vec = vec![];

    // go through the iterator one at the time.
    while let Some(x) = arr.next() {
        // push the value into the new vector
        new_vec.push(x);
    }
    println!("{:?}", new_vec);
    let some_value: String = vec!["This", "is", "value"].into_iter().collect();
    println!("{:?}", some_value);

    let what_about_this = vec![1, 2, 3].iter().filter(|x| *x % 2 == 0).count();
    println!("{:?}", what_about_this);
    let map = std::collections::HashMap::from([("foo", 1), ("bar", 2)]);
    map.iter().for_each(|(k, v)| println!("{}: {}", k, v));

    let file = std::fs::read_to_string("test").unwrap();
    file.lines()
        .enumerate()
        .filter(|(id, _)| id % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, x)| println!("{}", x));

    // Enums
    enums::enums_functionality();
}
