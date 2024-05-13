// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
fn main() {
    struct Plant {
        flowering: bool,
        mass: f64,
    }
    let Plant { flowering, mass };

    let plant = Plant {
        flowering: true,
        mass: 10.0,
    };
    let Plant { flowering, mass } = plant;

    enum Meal {
        FishAndChips { chip_proportion: f64 },
        Hamburger { vegetarian: bool },
    }

    let meal = Meal::Hamburger {
        vegetarian: true,
    };
    if let Meal::Hamburger { vegetarian: true } = meal {
        println!("I had a vegetarian hamburger!");
    }    

    for n in 0..=5 {
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            // or-pattern
            3 | 4 => println!("It was a bit more than two!"),
            // match guard
            high if high >= 5 => println!("It was a high number!"),
            // a pattern consisting only of the name `other`
            other => println!("It was {other}!"),
        }
    }

    match meal {
        Meal::FishAndChips { chip_proportion } => {
            if chip_proportion > 0.5 {
                println!("I had some fish and plenty of chips!");
            } else if chip_proportion < 0.5 {
                println!("I had plenty of fish and some chips!");
            } else {
                println!("I had fish and chips!");
            }
        }
        Meal::Hamburger { vegetarian } => {
            if vegetarian {
                println!("I had a vegetarian hamburger!");
            } else {
                println!("I had a meaty hamburger!");
            }
        }
    }

    match meal {
        Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
            println!("I had some fish and plenty of chips!");
        }
        Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
            println!("I had plenty of fish and some chips!");
        }
        Meal::FishAndChips { chip_proportion } => {
            println!("I had fish and chips!");
        }
        Meal::Hamburger { vegetarian: true } => {
            println!("I had a vegetarian hamburger!");
        }
        Meal::Hamburger { vegetarian: false } => {
            println!("I had a meaty hamburger!");
        }
    }

    let mut meal2 = Meal::FishAndChips {
        chip_proportion: 0.6,
    };
    while let Meal::FishAndChips { chip_proportion } = meal2 {
        println!("Having fish and chips with chip proportion {:.2}...", chip_proportion);
        if chip_proportion > 0.3 {
            // Order a meal with less chips.
            meal2 = Meal::FishAndChips {
                chip_proportion: chip_proportion - 0.2,
            }
        } else {
            // Too fishy, let's get a hamburger next.
            meal2 = Meal::Hamburger { vegetarian: true }
        }
    }
    println!("I'm so done with fish and chips!");

    let tuples: [(usize, &'static str); 3] = [
        (1, "red"),
        (2, "white"),
        (3, "blue"),
    ];

    for (numbering, color) in tuples {
        println!("Color #{numbering} is {color}!");
    }

    let colors = [
        "red",
        "white",
        "blue",
    ];

    for (index, color) in colors.into_iter().enumerate() {
        let numbering = index + 1;
        println!("Color #{numbering} is {color}!");
    }

    for n in 1..=3 {
        match n {
        0 | 1 => print!("a"),
        _ => print!("b"),
        other if other > 2 => print!("c"),
        }
    }

    enum Key { Up, Down, Left, Right };

    match Key::Left {
        Up => println!("Jumping"),
        Down => println!("Ducking!"),
        Left => println!("Sliding Left!"),
        Right => println!("Sliding Right!"),
    }
}
