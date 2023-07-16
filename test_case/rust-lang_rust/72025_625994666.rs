
type validation failed: MyStruct {
    field_b: {
        [0]: TupleStruct {
            0: <expected boolean, got 0x3>,
        },
    },
    field_c: Enum::Variant {
         meh: <dangling reference (created from integer)>,
    },
}
