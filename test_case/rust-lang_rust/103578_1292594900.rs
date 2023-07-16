rust
struct S; // type + value (constructor constant)
struct S(u8); // type + value (constructor function)
struct S { field: u8 } // only type

enum E {
    V, // type + value (constructor constant)
    V(u8), // type + value (constructor function)
    V { field: u8 }, // type + value (???)
}
