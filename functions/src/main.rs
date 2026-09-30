use std::println;

fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(6, 'h');

    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    let value = five();
    println!("The file function value : {value}");
}

fn another_function(x: i32) {
    println!("Another function, the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn five() -> i32 {
    let mut i = 5;
    i + 5
}