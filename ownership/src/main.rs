use std::println;

fn main() {
    let s = String::from("hello");

    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{s}");
    println!("Hello, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1}, world!");

    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{x}");

    
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership : {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy : {some_integer}");
}