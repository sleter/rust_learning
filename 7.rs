// Przykład formatowania stringów w języku Rust

use std::fmt;

struct KeyValue<T>
where
    T: fmt::Display,
{
    key: String,
    value: T,
}

impl<T> fmt::Display for KeyValue<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.key, self.value)
    }
}

fn main() {
    let kv = KeyValue {
        key: String::from("answer"),
        value: 42,
    };
    println!("{}", kv);
}