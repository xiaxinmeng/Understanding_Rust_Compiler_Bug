 rust
trait Foo {
    lifetime b; // or is this supposed to be `lifetime 'b;`?
    fn foo(&'Self::b self); // unsure if this is correct
}
