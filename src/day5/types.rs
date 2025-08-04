// Option<T> Result<T, E>, like built-in enums

// Option<type> can be Some(type) or None

// let user1: Option<str> = Some("Rizan");
// let user2: Option<str> = None;


fn find_user_contact_number(name: &str) -> Option<i32> {
    
    if name == "rizan" {
        Some(98989898)
    } else {
        None
    }
}

// Result<T, E> means, it returns T if it succeeds, E if it fails
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("cant divide by zero".to_string())
    } else {
        Ok(a/b)
    }
}

pub fn main() {

    println!("{:?}", find_user_contact_number("rizan"));
    println!("{:?}", find_user_contact_number("apple"));

    println!("{:?}", divide(234, 6))
}