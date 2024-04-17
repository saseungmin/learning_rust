# week3

### strings

```rust
fn main() {
    let word = String::from("green"); // Try not changing this line :)
    // if is_a_color_word(&word) {
    if is_a_color_word(word.clone()) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

// fn is_a_color_word(attempt: &str) -> bool {
fn is_a_color_word(attempt: String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
```

두가지 방법으로 수정이 가능한데, `&word`는 `String`의 참조를 전달하게 되어, `is_a_color_word` 함수에서도 해당 `String`을 참조하여 사용할 수 있게 된다.   
Rust에서는 가능한 경우에는 소유권을 이전하지 않고도 데이터에 접근할 수 있도록 참조를 사용하는 것이 권장. 그래서 `word.clone()`을 통해 main 함수 내에서 `word` 변수의 소유권을 보존하는 것이 좋음.   

문자열 compose 하는 방법

```rust
fn compose_me(input: &str) -> String {
  input.to_string() + " world!"
  format!("{} world!", input)
}
```

- `to_owned()`
  - 메서드는 문자열을 소유하는 `String` 타입으로 변환하는 메서드
- `into()`
  - `into()` 메서드는 Rust에서 소유권을 전환하는 역할을 함. 이 메서드는 원래 값의 소유권을 포기하고, 대신 새로운 타입의 소유권을 얻게 된다. 주로 타입 변환시 사용.
  - `into()` 메서드는 `to_owned()` 메서드와 유사하게 동작하지만, `to_owned()`는 명시적으로 `String` 타입으로 변환하는 데 사용되는 반면, `into()`는 일반적으로 컴파일러에게 타입을 추론하도록 하여 코드를 더 간결하게 만듬.

```rust
let str_literal: &str = "nice weather";
let owned_string: String = str_literal.into();
```

### modules

### hashmaps

### options
