rust
trait Test {
    fn is_some(&self) -> u8;
}

impl<T> Test for Option<T> {
    fn is_some(&self) -> u8 {
        self.is_some() as u8
    }
}

async fn f() {
    let x = Some(2);
    let _: () = x.is_some(); // type is bool.
}
