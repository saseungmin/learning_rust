# lifetimes
라이프타임은 어떤 타입이 원하는 동작이 구현되어 있음을 보장하기보다는, 어떤 참조자가 필요한 기간 동안 유효함을 보장하도록 한다.   

러스트에서는 모든 참조자는 라이프타임이라는 참조자의 유효성을 보장하는 범위를 갖는다.   

타입이 추론되듯, 라이프타임도 암묵적으로 추론된다.   
타입을 명시해줘야하는 상황이 있듯, 참조자의 lifetime이 여러 방식으로 서로 연관될 수 있는 경우에는 lifetime을 명시해야한다.   

라이프타임의 주 목적은 dangling reference 방지이다. dangling reference는 프로그램이 참조하려고 한 데이터가 아닌 엉뚱한 데이털르 참조하게 되는 원인이다.

```rust
fn main() {
  let r;  // r의 라이프타임 start
  { // x의 라이프타임 start
    let x = 5;
    // r은 바깥쪽 스코프에서 유효하지만, x는 안쪽 스코프가 끝남.
    r = &x;
  } // x의 라이프타임 end

  // r의 참조하는 값이 사용하려는 시점에 이미 자신의 스코프를 벗어났기 때문에 컴파일 되지 않음.
  println!("r: {}", r);
  // r의 라이프타임 end
}
```

러스트에서 코드가 유효한지 판단하는 방법으로 borrow checker(대여 검사기)를 이용.   
러스트 컴파일러는 borrow checker로 스코프를 비교하여 대여의 유효성을 판단한다.

```rust
// r보다 x의 라이프타임이 더 김.
fn main() {
  let x = 5;
  let r = &x;
  println!("r: {}", r);
}
```

라이프타임을 명시하는 이유는 여러 참조자에 대한 수명에 영향을 주지 않으면서 서로 간 수명의 관계가 어떻게 되는지에 기술하는게 목적. (ex. 함수 시그니처에 제네릭 타입 메개변수 작성)   
보통 제네릭 타입처럼 라이프타임 명시도 짧은 소문자로 정하고, 첫 번째 라이프타임을 명시할 때 `'a`를 사용.   

```rust
&i32 // 참조자
&'a i32 // 명시적인 라이프타임이 있는 참조자
&'a mut i32 // 명시적인 라이프타임이 있는 가변 참조자
```

```rust
// 명시적으로 시그니처 내의 모든 참조자가 동일한 라이프타임 'a를 가져야함
// 두 매개변수는 라이프타임 'a만큼 살아 있는 문자열, 반환하는 문자열도 라이프타임 'a만큼 살아 있다.
// 실제 의미는 함수가 반환되는 참조자의 라이프타임은 함수 인수로서 참조된 값들의 라이프타임 중 작은 것과 동일하다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```

시그니처에 라이프타임 매개변수를 지정한다고해서, 전달되는 값이나 반환값의 라이프타임이 변경되는 건 아님.   
어떤 값이 제약 조건을 지키지 않았을 때 borrow checker(대여 검사기)가 검사하고 판단만 할 수 있도록 명시만 할 뿐이다.

```rust
fn main() {
  // string1 라이프타임 start
  let string1 = String::from("long string is long");

  {
    // string2 라이프타임 start
    let string2 = String::from("xyz");
    // result는 안쪽 스코프가 끝나기전까지 유효
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longes string is{}", result);
    // string2 라이프타임 end
  }

  // string1 라이프타임 end
}
```

`result`가 유효하려면 `string2`가 바깥쪽 스코프가 끝나기 전까지 유효해야함. 함수 매개변수와 반환값에 모두 동일한 라이프타임 매개변수 `'a`를 명시했으므로, 러스트는 문제를 더 정확히 파악할 수 있다.

```rust
fn main() {
  // string1 라이프타임 start
  let string1 = String::from("long string is long");
  let result;

  {
    // string2 라이프타임 start
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    // string2 라이프타임 end
  }

  println!("The longes string is{}", result);
  // string1 라이프타임 end
}
```

참조자를 반환하는 함수를 작성할 때는 반환 타입의 라이프타임 매개변수가 함수 매개변수 중 하나와 일치해야한다.

```rust
// result는 함수가 끝나는 시점에 스코프를 벗어나므로 댕글링 참조가 되어 컴파일 되지 않음.
// 참조자 대신 값의 소유권을 갖는 데이터 타입을 반환하여 함수를 호출한 함수 측에서 값이 정리하도록 하여 문제를 해결할 수 있음.
fn longest<'a>(x: &str, y: &str) -> &'a str {
  let result = String::from("really long string");
  result:as_str();
}
```

구조체의 라이프타임

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

라이프타임 생략   
- 초반 러스트는 생략되지 않았음.
1. 매개변수가 하나인 함수는 lifetime 매개변수 하나
   - `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
2. 정확히 하나의 입력 lifetime 매개변수가 있는 경우 해당 lifetime이 모든 출력 lifetime 매개변수에 할당
   - `fn foo<'a>(x: &'a i32) -> &'a i32`
3. 메소드인 경우 여러 개의 입력 lifetime 매개변수가 있지만 그 중 하나가 메소드 `&self`이거나 `&mut self`메소드이기 때문에 모든 출력 lifetime 매개변수에 lifetime이 `self` 할당


```rust
fn first_word(s: &str) -> &str {
fn first_word<'a>(s: &'a str) -> &'a str {

fn longest(x: &str, y: &str) -> &str {
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

정적 라이프타임은 `'static` 라이프타임은 해당 참조자가 프로그램의 전체 생애주기 동안 살아 있음을 의미. 모든 문자열 리터럴은 `'static` 라이프타임을 가진다.

```rust
let s: &'static str = "I have a static lifetime.";
```

어떤 참조자를 `'static`으로 지정하기 전에 해당 참조자가 반드시 프로그램의 전체 라이프타임동안 유지되여야만 하는지, 고민해야함.   
`'static` 라이프타임을 제안하는 에러메시지는 대부분 댕글링 참조를 만들거나 라이프타임이 잘못 짝지어져있는 경우이기때문에 잘 사용해야함.
