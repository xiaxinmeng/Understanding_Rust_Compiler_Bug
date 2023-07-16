rust
struct Foo<#[cfg(not(unix))] H>(H);
fn main() {
    let _: Foo<u32> = Foo::<u32>(1);
}
