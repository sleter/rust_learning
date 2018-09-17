// Zwracanie wartości z funkcji bez instrukcji return 

use std::ops::Mul;

fn square<T>(v: T) -> T::Output
where
    T: Mul + Copy,
{
    v * v
}
fn main() {
    println!("{:.2}", square(6.480745));
}