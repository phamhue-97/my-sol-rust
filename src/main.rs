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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err(Error::B)?; // Error: B
    Ok(())
}
