fn main() {
    let _8bit: i8 = -128;
    let _8bit: u8 = 127;
    let _32bit: f32 = -100.458;
    let _64bit: f64 = -100.456;
    let _array: [u32; 3] = [1, 2, 3];
    let _slice_arr: &[u8] = &[1, 2, 3];
    let _tuple: (bool, u32, f64) = (true, 2, 3.0);
    let _say: &str = "Hello Dev";

    struct MyStruct {
        should_do_groceries: bool,
        birth_year: u32,
        height_in_meters: f64,
    }
    let my_struct = MyStruct {
        should_do_groceries: true,
        birth_year: 1992,
        height_in_meters: 1.79,
    };

    const NORTH: u32 = 0;
    const EAST: u32 = 1;
    const SOUTH: u32 = 2;
    const WEST: u32 = 3;
    enum CardinalDirection {
        North,
        East,
        South,
        West,
    }
    let d = CardinalDirection::East;

    if let CardinalDirection::East = d {
        println!("We are going east!");
    } else {
        println!("We are not going east but in some other direction!");
    }

    enum Shape {
        Square {
            side: f64
        },
        Rectangle {
            width: f64,
            height: f64,
        },
        Circle {
            radius: f64,
        },
    }
    let s = Shape::Rectangle {
        width: 800.0,
        height: 60.0,
    };

    match s {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        },
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        },
        Shape::Circle { radius } => {
            println!("A circle of radius {} and diameter {}!", radius, radius * 2.0);
        }
    }
    println!("{}", _tuple.0);
    println!("{}", _tuple.1);
    println!("{}", _tuple.2);
    println!("{}", _say);
}
