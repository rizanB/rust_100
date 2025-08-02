// dont need a [cfg(test)] guard for files inside tests/

use rust_100::day3::interest::calculate_interest;

#[test]
fn test_calculate_interest(){
    assert_eq!(50, calculate_interest(100, 5, 10));
    assert_eq!(20, calculate_interest(200, 2, 5));
    assert_eq!(60, calculate_interest(500, 1, 12));   
}

#[test]
fn test_for_zero_values(){
    assert_eq!(calculate_interest(0, 5, 10), 0);
    assert_eq!(calculate_interest(100, 0, 10), 0);
    assert_eq!(calculate_interest(100, 5, 0), 0);
}

#[test]
#[should_panic(expected="inputs cannot be negative")]
fn test_negative_principal(){
    calculate_interest(-100, 5, 10);
}