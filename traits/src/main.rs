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

struct MyType(i32);

impl MyType{
    fn new() -> MyType {
        MyType(0)
    }
    fn get(&self) -> i32 {
        self.0
    }
    fn set(&mut self, val: i32) {
        self.0 = val
    }
}

struct Index ( i32);

trait Joining  {
    // Associated types
    type A;
    type B;
    fn join_to_str(&self, _: &Self::A, _: &Self::B) -> String; 
}

impl Joining for Index  {
    // Define type of associated types
    type A = String;
    type B = String;
    fn join_to_str(&self, name: &Self::A, last_name: &Self::B) -> String {
        format!("{}. {} {}", self.0, name, last_name)
    }
}
fn get_joined_str<J: Joining>(joining: &J, name: &J::A, last_name: &J::B) -> String {
    format!("Person: {}", joining.join_to_str(name, last_name))
}

fn main() {
    println!("{:?}", "Hello");
    println!("{:?}", vec!["Hello", "World"]);

    // let rs = MyStruct {
    //     a: "dev",
    //     b: "rust"
    // };
    // rs.print();

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

    let index = Index(10);
    println!(
        "{}",
        get_joined_str(&index, &"John".to_string(), &"Connor".to_string())
    );

    // Associated Constants
    println!();
    trait ConstantValue {
        const VALUE: i32;
    }
    struct MyStruct;
    
    impl ConstantValue for MyStruct {
        const VALUE: i32 = 10;
    }
    println!("{}", MyStruct::VALUE);

    #[derive(Debug)]
    struct Struct  {
        name: String,
        age:i32,
    }

   let v = Struct {
       name: "John".to_string(),
       age: 20,
   };
   println!("{:?}", v);

   // Supertraits
    // trait Engine {
    //     fn start(&mut self);
    //     fn stop(&mut self);
    //     fn state(&self) -> bool;
    // }
    // trait Transmission {
    //     fn set_gear(&mut self, _: i32);
    //     fn gear(&self) -> i32;
    // }
    // trait Vehicle: Engine+Transmission {
    //     fn wheel_count(&self) -> u32;
    // }
    // trait Car: Vehicle {
    //     fn fuel_type(&self) -> FuelType;
    // }
    // trait Motorcycle: Vehicle {
    //     fn fuel_type(&self) -> FuelType;
    // }
    trait Vehicle {
        fn wheel_count(&self) -> u32;
    }

    #[derive(Default)]
    struct Car;
    impl Vehicle for Car {
        fn wheel_count(&self) -> u32 {
           4
        }
    }
    #[derive(Default)]
    struct Motorcycle;
    impl Vehicle for Motorcycle {
        fn wheel_count(&self) -> u32 {
          2
        }
    }
    fn wheel_count_static<T: Vehicle>(obj: &T) -> u32 {
       obj.wheel_count()
    }
    fn wheel_count_dynamic(obj: &dyn Vehicle) -> u32 {
       obj.wheel_count()
    }

    let car =  Car::default();
    println!("{}", wheel_count_static(&car));

    let motorcycle =  Motorcycle::default();
    println!("{}", wheel_count_static(&motorcycle));
}
