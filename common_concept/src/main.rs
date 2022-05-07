use std::io;

fn main() {
    const PI: f64  = 3.14;
    println!("Hello, world!");
    let x = 5; // by default is immutable
    let mut y = 6; //add mut to define y with mutable
    println!("before change y: (x + y)*pi = ({} + {})*pi = {}", x, y, (f64::from(x + y)* PI)); //using f64::from(a) to change a => f64 data type
    y = 5;
    println!("after change y: (x + y)*pi = ({} + {})*pi = {}", x, y, (f64::from(x + y)* PI));

    {
        // x = 10; failed because x is immutable
        let x = x + y; //shadow a immutable varibale with the same name using let keyword
        println!("new x : {}", x);
        println!("after change x: (x + y)*pi = ({} + {})*pi = {}", x, y, (f64::from(x + y)* PI));
    }

    println!("old x: {}", x);

    let spaces = "dddd";
    // spaces = spaces.len(); // error because the shadow only change value cannot change varibale's type ( form string to int)

    println!("{}", spaces);

    // let f: bool = false;
    // let t = true;
    let cat = "😻";
    println!("{}", cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    let arr: [f32; 5]= [1.1, 2.2, 3.3, 4.4, 5.5];
    println!("{}", arr[3]);

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = arr[index];
    println!("{} {}", index, element)
}
