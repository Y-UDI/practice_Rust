// cannot assigne twice to imutable variablesというエラーがでる
// let で定義する変数は不変だから、あとから変更できない
fn main() {
    let x = 10;
    println!("Step1 : {x}");
    x = 20;
    println!("Step2 : {x}")
}