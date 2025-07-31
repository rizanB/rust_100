// fn
fn add(x: i32, y: i32) -> i32 {
   return x + y;
}

fn greet(name: &str){
    println!("hello {}", name);
}

// simple macro
macro_rules! food_of_the_day {
    ($food:expr) => {
        println!("chef. macro says: food of the day is {}!", $food);
    };
}

pub fn main() {
    println!("from basics.rs file");

    // println!("hello rust");

    // variables
    let x = 5;
    let y = 10;

    println!("x is {}, y is {}", x, y);

    // constant
    const PI: f64 = 3.1415;
    println!("pi = {}", PI);

    println!("{}", add(2, 2));

    // ownership and borrow
    let name = String::from("Rust");
    greet(&name);
    println!("from main: {}", name);

    // vectors 
    let mut numbers = vec![1,2,3];
    numbers.push(4);
    numbers.reverse();
    println!("reversed numbers vector is {:?}", numbers);

    // macro
    food_of_the_day!("pizza");
}
