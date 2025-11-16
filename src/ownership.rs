pub fn run() {
    println!("--- Ownership ---");
    let s1 = String::from("Hello");
    let s2 = s1; // s1 move ke s2
    // println!("{}", s1); // error jika di-uncomment

    let s3 = s2.clone(); // clone untuk copy
    println!("s2 = {}, s3 = {}", s2, s3);

    let s4 = give_ownership();
    println!("s4 = {}", s4);

    fn give_ownership() -> String {
        String::from("Owned")
    }
}
