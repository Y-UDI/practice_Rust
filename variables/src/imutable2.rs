// 不変変数を定義しても定義しなおすと上書きされる
fn main(){
    let x = 10; // 不変変数の定義
    println!("Step1 : {x}");
    let x = 20;
    println!("Step2 : {x}");
    
}