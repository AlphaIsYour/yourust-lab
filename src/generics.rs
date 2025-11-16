pub fn run() {
    println!("--- Generics ---");

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = [3, 7, 2, 9];
    println!("Largest = {}", largest(&numbers));
}
