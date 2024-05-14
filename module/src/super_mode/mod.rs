fn get_string() -> String {
    String::from("Hello world Super!")
}
pub mod mod1 {
    pub fn get_string() -> String {
        super::get_string()
    }
}
