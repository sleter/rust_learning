// opóźniona inferencja typów w języku Rust
fn main() {
    letmut vec = Vec::new();
    vec.push(42); // dopiero tutaj odbywa się
                  // dedukcja typu
}