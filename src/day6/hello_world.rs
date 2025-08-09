
/*
formatted print, macros defined in std::fmt
 
format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as print! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint! but a newline is appended.
*/

// use std::fmt::format; 

pub fn main() {
    let _days = format!("{day_count} in {month_name}", day_count = 22, month_name = "August");
    println!("{}", _days)
}