 rust
#[derive(Debug)]
enum A<T> {
    A1(T),
    A2(Box<A<B<T>>>)
}

#[derive(Debug)]
enum B<T> {
    B(T)
}

fn main() {
    let wub = A::A2(Box::new(A::A1(B::B("wub"))));

    println!("{:?}", wub);
}
