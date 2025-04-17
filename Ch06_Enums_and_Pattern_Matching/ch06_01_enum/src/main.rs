use std::ptr::null;

enum IpAddrKind {
    V4,
    V6
}
enum IpAddrString {
    V4(String),
    V6(String)
}
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}


impl IpAddr {
    fn to_string(&self) -> String {
        match self {
            IpAddr::V4(o1, o2, o3, o4) => format!("{}.{}.{}.{}", o1, o2, o3, o4),
            IpAddr::V6(addr) => addr.clone(),
        }
    }
}


fn main() {
    // let ip4 = IpAddrKind::V4;
    // let ip6 = IpAddrKind::V6;

    // let home = IpAddrString::V4(String::from("127.0.0.1"));

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // println!("Home IP: {}", home.to_string());
    // println!("Loopback IP: {}", loopback.to_string());


    let dice_number = roll_dice(); // u8
    match dice_number {
        3 => add_fancy_hat(),
        6 => remove_fancy_hat(),
        other => move_player(other), // ca
    }
}

fn roll_dice() -> u8 { 1 }
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    print!("dice: {}", num_spaces);
}