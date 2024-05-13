fn length_of_string(value: &String) -> usize {
    value.len()
}
fn append_world(value: &mut String) {
    value.push_str(", World!");
}
fn main() {
    let s1 = String::from("Hey there!");
    // v - Take a reference to `s1`.
    let len = length_of_string(&s1);
    println!("The length of {s1:?} is {len}.");

    let mut s1 = String::from("Hello");
    append_world(&mut s1);
    println!("The value is now {s1:?}.");

    let s1 = String::from("Hey there!");
    let r1 = &s1;
    let r2 = &s1;
    // The immutable references `r1` and `r2` happily co-exist here.
    let len = length_of_string(r1);
    println!("The length of {r2:?} is {len}.");

    let mut items = vec![String::from("first")];
    for item in items {
        items.
        print!("{item}");
        if item != "c" {
            items.push(String::from("c"));
        }
    }
}
