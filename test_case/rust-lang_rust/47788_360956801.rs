rust
union DummyUnion {
    field1: i32,
    field2: i32,
}

const UNION: DummyUnion = DummyUnion { field1: 1 };

fn main() {
    assert_eq!(unsafe { UNION.field2 }, 1);
}
