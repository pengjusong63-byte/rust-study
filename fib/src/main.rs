use std::{io, print, println};
fn main() {
    let value = fib(6);

    println!("The value is {value}");
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}