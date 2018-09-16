fn main() {
    // Przykład borrow checker
    let mut a = 42;
    {
        let b = &mut a;
        //println!("{}", a); // error!
    }
    println!("{}", a);
}