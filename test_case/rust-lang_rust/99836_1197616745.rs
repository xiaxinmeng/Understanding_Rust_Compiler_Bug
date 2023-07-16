rust
fn main() {
    #[derive(Debug)]
    #[allow(unused)]
    enum Error { A, B }

    type Foo = Result<[usize; 0], Error>;

    dbg!(std::mem::size_of::<Foo>());
    dbg!(std::mem::align_of::<Foo>());
    
    let x: [Foo; 2] = [Ok([]), Ok([])];
    let r0: &[usize] = x[0].as_ref().unwrap();
    let r1: &[usize] = x[1].as_ref().unwrap();
    eprintln!("{r0:p} {r1:p}");
}
