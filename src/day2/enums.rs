enum Fruit{
    Kiwi, 
    Apple,
    Banana
}

pub fn main(){
    let my_fruit = Fruit::Banana;

    match my_fruit {
        Fruit::Apple => println!("too expensive"),
        Fruit::Kiwi => println!("more expensive than apples"),
        Fruit::Banana => println!("i like this fruit!")        
    }
}