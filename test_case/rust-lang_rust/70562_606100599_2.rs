rust
struct A;

impl From<A> for Vec<u8> {
    fn from(a: A) -> Self {
        vec![0]
    }
}

impl From<A> for Vec<u16> {
    fn from(a: A) -> Self {
        vec![1]
    }
}


fn main() {
    let a = A;
    let mut vec = a.into();
    //~^ ERROR type annotations needed
    vec.push(2u16);
}
