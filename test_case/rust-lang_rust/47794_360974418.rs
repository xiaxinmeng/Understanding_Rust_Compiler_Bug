rust
type Field1 = u64;
type Field2 = (u32, u32);

union DummyUnion {
    field1: Field1,
    field2: Field2,
}

const UNION: DummyUnion = DummyUnion {
    field1: 0xdeadbeeff00fb00f,
};

fn read_field1() -> Field1 {
    const FIELD1: Field1 = unsafe { UNION.field1 };
    FIELD1
}

fn read_field2() -> Field2 {
    const FIELD2: Field2 = unsafe { UNION.field2 };
    FIELD2
}

fn main() {
    assert_eq!(read_field1(), 0xdeadbeeff00fb00f);

    let (first, second) = read_field2();
    assert_eq!(first, 0xf00fb00f);
    assert_eq!(second, 0xdeadbeef);
}
