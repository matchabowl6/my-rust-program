fn main() {
    println!("Hello, world!");
    let number= sum(40.9, 59.2);
    let number_as_string = format!("Sum: {}", number);
    println!("{}", number_as_string);
}

fn sum(a: f64, b: f64) -> f64 {
    return a + b;
}
