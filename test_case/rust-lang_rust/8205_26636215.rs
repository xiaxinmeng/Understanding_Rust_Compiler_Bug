 rust
fn main() {
    for _ in range(0, 3) {
        do spawn {
            for _ in range(0, 3) {
                println("hi")
            }
        }
    }
}
