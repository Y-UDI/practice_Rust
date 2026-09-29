fn main() {
    for number in 1..=10 {
        // 3の倍数は表示しない
        if number % 3 == 0 {
            continue;
        }
        if number >= 8 {
            break;
        }
        // 8になったらループを終了する

        println!("{}", number);
    }
}