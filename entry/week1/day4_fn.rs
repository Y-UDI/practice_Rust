fn multiply(a: i32, b: i32) -> i32 {
    a * b //セミコロンをつけると返り値じゃなくなる
}


fn multiply(a: i32, b: i32) -> i32 {
    return a * b; //return をつけるならOK
}


fn main() {
    let result = multiply(4, 5);
    println!("{}", result);
}