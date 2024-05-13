// Learning about unsafe Rust is out of scope so ignore this function.
fn print_string_stack_data(value: &String) {
    let ptr = value as *const _ as *const usize;
    println!("pointer  {0:16} 0x{0:016X}", unsafe { *ptr });
    println!("capacity {0:16} 0x{0:016X}", unsafe { *ptr.offset(1) });
    println!("length   {0:16} 0x{0:016X}", unsafe { *ptr.offset(2) });
}
fn main() {
    // Print the stack-size of a String.
    println!("The size of a `String` is {}", std::mem::size_of::<String>());
    // Create a String with a capacity of 4.
    let mut hello = String::with_capacity(4);
    // Print how the String is represented on the stack.
    print_string_stack_data(&hello);
    // Append the text "Hello!" to the (currently empty) String.
    hello.push_str("Hello!");
    // The capacity and length should have changed, and maybe the pointer.
    print_string_stack_data(&hello);

    println!();
    let a = String::from("a");
    let b = a.clone();
    println!("a = {a}");

    println!();
    struct A;

    impl Drop for A {
        fn drop(&mut self) {
            print!("a");
        }
    }
    
    struct B(A);
    
    impl Drop for B {
        fn drop(&mut self) {
            print!("b");
        }
    }
    let _ = A;
    let _ = B(A);

    let x = String::from("purple");
    let y = match String::as_str(&x) {
    "red" => "red",
    _ => "not red",
    };
    println!("{x} is {y}");
}
