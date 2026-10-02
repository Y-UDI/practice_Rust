use std::print;

fn show(text: &String) {
    println!("{}", text);
}

fn main() {
    let message: String = String::from("Hello");

    show(&message); // &をつけると参照になる

    println!("{}", message);

}

    
fn main() {
    let message: String = String::from("Hello");

    let a = &message;
    let b = &message;
    let mut c  = &message; // これはだめ
    // コンパイルできる。
}

fn main() {
    let mut message: String = String::from("Hello");

    let mut a = &message;
    let mut b = &message;
    // コンパイルできない。
}

fn main() {
    let mut message: String = String::from("Hello");

    let a = &message;
    let b = &mut message;
    // コンパイルできない。
}

fn main() {
    let mut message: String = String::from("Hello");

    let a = &message;
    let b = &message;
    // コンパイルできる。
}

fn main() {
    let mut message: String = String::from("Hello");

    let a = &message;

    println!("{}", a);

    let b = &mut message;
    // コンパイルできる。借用は使うまで
}