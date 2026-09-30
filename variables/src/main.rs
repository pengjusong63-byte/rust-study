use std::println;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // 变量可变性
    let mut x = 5;
    println!("The value of x is {x}"); // 5
    x = 6;
    println!("The value of x is {x}"); // 6

    // 遮蔽
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6 

    let mut space = "   ";
    let space = spaces.len();
}
