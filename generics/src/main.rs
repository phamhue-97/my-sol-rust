use std::cmp::PartialEq;
use std::ops::Add;
use std::fmt::Display;

struct Sequence3<T> {
    first: T,
    second: T,
    third: T,
}
// Read this as: for any type `T`, implement for `Sequence<T>` ...
impl<T: PartialEq> Sequence3<T> where T: Copy + Add<Output = T> {
    pub fn new(first: T, second: T, third: T) -> Self {
        Self { first, second, third }
    }
    pub fn all_same(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
    pub fn sum(&self) -> T {
        self.first + self.second + self.third
    }
}

struct MyStruct<A, B> {
    a: A,
    b: B,
}

enum MyEnum<A, B> {
    A(A),
    B(B),
}

fn say_hello<T: Display>(value: &T) {
    println!("Hello, {value}!");
}

fn f<A, B>(a: A, b: B) where A == B {
    // body
}

fn main() {
    let s = Sequence3 { first: 1, second: 2, third: 3 };
    println!("{}", s.all_same()); // false
    println!("{}", s.sum());

    let s = MyStruct { a: 10, b: "Hello" };

    // We have to specify the type of the `MyEnum::A` variant here because Rust does not have
    // information to infer it.
    let e = MyEnum::<i32, _>::B("Hello");

    say_hello(&true); // Hello, true!
    say_hello(&String::from("World")); // Hello, World!
    say_hello(&1337); // Hello, 1337!
}
