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

### lifetimes
