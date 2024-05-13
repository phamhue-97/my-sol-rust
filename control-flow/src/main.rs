fn main() {
    let should_print = true;
    if should_print {
        println!("Printing!");
    }
    let value = 10;
    if value == 0 {
        println!("Zero!");
    } else if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }

    let mut i = 10;
    loop {
        if i == 0 {
            break;
        }
        println!("{i}...");
        i -= 1;
    }
    println!("Launch!");

    let mut i = 10;
    while i != 0 {
        println!("{i}...");
        i -= 1;
    }
    println!("Launch!");

    for i in (1..=10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}...");
    }
    println!("Launch!");

    for i in 1..=10 {
        println!("{i}...");
    }
    println!("Launch!");
}
