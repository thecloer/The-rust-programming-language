# Chapter 3. 일반적인 프로그래밍 개념

## 학습 목표

> - 대부분의 프로그래밍 언어에서 일반적 사용되는 개념을 러스트에서 살펴본다.
> - 변수, 기본타입, 함수, 주석, 제어 흐름 등

## 3.1. 변수와 가변성

변수는 기본적으로 불변(immutable)이다.

- 불변성은 프로그램의 동적을 더욱 예상 가능하게 만든고 추적이 쉽도록 한다.
- 가변성은 유용하고 재사용성을 높인다.
- 코드의 의도를 명확히해 가독성과 예상 가능성을 높인다.

```rs
let x = 5;
x = 6; // error[E0384]: cannot assign twice to immutable variable `x`

let mut y = 5;
y = 6;
```

### 3.1.1. 상수

- `상수(constant)`는 변수명에 바인딩된 값이 변하지 않는다는 점에서 불변 변수와 비슷하다.
- 불변 변수와의 차이점
  - 상수는 `mut` 키워드와 함께 사용할 수 없다.(항상 불변이다.)
  - `const` 키워드로 선언한다.
  - 값의 타입이 반드시 명시되어야 한다.
  - <ins>상수는 상수 표현식으로만 선언 가능하며 런타임에서 계산될 수 있는 결괏값으로는 초기화 할 수 없다.</ins>
  - 컴파일 타임에 제한된 연산을 평가해 상수 선언에 사용할 수 있다. (참고: [상숫값 평가](https://doc.rust-lang.org/stable/reference/const_eval.html))
  - 전역 스코프를 포함한 모든 스코프에서 선언 가능하다.
  ```rs
  let a = 'a'; // error: expected item, found keyword `let`
  const b = 'b'; // error: missing type for `const` item
  // const b: char = 'b';
  fn main() {
      println!("{a}, {b}");
  }
  ```
- 변경점을 모으고 가독성을 향상시킨다.

### 3.1.2. 섀도잉

- 같은 이름의 변수 재선언이 가능하며 이전 선언된 변수는 가려져(shadowed) 접근할 수 없다.
- 섀도잉은 가변 변수와 다르다. 섀도잉은 새로룬 변수를 정의한다.(타입이 다를 수 있다.)

```rs
let mut x = 5; // x = 5
x = 6; // x = 6
let x = x + 100; // new x is 106 and old x is shadowed
{
    let x = x + 1000; // inner x is 1106
    println!("The value of x in the inner scope is: {x}");
}
println!("The value of new x is: {x}"); // new x is 106

let x = 'A';
println!("The value of x is: {x}"); // A
```

## 3.2. 데이터 타입

- 러스트에서 모든 값은 데이터 타입을 가지며 `스칼라 타입`과 `복합 타입` 두 종류고 분류된다.
- 러스트는 정적 타입(statically typed)언어이므로 모든 변수의 타입은 컴파일 시간에 정해져있어야 한다.
  ```rs
  // 런타임에서 결정되는 값은 타입 명시가 필요하다!
  let guess: u32 = "42".parse().expect("Not a number!");
  ```

### 3.2.1. 스칼라 타입

스칼라 타입은 하나의 값을 표현한다. 아래 네가지 스칼라 타입이 있다.

- 정수: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- 부동 소숫점 숫자
- 불리언
- 문자

#### 정수

- 러스트의 기본 정수 타입은 `i32`다. `iszie`와 `usize`는 주로 어떤 컬렉션 종류의 인덱스에 사용된다.

|  길이   | signed  | unsigned |
| :-----: | :-----: | :------: |
|  8-bit  |  `i8`   |   `u8`   |
| 16-bit  |  `i16`  |  `u16`   |
| 32-bit  |  `i32`  |  `u32`   |
| 64-bit  |  `i64`  |  `u64`   |
| 128-bit | `i128`  |  `u128`  |
|  arch   | `isize` | `usize`  |

|   숫자 리터럴    |           예시           |
| :--------------: | :----------------------: |
|   십진 자릿수    |        1_000_000         |
|       2진        | 0b11110000 = 0b1111_0000 |
|       8진        |          0o_360          |
|       16진       |   0x_f0 = 0xf_0 = 0xf0   |
| 타입 지정 접미사 |   240u8, 0b1111_0000u8   |

> 러스트에서 오류가 발생해 프로그램을 종료되는 경우를 `패닉(panic)`이라 한다.  
> 코드를 디버그 모드로 컴파일 할 경우, 러스트 컴파일러는 런타임에 정수 오버플로우가 발생하면 패닉을 발생시키는 검사를 포함시킨다.  
> `--release` 플래그를 사용해 코드를 릴리스 모드로 컴파일 하는 경우, 패닉을 발생시키는 정수 오버플로우가 발생하면 러스트는 `2의 보수 감싸기(two's complement wrapping)`를 수행한다. 즉, 최댓값이 255인 u8에 오버플로우 정수를 할당 할 경우 256은 0, 257은 1이된다. 프로그램은 패닉을 발생시키지 않지만 예상치 못한 결과를 얻을 수 있다.  
> 명시적으로 오버플로우의 가능성을 다루기 위해 표준 라이브러리에서 기본 수치 타입에 대해 제공하는 아래 메서드 종류들을 사용할 수 있다.
>
> - `wrapping_add`와 같은 `wrapping_*` 메서드로 wrapping 동작 실행하기
> - `checked_*` 메서드를 사용해 오버플로우가 발생하면 `None`값 반환하기
> - `overflowing_*` 메서드를 사용해 값과 함께 오버플로우 발생 여부를 불리언 값으로 반환 하기
> - `saturating_*` 메서드를 사용해 값의 최댓값 혹은 최솟값 사이로 제안하기

#### 부동소수점 타입

- 러스트에 부동소수점 타입 `f32`와 `f64`가 있다.
- 기본 타입은 `f64`다.
- 모든 부동소수점에는 부호가 있다.
- 부동소수점은 IEEE-754 표준을 따른다.

```rs
let double = 2.0; // f64
let float: f32 = 2.0; // f32
```

#### 수치 연산

- `+`, `-`, `*`, `/`, `%`, ...
- 정수의 나눗셈은 가장 가까운 정숫값을 버림한다.

#### 불리언 타입

- 1바이트
- `bool`, `true`, `false`

#### 문자 타입

- 4바이트, 유니코드 스칼라 값
- `char`, 작은따옴표

### 3.2.2. 복합 타입

- `복합 타입(compound type)`
- `튜플`과 `배열` 기본 복합 타입이 있다.

#### 튜플

- 고정된 길이와 위치에 고정된 타입
- 빈 튜플을 `유닛(unit)`이라 한다. 표현식이 아무 값도 반환하지 않는다면 암묵적으로 유닛을 반환한다.

```rs
let tup: (i32, f64, u8) = (500, 6.4, 1); // 타입 생략 가능
let (x, y z) = tup; // destructuring
let first = tup.0;
let second = tup.1;
let third = tup.2;
```

#### 배열

- 모든 요소의 타입이 같아야한다.
- 고정 길이다.
- cpp와 같이 가변 길이의 유연한 컬렉션 표준 라이버리 벡터가 있다.

```rs
let arr2 = [1, 2, 3, 4, 5];
let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // 길이와 타입을 지정할 수 있다.
let init_arr = [0; 5]; // = [0, 0, 0, 0, 0]: 초깃값과 길이로 초기화 할 수 있다.
let first = arr1[0];
let second = arr1[1];
```

## 3.3. 함수

- 러스트는 관례로 함수명과 변수명에 `스네이크 케이스(snake_case)`를 사용한다.
- `fn`키워드로 함수를 정의한다.
- 함수 정의의 순서는 중요하지 않고 호출부에서 참조 가능한 스코프에만 정의 되어있으면 된다.

### 3.3.1. 매개변수

- `함수 시그니처(function signature)`에 정의되는 변수를 `매개변수(parameter)`라 한다.
- 매개변수에 전달된 값을 `인수(argument)`라 한다.

```rs
fn main() {
    let res = sum(1, 2);
    println!("{res}");
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}
```

### 3.3.2. 구문과 표현식

- 함수의 시그니처에는 매개변수의 타입이 반드시 선언되어야한다.
- 함수의 본문은 필요에 따라 `표현식(expression)`으로 끝나는 `구문(statement)`의 나열로 구성된다.
  - `구문(statement)`: 어떤 동작을 수행하고 값을 반환하지 않는 명령어
  - `표현식(expression)`: 결괏값을 평가하는 명령어
  - 표현식은 구문의 일부분 일 수 있다.
- 러스트에서는 대부분이 표현식이지만 할당문은 표현식이 아니다.
  - 러스트는 `표현식 기반 언어(expression-based)`로 Javascript의 undefined 처럼 `()`(unit)을 반환할 뿐 대부분 표현식이다.
  - 함수 호출, 매크로 호출 또한 표현식이며 중괄호로 만든 스코프 블록도 표현식이다.
- <u>표현식은 종결을 나타내는 세미콜론을 쓰지 않는다.</u>
  - 표현식 끝에 세미 콜론을 추가할 경우 표현식은 구문으로 변경되어 값을 반환하지 않는다.
  - 이를 이용해 함수의 마지막 부분에 세미콜론을 쓰지 않는 표현식을 위치시켜 반환할 수 있다.

```rs
fn main() { // 함수 정의 구문
    let x = 6; // 변수 할당 구문
    let y = (let x = 6); // error: expected expression, found `let` statement

    let z = { // 평가 결과는 4다.
        let x = 3;
        x + 1 // 마지막 줄이 세미클론으로 끝나지 않는다.
    };

    let res = sum(1, 2); // 3
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // 표현식 반환
}
```

### 3.3.3. 반환값을 갖는 함수

- 함수의 반환값은 함수 본문의 마지막 표현식 값과 동일하다.
- `return` 키워드와 값을 지정해 일찍 값을 반환할 수 있지만 대부분의 함수는 암묵적으로 마지막 표현식을 반환한다.

```rs
fn get_five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    let x = get_five(); // 5
    let y = plus_one(x); // 6
}
```

## 3.4. 제어 흐름

### 3.4.1. if 표현식

- 조건식은 반드시 bool타입이어야 한다. 다른 타입일 경우 컴파일되지 않는다.
- `else if`문으로 여러 분기를 만들 수 있지만 표현식이 3개 이상일 경우 `match`를 사용해 리펙터링하는 것이 좋다.
- `if`는 표현식이므로 Rvalue로 사용가능하다.
  - 러스트는 컴파일 타임에 변수의 타입이 결정되어야 하므로 분기 결괏값의 타입이 다를 경우 런타임에 변수의 타입이 결정되므로 오류가 발생한다.

```rs
let number = 3;
if number { // error: mismatched type
    println!("{number}");
}

let is_negative = true;
let number = if is_negative { -1 } else { 1 }; // Rvalue
```

### 3.4.2. 반복문

- 세 종류의 반복문이 있다. `loop`, `while`, `for`
  - `loop`만 `break`를 이용해 값을 반환할 수 있다.
  - `while`, `for`는 `break`로 값을 반환 할 수 없다.
  - `while`, `for` 역시 표현식이지만 항상 유닛을 반환해 구문처럼 사용된다.
- `break`와 `continue`를 통해 흐름을 제어할 수 있다.
- `break`를 `return`처럼 사용해 반복문에서 값을 반환할 수 있다.
- Nested loop에는 `루프 라벨(loop label)`을 추가해 명시적으로 `break`와 `continue`를 적용할 수 있다.

```rs
let mut counter = 0;

let result = loop { // 반복문 할당
    counter += 1;
    if counter == 10 {
        break counter * 2; // break로 반환
    }
};

println!("Result is {result}"); // 20
```

#### loop label

```rs
let mut counter = 0;
let res = 'outer: loop {
    counter += 1;
    loop {
        counter += 1;
        if counter < 5 {
            break;
        }
        break 'outer counter;
    }
}
```

#### while 문

- `while`문은 항상 유닛을 반환하는 표현식이다.
- `break`로 값을 반환 할 수 없다.

```rs
let mut counter = 3;
while counter != 0 {
    counter -= 1;
};
println!("{counter}");
```

#### for 문

- `for`문은 항상 유닛을 반환하는 표현식이다.
- `break`로 값을 반환 할 수 없다.
- 파이썬의 for문과 비슷해 컬렉션을 순회한다.
  - 파이썬의 Range 함수 처럼 표준 라이브러리에서 `Range` 타입을 제공한다.

```rs
let arr = [10, 20, 30, 40, 50];
for element in arr {
    println!("{element}");
};

// 3, 2, 1 역순 출력
for number in (1..4).rev() {
    println!("{number}");
}
```
