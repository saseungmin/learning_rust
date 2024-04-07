# week2

### 5. primitive_type

- char 타입의 메서드

```rust
let my_first_initial = 'C';

my_first_initial.is_alphabetic() // 알파벳인지 아닌지 확인: boolean
my_first_initial.is_numeric() // 숫자인지 아닌지
```

- Array
- 같은 타입인 경우에만 사용가능. `[타입, 길이]`

```rust
use std::mem;

fn main() {
    let a: [i32; 500] = [0; 500];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }

    println!("Array occupies {} bytes", mem::size_of_val(&a)); // 메모리 할당이 얼마나되어있는지 확인가능.
}
```

- 배열 자르기 확인

```rust
#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
```

- 튜플은 다양한 유형의 값 모음
- 값 사용시 약간 javascript의 구조분해할당 느낌.

```rust
fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
```
