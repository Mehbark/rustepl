fn main() {
    println!("Hello, world!");
    "hello".chars().for_each(|c| println!("{}", c));
    let h = "h".repeat(100);
    println!("{}", h);
}
