fn main() {
    let mut s = String::from("hello");
    let len = get_length(&s);
    println!("The length of '{s}' is {len}.");

    let r1 = &mut s;
    println!("{r1}");
    s.push_str(", world!");


    change(&mut s);
    println!("{s}");
}

fn get_length(s1: &String) -> usize {
    s1.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}