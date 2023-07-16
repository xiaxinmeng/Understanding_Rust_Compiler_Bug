rust
// src/main.rs
fn main() {
    macro_rules! mac {
        ($e:expr) => {
            #[tokio::main]
            async fn f() -> i32 {
                $e(())
            }
        };
    }
    mac!(|_| 5);
}
