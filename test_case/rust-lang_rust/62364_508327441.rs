rust
struct Struct<P1> {
    field: P1,
}

type Alias<'l1, P2> = Struct<*const Self>;
