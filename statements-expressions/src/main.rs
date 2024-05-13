fn main() {
    println!("{}", 1 + 1);

    let mut brownies_eaten: u8 = 0;
    let quantifier = if brownies_eaten == 0 {
        "no brownies"
    } else {
        "at least one brownie"
    };
    println!("I had {quantifier} today.");

    brownies_eaten = 8;
    let quantifier2 = match brownies_eaten {
        0 => "no brownies",
        1 => "a brownie",
        _ => "multiple brownies",
    };
    println!("I had {quantifier2} today.");

    let mut i = 0;
    let x = loop {
        if i > 7 {
            break i;
        }
        i += i*2 + 1;
    };
    println!("{x}");
}
