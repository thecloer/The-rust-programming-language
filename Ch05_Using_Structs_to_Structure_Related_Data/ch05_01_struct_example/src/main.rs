struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}
impl Rectangle {
    // 메서드가 아닌 연관함수(static function): Rectangle::square(size)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        username: String::from("user2"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        height: 30,
        width: 50
    };
    println!("Function Area = {}", area(&rect1));
    println!("Method Area = {}", rect1.area());
    println!("{:?}", rect1);

    #[derive(Debug)]
    struct Rect {
        height: u32,
        width: u32
    }
    let scale = 30;
    let rect1 = Rect {
        width: dbg!(30 * scale),
        height: 50
    };
    println!("{:#?}", rect1);
    dbg!(&rect1);
    

    let square = Rectangle::square(10);
    println!("{:#?}", square);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}