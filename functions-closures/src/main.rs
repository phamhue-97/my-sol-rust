fn f(n: u32) -> u32 {
    fn g(n: u32) -> u32 {
        n + 1
    }

    g(n * 2)
}

struct X(&'static str);
// An implementation block for the type `X`.
impl X {
    // An associated function.
    fn associated_fn() -> &'static str {
        "I am always the same!"
    }
    // A method.
    // self <=> self: Self
    // &self <=> self: &Self
    // &mut self <=> self: &mut Self
    // https://doc.rust-lang.org/stable/reference/items/associated-items.html#methods
    fn method(self: &Self) -> &'static str {
        self.0
    }
}

fn prefix_print(prefix: String) -> impl Fn(&str) {
    move |suffix| println!("{prefix} {suffix}")
}

fn main() {
    println!("I am the first statement executed by this program!");

    println!("{}", f(3));

    // Call a function associated to the type `X`.
    println!("{}", X::associated_fn());
    // Create an instance of X and call a method on the instance.
    let instance = X("My value depends on an instance of `X`!");
    println!("{}", instance.method());

    // Closures
    let c = |x| {
        x * 2
    };
    println!("{}", c(6));

    let c = |x| x * 2;
    println!("{}", c(6));

    fn c(x: i32) -> i32 {
        x * 2
    }
    println!("{}", c(6));
    
    let mut n = 0;
    let mut c = |x| {
        n += 1;
        x + n - 1
    };
    // n=1 => 2+1-1=2
    // n=2 => 2+2-1=3
    // n=3 => 2+3-1=4
    println!("{}", c(2));
    println!("{}", c(2));
    println!("{}", c(2));

    let a = [1, 2, 3];
    let n: i32 = a.iter().map(|x| x * 2).sum();
    println!("Sum of {:?} after doubling: {}", a, n);

    let pp = prefix_print("Hello,".to_string());

    pp("World!");
}
