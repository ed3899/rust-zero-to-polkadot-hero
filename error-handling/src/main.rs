fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[99]); // ❌ panics at runtime
}