
fn main() {
    println!("Example2");
    let mut a: Vec<i32> = Vec::new();
    for n in 1..=5 {
        println!("n={}", n);
        a.push(n);
    }
    println!("a3={}",a[3]);
}
