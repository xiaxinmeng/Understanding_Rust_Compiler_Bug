rust
type Foo = impl Send;
struct Struct<
    const C: usize = {
        let _: Foo = ();
        0
    }
>;
