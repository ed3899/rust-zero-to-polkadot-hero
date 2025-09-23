fn main() {
    let x = 5;
    println!("x = {x}");

    // This will fail unless you use `mut`
    let mut y = 10;
    y = y + 5;
    println!("y = {y}");
}
