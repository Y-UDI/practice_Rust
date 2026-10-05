fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];

    let a = &numbers[..3];
    let b = &numbers[4..];

    println!("{:?}", a); // リストの表示は{:?}がひつよう
    println!("{:?}", b);
}