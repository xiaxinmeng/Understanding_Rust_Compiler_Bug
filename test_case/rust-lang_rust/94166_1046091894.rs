rust
struct Baz;

impl Into<()> for Baz {
    fn into(self) -> () { () }
}

impl Into<i32> for Baz {
    fn into(self) -> i32 { 0 }
}

fn main() {
    let _: () = vec![Baz].into_iter().map(|b| b.into()).collect();
}
