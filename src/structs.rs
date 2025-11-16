pub fn run() {
    println!("--- Structs ---");

    struct User {
        name: String,
        age: u8,
    }

    impl User {
        fn greet(&self) {
            println!("Hello, {}!", self.name);
        }
    }

    let user1 = User { name: String::from("Alpha"), age: 19 };
    println!("User age = {}", user1.age);
    user1.greet();
}
