// Traity i domyślne implementacje funkcji 

// Zamiast dziedziczenia traity, które są jako 
// interfejsy pozwalające na domyślną implementacje

trait HasArea<T> {
    fn area(&self) -> T;
}

trait HasName {
    fn name(&self) -> String {
        String::from("Unknown")
    }
}

struct Circle {
    radius: f64,
}

impl HasArea<f64> for Circle {
fn area(&self) -> f64 {
        self.radius * self.radius * 3.1415926535
    }
}

impl HasName for Circle {}

fn main() {
let c = Circle { radius: 10.0 };
    println!("{}", c.area());
    println!("{}", c.name());
}