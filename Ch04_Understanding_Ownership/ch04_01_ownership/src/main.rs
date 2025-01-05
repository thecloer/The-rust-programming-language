fn main() {
    // move
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}, world!"); // error


    // clone
    let s1 = String::from("hello");

    // 힙 영역까지 복사된다.
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");


    // copy
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}"); // x는 유효하다.


    // 매개변수 소유권 이동
    let s = String::from("hello");

    take_ownership(s); // move
    // println!("{s}"); // error[E0382]: borrow of moved value: `s`

    let x = 5;

    make_copy(x); // copy
    println!("{x}");


    // 반환값 소유권 이동
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);


    // 튜플로 소유권 돌려 받기
    let s = String::from("hello");
    let (len, s) = get_length(s);
    println!("\"{s}\" is {len} letters");
}

fn take_ownership(s: String) {
    println!("{s}");
} // s drop

fn make_copy(x: u32) {
    println!("{x}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String{
    s
}

fn get_length(s: String) -> (usize, String) {
    (s.len(), s)
}