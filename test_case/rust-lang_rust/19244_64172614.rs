
struct MyStruct {
    field1: uint
}

const S: MyStruct = MyStruct { field1: 10 };
const SField : uint = S.field1;

struct ConstCheck {
    broken_array: [int, ..SField],
}
