use std::thread as my_name;
// declarate the module
mod module1;
mod super_mode;

fn main() {
    println!("{}", module1::get_string());
    println!("{}", super_mode::mod1::get_string());
    println!("{:?}", my_name::current().id());
}
