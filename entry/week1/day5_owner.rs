fn main() {
    let a = 10;
    let b = a; //数値型はOK

    println!("{}", a);
    println!("{}", b);
}

fn main() {
    let a = String::from("Rust");
    let b = a; // Stringは代入すると値が移る(Move)のでaはつかえなくなる
    // b: String = a.clone() ならいける

    println!("{}", a);
    println!("{}", b);
}

// コンパイルできない例
fn show(text: String) {
    println!("{}", text);
}

fn main() {
    let message = String::from("Hello");

    show(message); // text = message になるイメージ？

    println!("{}", message);
}