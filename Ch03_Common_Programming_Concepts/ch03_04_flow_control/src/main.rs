use std::io;

fn if_else_example() {
    loop {
        let cmd = get_input();
        if cmd == "q" || cmd == "Q"{
            break;
        }

        let number: i32 = match cmd.parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                println!("Not a number!");
                continue;
            }
        };

        if number < 5 {
            println!("{number} is less than 5.");
        } else if number > 5 {
            println!("{number} is grater than 5.");
        } else {
            println!("{number} is equal to 5.");
        }
    }
}

fn nested_loop_example() {
    let mut counter = 0;

    let result = loop { // 반복문 할당
        counter += 1;
        if counter == 10 {
            break counter * 2; // break로 반환
        }
    };

    println!("Result is {result}"); // 20

    // loop label
    println!("o: loop outer, i: loop inner, others: quit");
    let last_point = 'outer: loop {
        println!("outer loop");
        let cmd = get_input();

        if cmd == "i" {
            loop {
                println!("inner loop");
                let cmd = get_input();

                if cmd == "i" { continue; } 
                if cmd == "o" { continue 'outer; } 
                else { break 'outer "inner"; } // loop label with return value;
            }
        } else if cmd == "o" { continue; }
        else { break "outer"; } // return value; 
    };
    println!("Nested loop is broken at {last_point} loop.");
}



fn main() {
    if_else_example();
    nested_loop_example();
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}