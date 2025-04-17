# Chapter 4. 소유권 이해하기

## 학습 목표

> - 소유권을 이해한다.
> - 대여(borrowing), 슬라이스(slice), 러스트에서 데이터를 메모리에 저장하는 방법을 이해한다.

## 4.1. 소유권

- `소유권(ownership)`은 러스트 프로그램의 메모리 관리 법을 지배하는 규칙 모음이다.
- 소유권의 주요 목표는 힙 데이터 관리다.

모든 프로그램은 작동하는 동안 컴퓨터의 메모리를 관리해야한다. 크게 프로그래머가 직접 메모리를 관리하는 언매니지드 언어와 가비지 컬렉터에게 메모리 관리를 위임하는 매니지드 언어로 분류된다.  
러스트는 소유권 시스템을 통해 언매니지드 언어에서 발생할 수 있는 휴먼 에러와 매니지드 언어에서 발생할 수 있는 성능 저하를 개선하여 안정성과 속도를 취한다.

### 4.1.1. 소유권 규칙

- 각각의 값은 `소유자(owner)`가 정해져 있다.
- 한 값의 소유자는 동시에 여럭 존재할 수 없다.
- 소유자가 스코프 밖으로 벗어날 때, 값은 버려진다.(dropped)

* `스코프(scope)`: 프로그램 내에서 아이템이 유효한 범위
* 스코프는 다른 언어와 비슷하다.

### 4.1.2. 메모리와 할당

- `::`: 타입에 정의된 함수를 지정할 수 있는 네임 스페이스 연산자.

```rs
let mut s = String::from("hello");
s.push_str(", world!"); // String 타입 값 s에 문자 리터럴 추가
```

- 문자열 리터럴은 불변(immutable)로 컴파일 타임에 길이가 고정되고 최종 실행 바이너리 파일에 하드코딩된다.
- 컴파일 타임에 크기를 알 수 없고 런타임에 변할 수 있는 텍스트는 바이너리 파일에 넣을 수 없다.
- `String` 타입은 힙에 메모리를 할당하는 방식을 사용해 텍스트의 내용 및 크기를 변경할 수 있다.
  - 실행 중 메모리 할당자로부터 메모리를 요청해야한다.
    - `String::from`, `String::new`, ...
  - String 사용을 마쳤을 때 메모리를 해제할(즉, 할당자에게 베모리를 반납할) 방법이 필요하다.
    - GC, free, 스코프를벗어나는 순간 메모리 자동 해제([drop](https://doc.rust-lang.org/std/ops/trait.Drop.html)), ...

```rs
{
    let s = String::new();

    // s의 유효 지점
} // 스코프가 종료되면 s는 더 이상 유효하지 않다.
```

- rust는 변수가 스코프 밖으로 벗어나면 [drop](https://doc.rust-lang.org/std/ops/trait.Drop.html)이라는 특별한 함수를 호출한다.
- `drop` 함수를 구현함으로써 해당 타입을 개발할 때 직접 메모리 해제 코드를 작성할 수 있다.
- `drop` 함수는 스코프가 종료되는 `}`가 나타나는 시점에서 자동으로 호출된다.
- C++에서는 이런 식으로 아이템의 수명이 끝나는 시점에 리소스를 해제하는 패턴을 `RAII(Resource acquisition is initialization)`라 한다.

#### 변수와 데이터 간 상호작용 방식: `이동`

- 컴파일 타임에 크기가 고정된 타입은 값의 복사본을 바인딩해 두 값이 스택에 push 된다.

```rs
let x = 5;
let y = x; // 값 복사
```

- 스택 영역에 저장되는 `String` 타입은 세 부분으로 이루어져 있다.
  - `포인터`: 힙 영역에 문자열 내용이 저장된 메모리 주소를 참조하는 포인터
  - `문자열 길이`: 현재 사용하고 있는 메모리의 바이트
  - `메모리 용량`: 메모리 할당자가 `String`에 할당한 메모리 양
- `String`타입의 변수 `s1`을 `s2`에 대입하면 <u>스택 영역의 데이터만 복사</u>된다.
  - `s1`과 `s2`가 스코프 밖으로 벗어나 `drop`함수를 호출한다면 같은 힙 메모리 영역에 대해 `중복 해제(double free)` 에러가 발생한다.
  - 메모리 안정성을 보장하기 위해 러스트는 `let s2 = s1;` 라인 뒤로 `s1`이 더 이상 유효하지 않다고 판단한다. 따라서 `s1`은 스코프를 벗어나더라도 해제할 필요가 없다.
- 힙 영역을 복사하지 않고 스택 영역만 복사하기 때문에 `얕은 복사(shallow copy)`라고 생각할 수 있지만 러스트에서는 <u>기존 변수를 무효화하기 때문에 이를 얕은 복사가 아닌 `이동(move)`이라 한다.</u> (`s1`이 `s2`로 이동됐다.)
- `이동`은 기존 변수를 무효화 시키고 소유권 또한 이동한다.

```rs
let s1 = String::from("hello");

// 스택 영역의 데이터만 복사. 힙 영역은 복사되지 않는다.
// `s1`이 `s2`로 이동해 더 이상 `s1`은  더 이상 유효하지 않다.
let s2 = s1;

// error[E0382]: borrow of moved value: `s1`
println!("{s1}, world!");
```

```text
      stack mem (s1)                 heap mem
  +----------+---------+         +-------+-------+
  |   name   |  value  |         | index | value |
  +----------+---------+         +-------+-------+
  | pointer  |    ·----+--X--+-->|   0   |   h   |
  |  length  |    5    |     |   |   1   |   e   |
  | capacity |    5    |     |   |   2   |   l   |
  +----------+---------+     |   |   3   |   l   |
           |                 |   |   4   |   o   |
           |  move           |   +-------+-------+
           V                 |
      stack mem (s2)         |
  +----------+---------+     |
  |   name   |  value  |     |
  +----------+---------+     |
  | pointer  |    ·----+-----+
  |  length  |    5    |
  | capacity |    5    |
  +----------+---------+
```

#### 변수와 데이터 간 상호작용 방식: `클론`

- `클론(clone)`을 통해 힙 영역까지 복사할 수 있다.
- 힙 영역의 데이터가 큰 경우 성능에 영향을 미칠 수 있다.

```rs
let s1 = String::from("hello");

// 힙 영역까지 복사된다.
let s2 = s1.clone(); // 메서드

println!("s1 = {s1}, s2 = {s2}");
```

```text
      stack mem (s1)                 heap mem
  +----------+---------+         +-------+-------+
  |   name   |  value  |         | index | value |
  +----------+---------+         +-------+-------+
  | pointer  |    ·----+-------->|   0   |   h   |
  |  length  |    5    |         |   1   |   e   |
  | capacity |    5    |         |   2   |   l   |
  +----------+---------+         |   3   |   l   |
           |                     |   4   |   o   |
           |  clone              +-------+-------+
           V
      stack mem (s2)
  +----------+---------+         +-------+-------+
  |   name   |  value  |         | index | value |
  +----------+---------+         +-------+-------+
  | pointer  |    ·----+-------->|   0   |   h   |
  |  length  |    5    |         |   1   |   e   |
  | capacity |    5    |         |   2   |   l   |
  +----------+---------+         |   3   |   l   |
                                 |   4   |   o   |
                                 +-------+-------+
```

#### 스택에만 저장되는 데이터: `복사`

- 정수 등 컴파일 타임에 크기가 고정되는 타입은 모두 스택에 저장된다.
- 스택 영역에 저장되기 때문에 복사본을 빠르게 만들 수 있고 굳이 원본을 무효화 할 필요가 없다.
- 따라서 `clone`을 호출 해도 얕은 복사와 차이가 없으니 생략된다.
- 러스트에는 정수형처럼 스택에 저장되는 타입에 붙여놓을(place) 수 있는 `Copy 트레이트`가 있다.
  - `Copy 트레이트`가 구현되어 있다면, 이 타입의 변수는 사용되어도 이동하지 않고, 대입 연산 이후에도 유효하다.
  - 구현하려는 타입 혹은 구현하려는 타입 중 일부분에 `Drop 트레이트`가 구현된 경우엔 `Copy 트레이트`를 애너테이션(annotation) 할 수 없다.
  - 스코프 밖으로 벗어났을 때 특정 동작이 요구되는 타입에 `Copy 애너테이션`을 추가하면 컴파일 에러가 발생한다.
  - 일반적으로 스칼라 값의 묶음은 Copy가 가능하고 할당이 필요하거나 리소스의 일종인 경우엔 불가능하다.
    - 모든 정수형, bool, 모든 부동소수점 타입, char, Copy 가능한 타입으로만 구성된 튜플

```rs
let x = 5;
let y = x; // let y = x.clone(); 도 같은 동작을 한다.
println!("x = {x}, y = {y}"); // x는 유효하다.
```

### 4.1.3. 소유권과 함수

- 함수로 값을 전달하는 메커니즘은 변수에 값을 대입할 때와 유사하다.
- 함수에 변수를 전달하면 대입 연산과 마찬가지로 이동 혹은 복사가 발생한다.

```rs
fn main() {
    let s = String::from("hello");

    take_ownership(s); // move
    // println!("{s}"); // error[E0382]: borrow of moved value: `s`

    let x = 5;

    make_copy(x); // copy
    println!("{x}");
}

fn take_ownership(s: String) {
    println!("{s}");
}

fn make_copy(x: u32) {
    println!("{x}");
}
```

#### 반환값과 스코프

```rs
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // move
} // s1 and s3 dropped, s2 is moved

fn gives_ownership() -> String {
    String::from("yours") // move
}

fn takes_and_gives_back(s: String) -> String{
    s // move
}
```

- 아래와 같이 튜플을 이용함면 함수의 결괏값과 매개변수의 소유권을 다시 획득할 수 있다.
- 참조자를 사용하면 소유권 이동 없이 값을 사용할 수 있다.

```rs
fn main() {
    let s = String::from("hello");
    let (len, s) = get_length(s);
    println!("\"{s}\" is {len} letters");
}
fn get_length(s: String) -> (usize, String) {
    // (s, s.len())으로 반환하면 s가 먼저 이동하기 때문에 s.len()에서 에러가 발생한다.
    (s.len(), s)
}
```

## 4.2. 참조와 대여

- `참조자(reference, &)`: 해당 주소에 저장된 데이터에 접근할 수 있도록 해주는 주솟값에 해당하는, 포인터와 같은 것
- 포인터와의 차이점은 참조자는 살아 있는 동안 특정 타입에 대한 유효한 값을 가리킴을 보장한다.
- 참조의 반대는 `역참조(dereferencing)`라 하고 `*`기호를 사용한다.
- `대여(borrowing)`: 참조자를 만드는 행위

```rs
fn main() {
    let s0 = String::from("hello");

    // `&s0`는 `s0`를 참조하지만 소유권을 가지지 않는 참조자를 생성
    let len = get_length(&s0);
    println!("The length of '{s}' is {len}.");
}

fn get_length(s1: &String) -> usize {
    s1.len()
} // s1은 참조자 타입으로 값을 소유하지 않아 drop하지 않는다.
```

```text
      stack mem (s1)              stack mem (s0)              heap mem
  +----------+---------+      +----------+---------+      +-------+-------+
  |   name   |  value  |      |   name   |  value  |      | index | value |
  +----------+---------+      +----------+---------+      +-------+-------+
  | pointer  |    ·----+----->| pointer  |    ·----+----->|   0   |   h   |
  +----------+---------+      |  length  |    5    |      |   1   |   e   |
                              | capacity |    5    |      |   2   |   l   |
                              +----------+---------+      |   3   |   l   |
                                                          |   4   |   o   |
                                                          +-------+-------+
```

### 4.2.1. 가변 참조자

- 참조자 역시 기본적으로 불변성을 지닌다.
  > error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
- `&mut {type}`으로 가변 참조자를 생성할 수 있다.

```rs
fn main() {
    let mut s = String::from("hello");

    change(r1);
}
fn change(s: &String) {
    s.push_str(", world");
}
```

- 러스트는 아래와 같은 가변 참조에 대한 제약으로 `데이터 경합(data race)`을 방지함
  - 데이터 경합은 다음 세 상황이 겹칠 때 일어날 수 있는 특수한 `경합 조건(race condition)`
  - 둘 이상의 포인터가 동시에 같은 데이터에 접근
  - 포인터 중 하나 이상이 데이터 쓰기 작업을 수행
  - 데이터 접근 동기화 메커니즘이 없음
- **In Rust, you can either have many immutable references, or one mutable reference.**
  - 동시에 여러 개의 불변 참조는 가질 수 있다.
  - 가변 참조는 한번에 오직 하나만 가질 수 있다. 가변 참조를 사용하는 동안 같은 변수를 가리키는 불변 참조도 함께 존재할 수 없다.
  - 가변 참조를 사용하려면 먼저 선언된 불변 참조들의 유효 범위(scope)가 끝나야한다.

```rs
let mut s = String::from("hello");
let r1 = &mut s;

// 가변 참조 r1의 생존 동안 불변 참조 r2를 생성할 수 없음
// error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
let r2 = &s;
r1;
```

```rs
let mut s = String::from("hello");
let r1 = &mut s;

// 가변 참조 r1의 생존 동안 s를 가변 변수로 사용할 수 없음
// error[E0499]: cannot borrow `s` as mutable more than once at a time
s.push_str(", world");
r1;
```

```rs
let mut s = String::from("hello");
let r1 = &s; // 문제 없음
let r2 = &s; // 문제 없음
println!("{r1} {r2}");
// 이 지점 이후 변수 r1, r2는 사용되지 않는다.

let r3 = &mut s; // 문제 없음
println!("{r3}");
```

### 4.2.2. 댕글링 참조

- `댕글링 포인터(dangling pointer)`: 어떤 메모리를 가리키는 포인터가 남아있는 상황에서 일부 메모리를 해제해버림으로써, 다른 개체가 할당받았을지도 모르는 메모리를 참조하기된 포인터
  - 댕글링 포인터(dangling pointer)란, 더 이상 유효하지 않은 메모리를 가리키는 포인터를 말한다. 예를 들어 C나 C++ 같은 언어에서 동적 할당된 메모리를 해제(free, delete)한 뒤에도 그 메모리를 가리키던 포인터를 계속 사용하면, 그 포인터는 댕글링 포인터가 된다. 이런 포인터를 사용하면 예측 불가능한 결과(Undefined Behavior)가 발생할 수 있다.
  - 러스트에서는 소유권(Ownership)과 빌림(Borrowing) 규칙을 통해, 이러한 댕글링 포인터 문제를 컴파일 타임에 원천적으로 방지하고 있다.

```rs
fn main() {
    let dangling_pointer = dangle();
}

// error[E0106]: missing lifetime specifier
// 이 함수는 빌린 값을 반환하고 있으나, 빌린 실젯값이 스코프가 끝나 존재하지 않는다.
fn dangle() -> &String {
    let s = String::from("hello");

    &s;
} // `s`가 drop 되어 메모리가 해제된다.

// 옳은 예시
fn dangle() -> String {
    let s = String::from("hello");
    s;
} // 값이 move되어 소유권이 이동한다.
```

- 단 하나의 가변 참조자만 갖거나, 여러개의 불변 참조자를 가질 수 있다.
- 참조자 값은 항상 유요해야한다.

## 4.3. 슬라이스 타입

- `슬라이스(slice)`: 컬렉션(collection)을 통째로 참조하는 것이 아닌, 컬렉션의 연속된 일련의 요소를 참조한다. 슬라이스는 참조자의 일종으로 소유권을 갖지 않는다.

### 4.3.0. 유효성이 보장되지 않는 예시

```rs
fn get_first_word_end_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return i;
        }
    }
    s.len()
}
```

- 위 함수는 문자열에서 첫번째 단어의 마지막 인덱스를 반환한다.
- 위 함수는 반환된 인덱스에 대한 유효성을 보장하지 못한다.
  - 위 함수에는 문법적인 오류는 없으나 의미적으로 오류가 발생할 수 있고 관리점이 늘어나는 문제가 있다.
  ```rs
  let mut s = String::from("hello, world!");
  let end_index = get_first_word_end_index(&s); // 6 반환
  s.clear(); // s = ""
  // end_index는 의미없는 값이 되며 유효성이 보장되지 않아 s[end_index]는 오류가 발생하는 코드가 된다.
  ```

### 4.3.1. 문자열 슬라이스

- 문자열 슬라이스: String의 일부를 가리키는 참조자
- `&s[start_index..end_index]`로 선언하며 범위는 `[start_index, end_index)`다.
- `start_index`가 0인 경우 생략 가능하고 `end_index`가 `s.len()`인 경우 생략 가능하다.
  - `&s[0..end_index]` = `&s[..end_index]`
  - `&s[start_index..s.len()]` = `&s[start_index..]`
  - `&s[0..s.len()]` = `&s[..]` = `s&` = `s`;
- 문자열 슬라이스는 참조자이므로 유효성이 컴파일러에 의해 보장된다.

```text
      stack mem (s)                  heap mem
  +----------+---------+         +-------+-------+
  |   name   |  value  |         | index | value |
  +----------+---------+         +-------+-------+
  | pointer  |    ·----+-------->|   0   |   h   |
  |  length  |    5    |         |   1   |   e   |
  | capacity |    5    |         |   2   |   l   |
  +----------+---------+         |   3   |   l   |
                                 |   4   |   o   |
                                 |   5   |   ,   |
                                 |   6   |       |
  stack mem (&s[7..12])     +--->|   7   |   w   |
  +----------+---------+    |    |   8   |   o   |
  |   name   |  value  |    |    |   9   |   r   |
  +----------+---------+    |    |   10  |   l   |
  | pointer  |    ·----+----+    |   11  |   d   |
  |  length  |    5    |         |   12  |   !   |
  +----------+---------+         +-------+-------+
```

```rs
let mut s = String::from("hello, world!");
let world = &s[7..12]; // "world"

// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
// 문자열 슬라이스 불변 참조자인 `world`가 살아있는 동안 가변 참조자를 생성할 수 없다. `s.clear()` 내부에서 가변 참조자가 사용되므로 에러
s.clear();
world;
```

### 4.3.2. 문자열 슬라이스 추가 정보

- 위 [4.1.2. 메모리와 할당](#412-메모리와-할당)에서 문자열 리터럴은 바이너리 실행 파일에 하드코딩된다했다.
- 문자열 리터럴을 가리키는 변수는 바이너리의 특정 지점을 가리키는 슬라이스다.
  - `let s = "Hello, world!"`에서 `s`는 슬라이스다.
- 문자열 리터럴을 변경할 수 없는 이유는 `s`가 슬라이스 `&str`타입이므로 불변 참조자이기 때문이다.
- 문자열 슬라이스(`&str`)는 `&String` 문자열 참조자 타입의 슈퍼 타입으로 생각할 수 있다.

  ```rs
  let literal = "Hello, world!";
  // 문자열 리터럴의 일부 혹은 전체 슬라이스를 `&str`로 참조할 수 있다.
  let slice: &str = &literal[7..12];
  let slice: &str = &literal[..];
  let slice: &str = literal; // 문자열 리터럴 자체가 슬라이스다.

  let string = String::from(literal);
  // 문자열의 일부 혹은 전체 슬라이스를 `&str`로 참조할 수 있다.
  let slice: &str = &string[7..12];
  let slice: &str = &string[..];
  let slice: &str = &string; // 슬라이스는 참조자이므로 가능하다.
  ```

### 4.3.3. 컬렉션의 슬라이스

- 슬라이스는 문자열 뿐만아니라 모든 컬렉션에서 사용가능하다.

```rs
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3]; // &[i32] 타입이다.
assert_eq!(slice, &[2, 3]);
```

## 정리

메모리에 저장된(또는 할당된) 자원의 생명주기가, 그것을 소유하고 있는 스택 변수(바인딩)의 생명주기에 의해 결정된다. 그리고 스택 변수가 스코프에서 벗어남과 동시에(혹은 명시적인 drop 호출 시점에) 자원이 해제된다.

- `소유권(Ownership)`
  - 어떤 값이든, 어느 시점에 오직 하나의 소유자(Owner)만 존재할 수 있다.
  - 소유자는 주로 스택에 있는 변수를 가리킨다.
- 스코프를 벗어나면(`drop`) 메모리 해제
  - 소유자가 스코프를 벗어나는 순간 drop이 자동으로 호출되며, 값에 할당된 메모리가 해제된다.
  - String처럼 힙에 실제 데이터가 저장되는 타입은, 스택에 있는 소유자 변수가 스코프에서 벗어날 때 힙 데이터도 같이 정리된다.
- 복사(`Copy`), 이동(`Move`), 빌림(`Borrow`) 등 다양한 방식의 메모리 관리
  - 단순 스택 복사가 가능한 유형(예: 정수, 부동소수점, bool 등)은 Copy 트레이트가 구현되어 있어서 복사 시에도 원본이 유효하다.
  - 반면 String 같은 타입은 이동(Move)의 개념이 적용되어, 변수를 다른 변수에 할당하면 원본 변수의 소유권이 넘간다.
  - 빌림(Borrow)을 통해서는, 소유권을 넘기지 않고도 참조만 사용하는 것이 가능하다(&/&mut).
- 슬라이스는 참조자다.
