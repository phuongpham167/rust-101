fn main() {
    const PI: f64  = 3.14;
    println!("Hello, world!");
    let x = 5; // by default is immutable
    let mut y = 6; //add mut to define y with mutable
    println!("before change y: (x + y)*pi = ({} + {})*pi = {}", x, y, (f64::from(x + y)* PI)); //using f64::from(a) to change a => f64 data type
    y = 5;
    println!("after change y: (x + y)*pi = ({} + {})*pi = {}", x, y, (f64::from(x + y)* PI));
}
