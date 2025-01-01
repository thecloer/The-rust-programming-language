use std::io;

fn get_nth_fib(nth: usize, memo: &mut [u128]) -> u128 {
    if nth == 0 || nth == 1 {
        return memo[nth];
    }
    if memo[nth] != 0 {
        return memo[nth];
    }
    memo[nth] = get_nth_fib(nth - 1, memo) + get_nth_fib(nth - 2, memo);
    memo[nth]
}

fn main() {
    let mut memo = [0u128; 101];
    memo[1] = 1;

    loop {
        println!("\nWhich Fibonacci number would you like to find? (Enter Q to quit)");
        let mut command = String::new();
        if io::stdin().read_line(&mut command).is_err() {
            println!("Failed to read input.");
            continue;
        }
        command = command.trim().to_string();
        if command == "Q" || command == "q" {
            break; 
        }

        let number: usize = match command.parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("Please enter an integer between 1 and 100.");
                    continue;
                }
                num
            },
            Err(err) => {
                println!("{}", err.to_string());
                continue;
            }
        };
        
        let fib = get_nth_fib(number, &mut memo);
        println!("{number}th fibonacci number is {fib}");
    }
}
