pub fn run() {
    println!("--- Variables ---");
    let x = 5;
    let mut y = 10;
    const PI: f64 = 3.1415;

    println!("x = {}, y = {}, PI = {}", x, y, PI);
    y += 5;
    println!("y after mutation = {}", y);
}
