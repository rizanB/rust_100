// rust supports markdown in documentation
// docs can have tests inside code block

/// simple interest calculator using formula: ptr/100
///
/// arguments
/// p: principal, in rs
/// t: time, in years
/// r: rate, in percentage
///
/// returns
/// calculated interest, i32
///
/// Example
/// ```rust
/// use rust_100::day3::interest::calculate_interest;
///
/// assert_eq!(50, calculate_interest(100,5,10));
/// ```
pub fn calculate_interest(p: i32, t: i32, r: i32) -> i32 {
    // check if inputs are -ve
    if p < 0 || t < 0 || r < 0 {
        panic!("inputs cannot be negative");
    };

    // calculate and return and interest
    if p == 0 || t == 0 || r == 0 {
        0
    } else {
        (p * t * r) / 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_positive() {
        let interest = calculate_interest(100, 5, 10);

        assert!(interest > 0, "interest should be positive");
    }
}

pub fn main() {
    println!(
        "interest of rs. 100 for 5 years, at 10% rate of interest is rs. {}",
        calculate_interest(100, 5, 10)
    );
}
