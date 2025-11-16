pub fn run() {
    println!("--- Error Handling ---");

    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10, 2) {
        Ok(res) => println!("10 / 2 = {}", res),
        Err(e) => println!("Error: {}", e),
    }

    match divide(5, 0) {
        Ok(res) => println!("5 / 0 = {}", res),
        Err(e) => println!("Error: {}", e),
    }
}
