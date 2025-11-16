pub fn run() {
    println!("--- Control Flow ---");
    let number = 7;

    if number < 5 {
        println!("less than 5");
    } else if number == 7 {
        println!("equal to 7");
    } else {
        println!("greater than 5");
    }

    for i in 0..3 {
        println!("for loop i = {}", i);
    }

    let mut count = 0;
    while count < 3 {
        println!("while loop count = {}", count);
        count += 1;
    }
}
