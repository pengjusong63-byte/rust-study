use std::{io, println};

fn main() {
    // 整型
    let guess: u8 = "255".parse()
        .expect("Not a number");
    println!("The guess value is {guess}");
    // 浮点型
    let x = 2.0; // 默认为f64
    let y: f32 = 3.0;

    // 数值运算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    // 布尔类型
    let t = true;
    let f: bool = false;

    // 字符类型
    let c = 'z';
    let z: char = 'Z';

    // 复合类型
    // 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(& mut index)
        .expect("Failed to read line");
    let index: usize = index.
        trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
