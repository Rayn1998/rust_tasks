use std::io;

pub fn fibonaci() {
    loop {
        println!("Please enter the number");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");

        let mut n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if n == 1 {
            println!("The 1st Fibonacci number is: 0");
            continue;
        } else if n == 2 {
            println!("The 2nd Fibonacci number is: 1");
            continue;
        } else if n == 3 {
            println!("The 3rd Fibonacci number is: 1");
            continue;
        }

        let mut first_index: u32 = 1;
        let mut second_index: u32 = 1;
        let mut res = 0;
        n -= 3;
        
        while n > 0 {
            res = first_index + second_index;
            first_index = second_index;
            second_index = res;
            n -= 1;
        };
        println!("The {input}'th Fibonacci number is: {res}");
    }
}

pub fn f_to_celsius() {
    loop {
        println!("Please enter the value in Fahrenheit:");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
    
        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let celsius: f32 = (input - 32.0) / 1.8;
        println!("This temperature in Celsius is: {celsius}");
    }
}