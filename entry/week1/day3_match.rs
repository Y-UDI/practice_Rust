fn main() {
    let score = 2;

    match score {
        1 => println!("low"),
        2 => println!("middle"),
        3 => println!("high"),
        _ => println!("other")
    }

    let level = match score {
        1 => "low",
        2 => "middle",
        3 => "high",
        _ => "other"
    };

    println!("{}", level);
}
