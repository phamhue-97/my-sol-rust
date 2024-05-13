use std::fmt::Display;

struct MyStruct<A, B> {
    a: A,
    b: B,
}

impl<A: Display, B: Display> MyStruct<A, B> {
    fn print(&self) {
        println!("a: {}, b: {}", self.a, self.b);
    }
}

fn main() {
    println!("{:?}", "Hello");
    println!("{:?}", vec!["Hello", "World"]);

    let rs = MyStruct {
        a: "dev",
        b: "rust"
    };
    rs.print();

    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    let mick = Person {
        name: "Mick".to_string(),
        age: 30
    };

    println!("{:?}", mick);
}
