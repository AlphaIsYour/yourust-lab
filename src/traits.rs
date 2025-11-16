pub fn run() {
    println!("--- Traits ---");

    trait Greet {
        fn greet(&self);
    }

    struct Person {
        name: String,
    }

    impl Greet for Person {
        fn greet(&self) {
            println!("Hello, {}!", self.name);
        }
    }

    let p = Person { name: String::from("Alpha") };
    p.greet();
}
