rust
struct S;

struct T;

impl PartialEq<&&&T> for &&S {
    fn eq(&self, _t: &&&&T) -> bool {
        todo!()
    }
}

fn main() {
    let s = S;
    let t = T;
    dbg!(&&s == &&&t);
}
