enum Vehicle2 {
    Car { wheel_count: u32 },
    Motorcycle { wheel_count: u32 },
}
impl Vehicle2 {
    fn new_car() -> Self {
        Self::Car { wheel_count: 4 }
    } 
    fn new_motorcycle() -> Self {
        Self::Motorcycle { wheel_count: 2 }
    }
    fn wheel_count(&self) -> u32 {
        match self {
            Vehicle2::Car { wheel_count, .. } => *wheel_count,
            Vehicle2::Motorcycle { wheel_count, .. } => *wheel_count,
        }
    }
}

fn main() {
    let car =  Vehicle2::new_car();
    println!("{}", car.wheel_count());

    let motorcycle =  Vehicle2::new_motorcycle();
    println!("{}", motorcycle.wheel_count());
}
