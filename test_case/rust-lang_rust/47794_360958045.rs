rust
union DummyUnion {
    field1: i32,
    field2: i32,
}

const UNION: DummyUnion = DummyUnion { field1: 1 };

fn read_field1() -> i32 {
    const FIELD1: i32 = unsafe { UNION.field1 };
    FIELD1
}

fn main() {
    read_field1();
}
