
pub fn main() {
    let mut count = 1;

    // infinite loop
    println!("infinite loop, 1-4");
    loop {
        println!("count: {}", count);
        count += 1;

        if count == 5 {
            break;
        }
    }

    // last number exclusive
    println!("last num excluded, in range 1-5");
    for i in 1..5 {
        println!("i: {}", i);
    }

    // last number inclusive
    println!("last num included, in range 1-5");
    for i in 1..=5 {
        println!("i : {}", i);
    }

    // looping with iterators
    let arr = [1, 2, 3, 4];

    println!("looping, iterators, 1-4");
    for i in arr.iter() {
        println!("i is: {}", i)
    }

    // loop that counts to 10 but skips 5
    for i in 1..=10 {
        if i == 5 {
            continue;
        }
        println!("{}", i)
    }

    // while loop
    let mut number = 5;

    while number < 10 {
        println!("number is: {}", number);
        number += 1;
    }
}
