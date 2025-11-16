pub fn run() {
    println!("--- Collections ---");

    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("Vector: {:?}", v);

    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 40);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
