pub fn run() {
    println!("--- Functions ---");
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let result = add(5, 7);
    println!("5 + 7 = {}", result);
}
