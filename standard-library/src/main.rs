use std::ops::{Add, Sub};
use std::path::{Path, PathBuf};
use std::net::UdpSocket;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Read, Result, Write, Error};
use std::{cell::RefCell, fs, io, rc::Rc};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
// https://doc.rust-lang.org/std/vec/struct.Vec.html
fn main() {
    let dynamically_sized_array: Vec<u8> = vec![0, 1, 2, 3];
    println!("{:?}", dynamically_sized_array); // Some(2)
    let dynamically_sized_array = {
        let mut a = Vec::new();
        a.push(0);
        a.push(1);
        a.push(2);
        a.push(3);
        a
    };

    let mut a = vec![1, 2];
    println!("{:?}", a.pop()); // Some(2)
    println!();

    let a = vec![1, 2, 3];
    println!("{}", a[2]); // 3
    println!();

    let mut vec = Vec::new();
    vec.push("Hue".to_string());
    vec.push("Pham".to_string());

    assert!(vec.len() == 2);
    for x in  &vec {
        println!("{x}");
    }
    println!();

    struct UserProfile {
        name: String,
        age: i32,
    }
    let users = vec![
        UserProfile {
            name: "Mick".to_string(),
            age: 30,
        },
        UserProfile {
            name: "Jenny".to_string(),
            age: 28,
        }
    ];
    // Iterate through the array to find "Mick" and print his age.
    println!("{:?}", users.iter().find(|profile| profile.name == "Mick").unwrap().age);
    println!();

    // HashMap<K, V>
    let name_to_profile: std::collections::HashMap<String, UserProfile> = users
        .into_iter()
        .map(|profile| (profile.name.clone(), profile))
        .collect();

    // You can quickly lookup "Mick"'s age
    println!("{:?}", name_to_profile["Mick"].age); // 30
    println!();

    // BTreeMap
    let mut movie_reviews = BTreeMap::new();
    // review some movies.
    movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
    movie_reviews.insert("The Godfather",      "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");
    // check for a specific one.
    if !movie_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.", movie_reviews.len());
    }
    // oops, this review has a lot of spelling mistakes, let's delete it.
    movie_reviews.remove("The Blues Brothers");
    println!("{:?}", movie_reviews); // 30

    // HashSet
    let mut cool_numbers = HashSet::from([2, 10, 8]);
    // Inserting an element that is already present will have no effect.
    cool_numbers.insert(8);
    // Sets implement basic set operations like subtraction.
    println!("{:?}", &cool_numbers - &HashSet::from([2])); // {10, 8}

    // BTreeSet
    // Type inference lets us omit an explicit type signature (which
    // would be `BTreeSet<&str>` in this example).
    let mut books = BTreeSet::new();
    // Add some books.
    books.insert("A Dance With Dragons");
    books.insert("To Kill a Mockingbird");
    books.insert("The Odyssey");
    books.insert("The Great Gatsby");
    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one.",
                 books.len());
    }

    // Remove a book.
    books.remove("The Odyssey");

    // Iterate over everything.
    for book in &books {
        println!("{book}");
    }
    println!();

    // https://doc.rust-lang.org/std/string/struct.String.html
    let mut hello = String::from("Hello, ");

    hello.push('w');
    hello.push_str("orld!");

    println!("{hello}"); // Hello, world!
    println!();

    println!("{:?}", "a".as_bytes()); // [97]
    println!("{:?}", "😊".as_bytes()); // [240, 159, 152, 138]
    println!("{:?}", "😊".as_bytes()[2]); // 152
    println!("{:?}", "😊!".chars().nth(1));

    // Smart point
    // Box<T>
    let b = Box::new(10);
    println!("{}", b); // 10
    let data = create_data(false);
    println!("{data:?}"); // [1, 2, 3, 4, 5]
    println!();

    // Storing Trait Objects
    trait Doubler {
        fn double(&self, value: i32) -> i32;
    }

    struct Simple;

    impl Doubler for Simple {
        fn double(&self, value: i32) -> i32 {
            value * 2
        }
    }

    struct Logged;

    impl Doubler for Logged {
        fn double(&self, input: i32) -> i32 {
            let output = input * 2;
            println!("Doubling {input} gives {output}");
            output
        }
    }

    // The compiler will generate a new function for each type D.
    // That also means that it can perform optimizations taking the implementation of D into account.
    fn static_double<D: Doubler>(doubler: D, value: i32) -> i32 {
        doubler.double(value)
    }
    // The compiler will only generate a single implementation.
    // The function to be invoked is be stored in a virtual method table.
    // The reference `&dyn Doubler` contains a pointer to the instance and to the vtable.
    fn dynamic_double(doubler: &dyn Doubler, value: i32) -> i32 {
        doubler.double(value)
    }
    println!("{}", static_double(Simple, 1));
    println!("{}", static_double(Logged, 3));
    println!("{}", dynamic_double(&Simple, 5));
    println!("{}", dynamic_double(&Logged, 7));
    println!();

    let x: Box<dyn Doubler> = Box::new(Simple);

    struct Light {
        on: bool,
    }

    impl Light {
        fn new() -> Self {
            Light { on: false }
        }

        fn turn_on(&mut self) {
            if !self.on {
                self.on = true;
                println!("Turned on the light.");
            }
        }

        fn turn_off(&mut self) {
            if self.on {
                self.on = false;
                println!("Turned off the light.");
            }
        }
    }

    impl Drop for Light {
        fn drop(&mut self) {
            self.turn_off();
        }
    }

    struct Person {
        light: Light,
    }

    impl Person {
        fn read_book(&self) {
            if self.light.on {
                println!("What a fantastic book!");
            } else {
                println!("It is hard to read without light...");
            }
        }
    }
    let mut light = Light::new();
    light.turn_on();
    let mick = Person { light };
    mick.read_book();
    println!();

    struct PersonPro<'a> {
        light: &'a Light,
    }
    impl<'a> PersonPro<'a> {
        fn read_book(&self) {
            if self.light.on {
                println!("What a fantastic book!");
            } else {
                println!("It is hard to read without light...");
            }
        }
    }
    let mut light = Light::new();
    let mick = PersonPro { light: &light };
    let anna = PersonPro { light: &light };
    mick.read_book();
    anna.read_book();
    println!();

    // Rc<T>
    // Ref<T> / RefMut<T> / RefCell<T>
    struct PersonVipPro {
        light: Rc<Light>,
    }
    impl PersonVipPro {
        fn read_book(&self) {
            if self.light.on {
                println!("What a fantastic book!");
            } else {
                println!("It is hard to read without light...");
            }
        }
    }
    let light = {
        let mut light = Light::new();
        light.turn_on();
        // Place the light in an `Rc<T>`
        Rc::new(light)
    };

    let mick = PersonVipPro {
        // Note that `Light` does not implement `Clone`. We are cloning the
        // smart pointer here, not the value contained within. It is considered
        // good practice to call the clone implementation `Rc::clone` explicitly
        // because it encodes the intent to clone the smart pointer, not the
        // value itself.
        light: Rc::clone(&light),
    };
    let anna = PersonVipPro { light };
    mick.read_book();
    anna.read_book();

    struct PersonVipVipPro {
        // You have wrapped `Light` in a `RefCell` to provide internal mutability.
        light: Rc<RefCell<Light>>,
    }

    impl PersonVipVipPro {
        fn read_book(&self) {
            // You have to call `RefCell::borrow` here to obtain an immutable reference `&Light`.
            if self.light.borrow().on {
                println!("[PersonVipVipPro] What a fantastic book!");
            } else {
                println!("[PersonVipVipPro] It is hard to read without light...");
            }
        }
    }
    let light = {
        let mut light = Light::new();
        light.turn_on();
        // Place the light in an `Rc<T>`
        Rc::new(RefCell::new(light))
    };
    let mick = PersonVipVipPro {
        // Note that `Light` does not implement `Clone`. You are cloning the
        // smart pointer here, not the value contained within. It is considered
        // good practice to call the clone implementation `Rc::clone` explicitly
        // because it encodes the intent to clone the smart pointer, not the
        // value itself.
        light: Rc::clone(&light),
    };
    let anna = PersonVipVipPro {
        light: Rc::clone(&light),
    };

    // The light is on so mick can read.
    mick.read_book();

    // In order to turn off the light, you need a mutable reference `&mut Light`.
    // If any other references were handed out at this point, the program would panic.
    light.borrow_mut().turn_off();

    // The light is off so anna will have trouble reading.
    anna.read_book();

    /** Arguments */
    println!();
    for argument in std::env::args() {
        println!("{argument}");
    }
    println!();

    /** Environment Variables */
    for (key, value) in std::env::vars() {
        println!("{key}={value}");
    }
    println!();

    /** Current Working Directory */
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    println!();

    /** The Filesystem */
    read_fs();
    println!();
    read_dir();
    println!();

    /** Networking */
    // network();
    // client();
    // udp();

    /** Threading */
    // https://doc.rust-lang.org/stable/std/sync/
    threading();
    println!();
    synchronization();

    /** Threading */
    // traits();

    /** Display */
    display();
    // from_str();

    /** Print */
    use_print_path();

    /** Deref */
    deref();

    /** Closures */
    use_twice();

    /** OPS */
    // https://doc.rust-lang.org/std/ops/index.html
    ops();

    /** Cmp */
    // https://doc.rust-lang.org/std/cmp/index.html
    cmp();

    /** Iterator */
    iterator();

    /** Into Iterator */
    into_iterator();

    /** Read & write */
    read_write();

    /** Show error */
    show_error();
}

fn show_error() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    enum Error {
        A,
        B,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Self::A => "a happened",
                Self::B => "b happened",
            })
        }
    }

    // The `Error` trait requires `Debug` and `Display` and it's trait methods are optional.
    impl std::error::Error for Error {}
    Err(Error::B)?; // Error: B
    Ok(())
}

fn read_write() -> Result<()> {
    // The essence of `Read`, there are more methods available with default implementations.
    pub trait Read {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    }

    // The essence of `Write`, there are more methods available with default implementations.
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
    }
    Ok(())
}
struct DoubleRange(usize);

impl IntoIterator for DoubleRange {
    // The type of the item the iterator will produce.
    type Item = usize;

    // The type of the iterator that we will create.
    // For this example you are reusing `std::ops::Range` which implements `Iterator`.
    type IntoIter = std::ops::Range<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0..self.0 * 2
    }
}

fn into_iterator() -> Result<()> {
    let mut iter = DoubleRange(3).into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    Ok(())
}

struct CountDown(usize);

impl Iterator for CountDown {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0)
        }
    }
}
fn iterator() -> Result<()> {
    println!("Start countdown");
    let mut iter = CountDown(3);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    Ok(())
}

fn cmp() -> Result<()> {
    Ok(())
}

fn ops() -> Result<()> {
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {x: self.x + other.x, y: self.y + other.y}
        }
    }

    impl Sub for Point {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {x: self.x - other.x, y: self.y - other.y}
        }
    }

    assert_eq!(Point {x: 3, y: 3}, Point {x: 1, y: 0} + Point {x: 2, y: 3}, "Add is failed");
    assert_eq!(Point {x: -1, y: -3}, Point {x: 1, y: 0} - Point {x: 2, y: 3}, "Sub is failed");
    Ok(())
}

fn twice<F: FnMut() -> &'static str>(mut f: F) {
    println!("{} {}", f(), f());
}
fn once<F: FnOnce() -> String>(f: F) {
    println!("{}", f());
}
fn use_twice() -> Result<()> {
    let mut iter = ["One", "Two"].into_iter();
    twice(|| iter.next().unwrap());
    let one = String::from("One");
    once(move || one);
    Ok(())
}

fn deref() -> Result<()> {
    struct DerefExample<T> {
        value: T
    }

    impl<T> std::ops::Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x, "OK");
    Ok(())
}

fn print_path<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();

    println!("{}", path.display());
}

fn use_print_path() -> Result<()> {
    let a: &'static str = "static_str";
    print_path(a);

    let b: String = String::from("owned_string");
    print_path(b);

    let c: PathBuf = PathBuf::from("owned path");
    print_path(c);

    Ok(())
}

/*fn from_str() {
    enum Color {
        Blue,
        Yellow,
    }
    impl str {
        pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
            FromStr::from_str(self)
        }
    }
    // `str::parse::<T>` is defined if `T` implements `FromStr`.
    let color = "blue".parse::<Color>().unwrap(); // Color::Blue
}*/

fn display() -> Result<()> {
    enum Color {
        Blue,
        Yellow,
    }

    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Color::Blue => "blue",
                Color::Yellow => "yellow",
            })
        }
    }
    println!();
    println!("{}", Color::Yellow); // yellow
    println!();
    Ok(())
}
/*fn traits() -> Result<()> {
    #[derive(Debug)]
    enum DatabaseError {
        DatabaseClosed,
        ProtocolViolation,
    }
    #[derive(Debug)]
    enum ApplicationError {
        Database(DatabaseError),
        IO(std::io::Error),
    }
    impl From<DatabaseError> for ApplicationError {
        fn from(value: DatabaseError) -> Self {
            Self::Database(value)
        }
    }
    impl TryFrom<ApplicationError> for DatabaseError {
        type Error = ApplicationError;

        fn try_from(value: ApplicationError) -> Result<Self, Self::Error> {
            match value {
                ApplicationError::Database(value) => Ok(value),
                _ => Err(value),
            }
        }
    }
    // You can use `Into::into` to convert the `DatabaseError` into an `ApplicationError`.
    let app_err: ApplicationError = DatabaseError::ProtocolViolation.into();

    // You can use `TryInto::try_into` to convert the `ApplicationError` back into a `DatabaseError`.
    let db_err: DatabaseError = app_err.try_into().unwrap();

    // You can not turn an `ApplicationError::IO` variant into a `DatabaseError`, so this conversion fails.
    let still_app_err: ApplicationError = DatabaseError::try_from(ApplicationError::IO(
        std::io::Error::new(std::io::ErrorKind::Other, "Something bad happened!"),
    )).unwrap_err();

    Ok(())
}*/

fn synchronization() -> Result<()> {
    // Create a simple streaming channel.
    let (tx1, rx) = std::sync::mpsc::channel();

    // Copy the producer.
    let tx2 = tx1.clone();

    std::thread::spawn(move || {
        tx1.send(1).unwrap();
    });

    std::thread::spawn(move || {
        tx2.send(2).unwrap();
    });

    // Wait until you receive two messages on the main thread.
    println!("{}", rx.recv().unwrap());
    println!("{}", rx.recv().unwrap());
    Ok(())
}

fn threading() -> Result<()> {
    let handle = std::thread::spawn(|| {
        for i in 0..100 {
            println!("spawned: {i}");
            std::thread::yield_now();
        }
    });
    for i in 0..100 {
        println!("main: {i}");
        std::thread::yield_now();
    }
    handle.join().unwrap();
    Ok(())
}

fn create_data(small: bool) -> Box<[u8]> {
    if small {
        Box::new([1, 2, 3])
    } else {
        Box::new([1, 2, 3, 4, 5])
    }
}

fn read_fs() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);

    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello - world!")?;
    Ok(())
}

fn read_dir() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let prefix = match file_type {
            t if t.is_dir() => "D",
            t if t.is_file() => "F",
            t if t.is_symlink() => "L",
            _ => "?",
        };
        println!("{prefix} {}", entry.path().display());
    }

    Ok(())
}

fn network() -> Result<()> {
    let listener = std::net::TcpListener::bind("0.0.0.0:12345")?;

    // This single threaded server can handle only one incoming connection at a
    // time.
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 4096];
        let count = stream.read(&mut buffer)?;
        stream.write_all(&buffer[0..count])?;
    }
    Ok(())

}

fn client() -> Result<()> {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:12345")?;

    stream.write_all(&[0, 1, 2, 3])?;

    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;
    println!("Received {buffer:?}");

    Ok(())
}

fn udp() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
    Ok(())
}