
struct MyStruct { field1: uint }

const S: MyStruct = MyStruct { field1: 42u };

struct ConstCheck {
    broken_1: [int, ..S.field2],
            //~^ ERROR: expected constant expr for array length: unexistent struct field
}
