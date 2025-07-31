fn check_if_odd(x: i32) -> bool {
    if x % 2 == 0 {
        return false;
    }

    return true;
}

pub fn main() {
    let is_odd: bool;

    is_odd = check_if_odd(5);

    match is_odd {
        true => println!("number is odd"),
        false => println!("number is even"),
        _ => println!("something else"),
        // true | false => println!("true or false happened")
    }
}
