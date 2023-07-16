
#[allow(dead_code)]
fn make<'a, A: Clone, I: Iterator<Item=&'a A>>(mut it: I) -> Vec<A> {
    let mut v = vec![];
    for a in it {
        v.push(a.clone())
    }
    v
}

fn main() {}
