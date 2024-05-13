fn main() {
    // Declarative Macros ! function. Ex println! <=> !
    macro_rules! create_vec {
        ( $( $item:expr ),* ) => {
            {
                let mut result = Vec::new();
                $(
                    result.push($item);
                )*
                result
            }
        }
    }
    let items = create_vec!(1, 2, 3);
    println!("{items:?}");

    // Procedural Macros

    /// Custom Derive Macros
    #[derive(Debug, Clone, Default, Eq, PartialEq)]
    struct MyType {
        name: String,
        items: Vec<i32>,
    }

    let v1 = MyType::default(); // Uses Default to instantiate a default value.
    let v2 = v1.clone(); // Uses Clone to create a clone of v1.
    assert_eq!(&v1, &v2); // Uses Eq to compare the two values for equality.
    println!("{v1:#?}"); // Uses Debug to print the value.

    /// Attribute-Like Macros
    #[my_attr_macro]
    fn x() {}

    #[my_attr_macro]
    const Y: u32 = 1;

    #[my_attr_marco]
    struct Z;

    /// Function-Like Macros. Ex my_fn_macro!(some stuff);
    
}
