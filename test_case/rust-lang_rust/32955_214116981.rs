
fn main() {
    struct FooVtable {
        destructor: fn(*mut ()),
        size: usize,
        align: usize,
        method: fn(*const ()) -> String,
    }

    fn drop(to_drop: *mut ()) {}

    // u8:

    fn call_method_on_u8(x: *const ()) -> String {
        // the compiler guarantees that this function is only called
        // with `x` pointing to a u8
        let byte: &u8 = unsafe { &*(x as *const u8) };

        byte.to_string()
    }

    static Foo_for_u8_vtable: FooVtable = FooVtable {
        destructor: drop,
        size: 1,
        align: 1,

        // cast to a function pointer
        method: call_method_on_u8 as fn(*const ()) -> String,
    };


    // String:

    fn call_method_on_String(x: *const ()) -> String {
        // the compiler guarantees that this function is only called
        // with `x` pointing to a String
        let string: &String = unsafe { &*(x as *const String) };

        string.to_string()
    }

    static Foo_for_String_vtable: FooVtable = FooVtable {
        destructor: drop,
        // values for a 64-bit computer, halve them for 32-bit ones
        size: 24,
        align: 8,

        method: call_method_on_String as fn(*const ()) -> String,
    };
}
