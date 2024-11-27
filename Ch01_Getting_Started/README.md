# Chapter 1. 시작해봅시다.

## 학습 목표

> - 운영체제 별 러스트 설치방법
> - Hello, World! 프로그램 작성
> - 러스트 패키지 매니저 `cargo` 사용법

## 1.1. 러스트 설치

`rustup`: 러스트 버전 컨트롤 CLI

linux & macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# rust 업데이트
rustup update
# rust 삭제
rustup self uninstall
# local 문서
rustup doc
```

## 1.2. Hello, world

`hello_world.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```

```bash
rustc hello_world.rs # 컴파일
./hello_world.rs # 실행
```

rust는 탭 대신 스페이스 4칸을 사용한다.

`println!`은 매크로(macro) 호출 코드다. 매크로는 항상 함수와 같은 규칙을 따르지 않는다.

컴파일 결과 `hello_world` 실행할 수 있는 바이너리 파일(실행파일) 생성.
윈도우의 경우 `hello_world.exe`와 `hello_world.pdb` 파일이이 생성된다. `.pdb`에는 디버깅정보가 포함된다.

## 1.2. Cargo

카고는 러스트 빌드 시스템 및 패키지 매니저
빌드, 외부 라이브러리 다운로드, 라이브러리 제작 등 디펜던시 매니저

```bash
cargo new hello_cargo

cargo build # 빌드
./target/debug/hello_cargo # 실행

cargo run # 빌드 | 실행

cargo check # 빌드하지 않고 검사만 수행

cargo build --release # 릴리즈 빌드
```

`cargo`로 프로젝트를 초기화 하면 .gitignore 생성과 함께 git 저장소가 초기화 되지만 이미 git저장소가 있는 디렉터리에서는 생성되지 않는다.
`cargo new --vcs=git`으로 덮어써 생성할 수 있다.
참고: `cargo new --help`

`TOML(Tom's Obvious, Minimal Language)`포멧으로 카고 설정을 작성한다.

러스트에서는 코드 패키지를 `크레이트(crate)`라 부른다.

카고는 소스파일이 `src` 디렉터리 내부에 있다고 예상한다. 최상위에는 README, 라이센스, 설정파일 등 소스코드와 관련없는 파일들을 저장하는데 사용된다.

`cargo build` 명령어는 기본 빌드가 디버그 빌드기 때문에 현재 디렉터리가 아닌 `target/debug` 디렉터리에 실행파일을 생성한다.

`cargo run` 명령어는 소스코드에 변경사항이 없다면 빌드 과정을 스킵하고 실행파일을 바로 실행한다.

`cargo check` 명령어는 빌드를 하지 않고 코드를 검사한다. 빌드를 하지 않기 때문에 빠르게 코드를 검사할 수 있다.

`cargo build --release` 명령어는 릴리즈 빌드를 수행한다. 릴리즈 빌드는 최적화가 적용되어 프로그램 실행 속도는 빠르지만 빌드 시간이 오래 걸린다.
target/debug 디렉터리가 아닌 target/release에 실행 파일이 생성된다.
