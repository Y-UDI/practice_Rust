fn main() {
    let language = "Rust";
    let total_week = 15;
    let week = 1;

    println!("{}学習", language);
    println!("全{}週間", total_week);
    println!("現在 week {}", week);

    let total_week = total_week - 1;
    let week = week + 1;

    println!("残り{}週間", total_week);
    println!("次は week {}", week);
}
