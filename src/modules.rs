pub fn run() {
    println!("--- Modules ---");

    mod submodule {
        pub fn hello() {
            println!("Hello from submodule!");
        }
    }

    submodule::hello();
}
