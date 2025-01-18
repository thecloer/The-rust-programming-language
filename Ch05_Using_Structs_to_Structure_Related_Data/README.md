# Chapter 5. 구조체로 연관된 데이터 구조화하기

## 학습 목표

> -
> -

## 5.1. 구조체

### 5.1.1. 구조체 정의 및 인스턴스화

- 구조체는 여러 개의 연관된 값을 가질 수 있다는 점에서 튜플 타입과 비슷하다.
- 필드와 타입 쌍으로 Typescript의 인터페이스터럼 정의할 수 있다.
  ```rs
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  ```
- Javascript의 객체 리터럴과 비슷한 문법으로 인스턴스를 생성할 수 있다.
  ```rs
  let user1 = User {
      active: true,
      username: String::from("username1"),
      email: String::from("email1@example.com"),
      sign_in_count: 1,
  };
  ```
- 점표기법으로 필드에 접근할 수 있다.
- 특정 필드만 가변을 만들 수 없고 인스턴스 전체가 가변 혹은 불변이다.
- Javascript와 비슷하게 필드명과 값 변수 명이 같을 경우 축약형을 사용할 수 있다.
  ```rs
  let username = String::from("username1");
  let email = String::from("email1@example.com");
  let user1 = User {
      active: true,
      username, // 축약형
      email, // 축약형
      sign_in_count: 1,
  };
  ```
- Javascript의 구조분해 할당과 비슷한 문법을 지원한다.(struct update syntax) base struct는 항상 마지막에 위치해야 한다.
  ```rs
  let user2 = User {
      email: String::from("another@example.com");
      ..user1 // base struct는 항상 마지막에 위치해야 한다.
  };
  ```
- 구조체 업데이트 문법은 대입 연산(`=`)을 사용한다. 따라서 위 예제에서 `String` 타입의 `username`과 `email`의 소유권이 이동한다. 스택 변수인 `active`와 `sign_in_count`는 복사된다.

### 5.1.2. 필드명 없는 튜플 구조체

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let black: Point = Color(0, 0, 0); // error[E0308]: mismatched types
let origin = Point(0, 0, 0);
black.0;
```

### 5.1.2. 필드가 없는 유사 유닛 구조체

- 주로 필드는 필요없지만 트레이트만 구현하고 싶을때 사용한다.

```rs
struct None;
```

### 5.1.3. Debug 트레이트 파생

```rs
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}
let scale = 30;
let rect1 = Rectangle {
    width: dbg!(30 * scale), // 소유권 이동
    height: 50
}
println!("{:#?}", rect1);
dbg!(&rect1); // 참조
```

- `println!("{}", rect)`는 `Display`를 사용하지만 `Rectangle`에는 구현되어있지 않다.
- `{:?}`: `println!` 매크로의 `Debug` 출력 형식을 사용한다는 뜻이다.
- 속성을 추가해 기본적인 `Debug` 트레이트를 파생(derive) 할 수 있다.(`#[derive(Debug)]`)
- `{:#?}`: Debug 포멧을 사용해 값을 출력한다.
- `dbg!` 매크로를 사용하면 표현식의 소유권을 가져와 파일 및 라인 번호를 결괏값과 함께 출력하고 다시 소유권을 반환한다.

## 5.2. 메서드

- 메서드는 구조체 콘텍스트에 정의되는 함수다.
- 열거형이나 트레이트 객체에도 정의될 수 있다.
- 첫 번째 매개변수는 항상 `self`이며 `self`는 메서드를 호출하는 구조체 인스턴스를 의미한다.

### 5.2.1. 메서드 정의

- 메서드는 첫 인자로 `self`를 받는다.
  - `self`(소유권 이동), `mut self`(가변 소유권 이동), `&self`(읽기), `&mut self`(변경)로 받을 수 있다.
  - 소유권을 얻어오는 경우는 거의 없으며 다른 소유권을 반환하고 메서드 이후 해당 인스턴스의 사용을 방지할 때 사용한다.
  - 필드와 같은 이름의 메서드를 선언할 수 있다.
- 러스트는 자동 참조 및 역참조 기능을 제공해 C와 C++의 역참조 연산자(`->`)를 `.`연산자로 지원한다.
  - `ref.method()` = `(&ref).method()`

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```
