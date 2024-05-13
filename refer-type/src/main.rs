fn main() {
    struct Config {
        port: u16,
    }
    let config: Config = Config {
        port: 8080
    };
    // Create a reference.
    let config_reference: &Config = &config;
    // In some other part of the program, use the reference.

    let val = 10;
    let r1 = &val;
    let r2 = &val;
    println!("{r1} should be the same as {r2}.");

    // Mutable References
    let mut config2: Config = Config {
        port: 8080
    };
    let config_reference_written: &mut Config = &mut config2;
    config_reference_written.port = 4000;

    println!("Using port {}.", config_reference.port);

    // Observe the original having been modified.
    println!("Using port {}.", config2.port);

    let mut val2 = 10;
    let rs1 = &mut val2;
    // Assign 5 to the value referenced by r1.
    *rs1 = 5;
    println!("{val2}.");

    let val3: i32 = 10;
    let rs2: &i32 = &val;
    // This creates a copy of the value 10.
    let val4: i32 = *rs2;
    //&'lifetime T

    let mut n = 99;
    let mut r = n; // = 99
    r += 1;
    println!("{n}");
}
