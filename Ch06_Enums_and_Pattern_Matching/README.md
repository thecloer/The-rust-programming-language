# Chapter 6. 열거형과 패턴 매칭

## 학습 목표

> - `Enum`과 `variant`의 사용을 익힌다.
> - `Option`이라는 특별한 열거형의 사용법을 익힌다.
> - `match` 표현식의 사용법을 익힌다.
> - `if let` 구문의 사용법을 익힌다.

## 6.1. 열거형 정의하기

구조체가 서로 연관된 필드 및 데이터를 묶는데 사용된다면 열거형은 여러 가능한 값의 집합을 제공한다.

### 6.1.1. 열거형 값

- 열거형을 정의할 때 식별자로 네임스페이스가 만들어져 연관함수와 마찬가지로 이중콜론(`::`)을 사용해야 한다.
- 이런 표현은 `enum` 자체가 타입이라는 것을 표현하기 직관적이다.

```rs
enum IpAddrKind {
    V4,
    V6
}
let v4 = IpAddrKind::V4;
let v6 = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) {} // enum을 타입으로 사용
route(IpAddrKind::V4) // enum의 배리언트 중 하나를 값으로 사용
```

- enum에 직접 값을 저장할 수 있다.

```rs
enum IpAddrString {
    V4(String),
    V6(String)
}
let home = IpAddrString::V4(String::from("127.0.0.1"));
let loopback = IpAddrString::V6(String::from("::1"));

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

- 참고: [표준 라이브러리의 IP 저장 방법](https://doc.rust-lang.org/std/net/enum.IpAddr.html)

```rs
struct Ipv4Addr { /* ... */ }
struct Ipv6Addr { /* ... */ }
enum IpAddr {
    V4(Ipv4Addr),
    V4(Ipv6Addr),
}
```

- 열거형 배리언트의 값에는 어떤 종류의 타입도 담을 수 있다.

```rs
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}
```

### 6.1.2. 열거형 메서드

- 구조체와 비슷하게 열거형 역시 `impl` 키워드를 이용해 메서드를 정의할 수 있다.

```rs
impl IpAddr {
    fn to_string(&self) -> String {
        match self {
            IpAddr::V4(o1, o2, o3, o4) => format!("{}.{}.{}.{}", o1, o2, o3, o4),
            IpAddr::V6(addr) => addr.clone(),
        }
    }
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

println!("Home IP: {}", home.to_string());
println!("Loopback IP: {}", loopback.to_string());
```

## 6.2. `Option` enum이 `null`보다 좋은 점들

- `Option`타입은 값이 있거나 없을 수 있는 상황을 나타내는 표준 라이브러리에 정의된 enum이다.
- Rust에서는 "현재 어떠한 이유로 인해 유효하지 않거나, 존재하지 않는 하나의 값"이라는 의미로 `null` 대신 `Option`을 사용한다.
- `Option`은 기본 import 목록인 플렐루드(prelude)에 포함되어있으며 `Option`의 variant인 `None`과 `Some(T)`역시 플렐루드에 포함되어있어 `Option::`을 붙이지 않고 사용할 수 있다.
- rust에서는 `Option`을 통해 예외처리를 강제한다.
- `match` 표현식을 사용하면 예외처리를 쉽게 할 수 있다.

```rs
let some_number = Some(5); // Option<i32>
let some_char = Some('e'); // Option<char>
let absent_number: Option<i32> = None; // Option<char>

enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

> `match` 표현식 사용법
>
> - `match` 키워드 뒤에는 표현식이 온다.
> - `if` 문과 비슷하지만 `match` 키위드 뒤의 표현식은 평가 결과가 어떤 타입이어도 상관 없다.
> - 갈래는 패턴과 코드 부분으로 나뉘며 `=>`연산자로 구분된다.
> - 갈래와 갈래는 쉼표로 구분된다.
> - 코드부가 길다면 `{}` 중괄호 블럭을 사용하며 쉼포는 옵션이다.
> - 갈래의 패턴이 변수를 담는 배리언트라면 변수 바인딩을 통해 내부 변수를 참조할 수 있다.
> - 모든 경우의 수에 대한 갈래를 작성해야한다.

## 6.3. 포괄자 패넡과 `_` 자리표시자

주사위를 굴려 특정 숫자에 따라 행동이 달라지는 게임을 개발한다고 가정해보자.

```rs
let dice_number = roll_dice(); // u8
match dice_number {
    3 => add_fancy_hat(),
    6 => remove_fancy_hat(),
    other => move_player(other), // catch-all
}
match dice_number {
    3 => add_fancy_hat(),
    6 => remove_fancy_hat(),
    _ => reroll(), // 값을 사용하지 않는 경우
}
match dice_number {
    3 => add_fancy_hat(),
    6 => remove_fancy_hat(),
    _ => (), // 유닛을 반환 (void)
}

fn roll_dice() -> u8 { 1 }
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

- 포괄 패턴(catch-all)은 마지막에 위치해야한다.
- `_`는 자리표시자로 left-hand value에만 사용가능하다.
- 변수를 사용하지 않을 경우 변수 명을 자리표시자(`_`)로 작성하여 unused value 경고를 띄우지 않도록 한다.

## 6.4. if let을 사용한 간결한 제어 흐름

`if let` 문법은 하나의 패턴만 매칭 시키고 나머지 경우는 무시하도록 값을 처리하는 간결한 Syntax sugar다.

```rs
let config_max = Some(3u8);
match config_max {
    Some(max) => pringln!("The maximum is configured to be {}", max),
    _ => (),
}

// 하나의 패턴만 검사하는 간결한 if let 문법
let config_max = Some(3u8);
if let Some(max) = config_max {
    pringln!("The maximum is configured to be {}", max);
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("This is a quarter form {}", state);
} else {
    count += 1;
}
```
