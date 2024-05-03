# week4

### error_handling
러스트는 에러를 복구가능한 에러와 불가능한 에러로 나뉜다.   
러스트에는 예외 처리 기능이 없다. 대신, 복구 가능한 에러를 위한 `Result<T, E>` 타입과 복구 불가능한 에러가 발생했을 때 프로그램을 종료하는 `panic!`매크로가 있음.   

코드가 패닉을 일으킬 동작을 하는 것(배열 끝부분을 넘어선 접근과 길이) 혹은 `panic!` 매크로를 명시적으로 호출하는 것 두 가지로 패닉을 일으킬 수 있다.   

`Result` 열거형은 다음과 같이 `Ok`와 `Err`라는 두 개의 variant를 갖도록 정의

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

`match` 표현식을 사용하여 반환 가능한 `Result` variant를 처리

```rust
match qty {
    Ok(qty) => Ok(qty * cost_per_item + processing_fee),
    Err(e) => Err(e),
}
```

`main` 함수는 실행 프로그램의 시작점이자 종료점이기 때문에 프로그램이 기대한 대로 작동하려면 반환 타입의 종류에 대한 제약이 있다.   
때문에 `main`는 `Result<(), E>`형식으로 반환되어야함. 그리고 함수의 마지막에 반환값 `Ok(())`를 추가한다.   
0값으로 종료되고, `main`이 `Err`값을 반환할 경우 0이 아닌 값으로 종료.

```rust
fn main() -> Result<(), ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}
```

`Box<dyn Error>`타입은 트레이트 객체인데 아직 잘모르겠음... `traits`에서.. 어떠한 종류의 에러를 의미.   
이 함수 시그니처에 `Box<dyn Error>`라고 명시하면 이후 `main`의 구현체에 다른 에러들을 반환하는 코드가 추가되더라도 올바르게 동작.

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
```

### generics

```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

`impl` 바로 뒤에 `T`를 선언하여 `Wrapper<T>` 타입에 메서드를 구현한다고 명시했음을 주의. 이렇게 하면 러스트는 `Wrapper`의 부등호 기호 내 타입이 구체적인 타입이 아닌 제네릭 타입임을 인지한다.

### traits
trait는 인터페이스와 비슷   

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

// impl 뒤에 구현하고자 하는 trait 이름을 적고, 그다음 for 키워드와 trait를 구현할 타입명 명시
impl AppendBar for String {
    fn append_bar(self) -> Self {
        format!("{}Bar", self)
    }
}

```

메서드 오버라이딩하는 구현을 하면 해당 메서드의 기본 구현을 호출할 수는 없다.   
매개변수에는 지정된 trait를 구현하는 타입이라면 어떤 타입이든 전달받을 수 있다.

```rust
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
```

트레이드 바운드 문법으로 부등호 기호 안의 제네릭 타입 매개변수 선언에 붙은 콜론 뒤에 위치   

```rust
// + 문법을 사용하면 trait를 여러 개 지정할 수 있다.
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}
```

두 매개변수를 전달받은 함수가 같은 타입으로 강제되어야 한다면, 트레이드 바운드로 유용하게 사용할 수 있음.   

`where` 절로 trait bound를 정리할 수 있다.

```rust
fn some_func<T>(item: T) -> bool where T: SomeTrait + OtherTrait {
    item.some_function() && item.other_function()
}
```

### lifetimes
