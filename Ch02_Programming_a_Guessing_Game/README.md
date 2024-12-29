# Chapter 2. 추리 게임

## 학습 목표

> - let, match, 메서드, 연관 함수, 외부 크레이트의 기초적인 사용법

## 2.1. 개요

이번 장에서는 1 ~ 100 사이의 임의의 정수를 생성하고 사용자가 추리하는 게임을 만들어본다. 만들고자하는 프로그램의 수도코드는 다음과 같다.

```txt
1. 1 ~ 100 사이의 임의의 정수를 생성한다.
2. 사용자에게 숫자를 입력받는다.
3. 입력받은 숫자와 생성된 숫자를 비교한다.
    3.1. 만약 입력받은 숫자가 생성된 숫자보다 작다면 "작다"를 출력한다.
    3.2. 만약 입력받은 숫자가 생성된 숫자보다 크다면 "크다"를 출력한다.
    3.3. 만약 입력받은 숫자가 생성된 숫자와 같다면 "정답"을 출력하고 종료한다.
```

## 2.2. 새 프로젝트 준비하기

```bash
cargo new guessing_game
```

## 2.3. 추릿값 처리하기

- `prelude`: 프로그램 내에서 별도의 명시적 import 없이도 바로 사용할 수 있도록 기본적으로 제공되는 표준 라이브러리 기능들의 집합.
- Rust 표준 라이브러리(std)에서 자주 사용되는 몇몇 타입과 트레이트들은 prelude에 포함되어 있다. 예를 들어, `Option`, `Result`, `Vec`, `String` 같은 타입이나, `Copy`, `Clone`, `Eq`, `Ord` 등의 트레이트는 별도의 use 선언 없이도 바로 사용할 수 있다.
- 일반적으로 Rust 코드의 최상단 부분에 `use` 키워드를 통해 필요한 네임스페이스를 불러와야한다. 하지만 prelude에 포함된 항목은 `use std::prelude::v1::*`; 와 같은 구문이 사실상 암시적으로 추가된 것으로 간주된다.

```rs
let mut name = String::new();
```

- 러스트에서 변수는 `let` 키워드로 선언하며 기본적으로 불변(immutable)이다.
- `mut` 키워드와 함께 선언하면 가변(mutable) 변수를 선언할 수 있다.
- `::new`는 String 타입의 연관 함수(associated function)
- `연관 함수(associated function)`: 타입에 구현된 함수 (class 의 static function으로 이해하면 쉬울 것 같다.)

```rs
io::stdin()
    .read_line(&mut guess)
    .expect("Fail to read line.");
```

- `&`는 메모리 복사를 하지 않고 참조자(reference)를 넘겨주는 것을 의미한다. (cpp의 reference)
- 참조자 역시 기본적으로 불변이다. 가변 참조자를 넘기기 위해서는 `&mut`를 사용해야한다.

## 2.4. `Result`를 이용한 에러 핸들링

- `Result`는 여러개의 가능한 상태중 하나가 될 수 있는 `enum`이다. 이러한 가능한 상탯값을 `variant`라 한다.
- `Result`의 배리언트는 `Ok`와 `Err`가 있다.
- `Result` 인스턴스에도 메서드가 있으며 `Result` 인스턴스의 값이 `Err`일 경우 프로그램을 멈추가 `expect`가 실행됩니다. `Ok`라면 결괏값을 반환합니다.

## 2.5. `println!`

`{}` 자리표시자(placeholder)

```rs
let var = 99;
println!("{{var}} {var} {}, {}", var, var + 1);
// "{var} 99 99 100"
```

## 2.6. 비밀번호 생성하기(크레이트)

- `크레이트(crate)`는 러스트 코드 파일 모음이다.
- 우리가 작성하는 프로그램은 실행 가능한 `바이너리 크레이트(binary crate)`다.
- `rand` 크레이트는 자체적으로 실행될 수 없고 다른 프로그램을 위해 작성된 `라이브러리 크레이트(library crate)`다.
- 카고를 이용해 프로젝트의 크레이트를 관리 할 수 있다.

`Cargo.toml`파일에 디펜던시를 추가.

```toml
[dependencies]
rand = "0.8.5"
```

[SemVer(Semantic Versioning)](https://semver.org/) "0.8.5" = "^0.8.5" = (0.8.5 이상 && 0.9.0 미만)

```bash
cargo add rand
cargo update
```

## 2.7. 값 비교

```rs
use std::cmp::Ordering;

// 생략...
let guess: i32 = guess.trim().parse().expect("Please type a number!");
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

- `Ordering`은 `Less`, `Greater`, `Equal` 배리언트를 갖는 열거형이다.
- `cmp()`는 비교하고 싶은 값의 참조자를 받아 `Ordering` 인스턴스를 반환한다.
- `match` 표현식으로 `cmp`의 결과에 따라 `갈래(arm)`로 분기된다.
- `match`의 값과 `패턴(pattern)`이 일치하는지 순서대로 확인하고 일치하는 패턴을 갖는 `갈래`가 실행된다.

* 러스트에서는 `섀도잉(shadowing)`으로 변수 오버라이딩을 허용한다. 주로 타입 변환시에 자주 사용된다.

## 2.8. `loop`를 이용한 반복문

- `loop`는 조건 없는 무한 반복문이다.
- `break`와 `continue`를 사용할 수 있다.

```rs
loop {
    println!("Please input your guess.");

    // 생략...

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
```

## 2.9. 예외 처리

```rs
// let guess: i32 = guess.trim().parse().expect("Please type a number!");
let guess: i32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("It's not a number.");
        continue;
    }
};
```
