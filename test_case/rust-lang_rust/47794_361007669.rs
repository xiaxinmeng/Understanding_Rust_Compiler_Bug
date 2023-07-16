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

fn read_field2_1() -> u32 {
    const FIELD2_1: u32 = unsafe { UNION.field2.1 };
    FIELD2_1
}

fn main() {
    assert_eq!(read_field2_1(), 0xdeadbeef);
}
