use std::io;

fn main() {
    loop {
        println!("Enter a number greater than 0 and hit 'Enter' \nto calculate the number at that position \nin the Fibonacci series:");
    
        let mut num = String::new();
    
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
    
        let num: i32 = match num.trim().parse() {
            Ok(dig) => {
                if dig < 1 {
                    println!("Please enter a number greater than 0.");
                    continue;
                }
                else {dig}},
            Err(_) => continue,
        };
    
        let fib = fib_calc(num);
    
        println!("Fibonacci number # {num} is {fib}.")
    }
}

fn fib_calc(num: i32) -> i32 {
    if num == 1 {
        1
    } else if num == 2 {
        2
    } else if num == 3 {
        3
    }
    else {
        let mut fib1: i32;
        let mut fib2: i32 = 3;
        let mut fib3: i32 = 5;
        let mut counter = 4;
        while counter < num {
            fib1 = fib2;
            fib2 = fib3;
            fib3 = fib1 + fib2;
            counter = counter + 1;
        }
        return fib3
    }
}
