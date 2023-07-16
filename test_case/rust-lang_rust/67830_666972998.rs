rust
trait Empty {
}

trait T<'l> {
    type Assoc;
}

fn foo() -> impl for<'a> T<'a, Assoc = impl Empty + 'a> {
    todo!()
}
