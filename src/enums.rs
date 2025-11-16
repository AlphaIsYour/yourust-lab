pub fn run() {
    println!("--- Enums ---");

    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;

    match dir {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        _ => println!("Other direction"),
    }
}
