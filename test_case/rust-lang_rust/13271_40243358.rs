
enum Foo {
    Bar = 1,
    Baz = 2
}

#[packed]
struct S3_Foo {
    a: u8,
    b: u16,
    c: Foo
}

static TEST_S3_Foo: S3_Foo = S3_Foo { a: 1, b: 2, c: Baz };

pub fn main() {
    println!("{:?}", TEST_S3_Foo);
}
