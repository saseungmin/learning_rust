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
```rust
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}
```

- 모듈에 `pub`키워드를 사용하여 모듈을 공개해도 모듈의 내용은 비공개.
- 모듈을 공개했다고 해서 내용까지 공개되지는 않는다. 모듈의 `pub`키워드는 상위 모듈이 해당 모듈을 가리킬 수 있도록 할 뿐, 그 내부 코드에 접근하도록 하는 것은 아님. 모듈은 단순한 컨테이너이기 때문에 모듈을 공개하는 것만으로는 할 수 있는 것은 별로 없음.
- 모듈이 가지고 있는 아이템도 마찬가지로 공개해야함.
- 크레이트: 라이브러리나 실행가능한 모듈로 구성된 트리 구조로 러스트가 한 번의 컴파일 시 고려하는 가장 작은 코드 단위.

```rust
// 절대 경로: 크레이트 루트로부터 시작되는 전체 경로. 외부 크레이트로부터의 코드에 대해서는 해당 크레이트 이름으로 절대 경로가 시작되고 현재의 크레이트로부터의 코드에 대해서는 crate리터럴로부터 시작된다.
crate::sausage_factory::make_sausage();
// 상대 경로
sausage_factory::make_sausage();
```


### hashmaps

### options
