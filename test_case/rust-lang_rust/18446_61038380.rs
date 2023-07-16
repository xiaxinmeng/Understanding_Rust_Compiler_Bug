
trait T {
    fn foo(&self);
}

impl<'a> T+'a {
    fn foo(&self) {}
}

impl T for int {
    fn foo(&self) {}
}

fn main() {
    let x: &T = &42i;
    x.foo(); //~ERROR
}
