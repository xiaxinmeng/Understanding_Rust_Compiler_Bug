rust
struct Wrapper(String);

impl From<Wrapper> for String {
    fn from(w: Wrapper) -> String {
        w.0
    }
}

fn main() {}
