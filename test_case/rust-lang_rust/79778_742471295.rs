rust
trait Foo<const N: usize> {
    const FOO: usize = bar(N);   
    fn foo() -> [u8; bar(N)] where [u8; bar(N)]: Sized;
}
