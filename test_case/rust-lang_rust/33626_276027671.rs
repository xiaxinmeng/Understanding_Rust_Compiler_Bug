
struct Align16(i32);

enum Enum { // enum should be align 16
    A(i32),
    B(Align16) // data should be offset to align 16
}

struct Nested { // struct should be align 16
    a: i32,
    b: Align16, // data should be offset to align 16
}
