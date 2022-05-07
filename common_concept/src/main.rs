fn main() {
    println!("Hello, world!");
    let x = 5; // by default is immutable
    let mut y = 6; //add mut to define y with mutable
    println!("before change y: x + y = {} + {} = {}", x, y, (x + y));
    y = 5;
    println!("after change y: x + y = {} + {} = {}", x, y, (x + y));
}
