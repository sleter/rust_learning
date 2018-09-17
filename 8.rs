// Przesłanianie stałych 

fn get_data() -> String {
    String::from("42")
}

fn main() {
    let value = get_data();
    let value = value.parse::<i32>().unwrap();
    // value = 42; // error!
    println!("{:04}", value);
}