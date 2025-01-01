const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");
    
    // constant
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
    
    // shadowing
    let x = x + 100;
    {
        let x = x + 1000;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of new x is: {x}");
    
    let x = 'X';
    println!("The value of x is: {x}");

    let y = 0o_3_6_0;
    println!("{y}")
}
