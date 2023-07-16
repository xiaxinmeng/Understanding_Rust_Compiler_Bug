 rust
struct A {
    a: i32
}

impl A {
    fn one<'a>(&'a mut self) -> &'a i32{
        self.a = 10;
        &self.a
    }
    fn two<'a>(&'a mut self) -> &'a i32 {
        loop {
            let k = self.one();
            if *k > 10i32 {
                return k;
            }
        }
    }
}
