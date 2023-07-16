rust
struct Bar(i32);
impl !Sync for Bar {}

const C: &'static Bar = &Bar(40);

fn main() {}
