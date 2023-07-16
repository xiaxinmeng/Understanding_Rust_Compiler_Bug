
struct Test<A, T: Iterator<Item=A>>(T);

impl <A, T: Iterator<Item=A>> Drop for Test<A, T> {
    fn drop(&mut self) {  }
}

fn main() {
    let x = Test(2..);
    let _ = x;
}
