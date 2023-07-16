
struct Foo<A, B=i32, C=i64>(A, B, C);

pub fn foo(_x: Foo<bool>, _y: Foo<u32, bool>, _z: Foo<u32, _, char>) {}