// trait ~ abstract class/ interface
trait Animal {
    fn speak(&self) {
        println!("some animal noise")
    }
}

// struct ~ data class
struct Dog;

// implement dog's own method (bark) first, then trait for struct
impl Dog {
    fn bark(&self) {
        println!("woof");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("dog says: woof!");
    }
}

pub fn main() {
    println!("from traits.rs file");

    let bruno = Dog;
    bruno.bark();
    bruno.speak();
}
