
trait Bar {
    const X: usize;
    fn return_n(&self) -> [u8; Bar::X];
}

//fn main() {}
