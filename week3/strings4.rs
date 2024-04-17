// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    // 문자열을 소유하는 String 타입으로 변환하는 메서드
    string("rust is fun!".to_owned());
    // into() 메서드는 Rust에서 소유권을 전환하는 역할을 함. 이 메서드는 원래 값의 소유권을 포기하고, 대신 새로운 타입의 소유권을 얻게 된다. 주로 타입 변환시 사용.
    // into() 메서드는 to_owned() 메서드와 유사하게 동작하지만, to_owned()는 명시적으로 String 타입으로 변환하는 데 사용되는 반면, into()는 일반적으로 컴파일러에게 타입을 추론하도록 하여 코드를 더 간결하게 만듬.
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
