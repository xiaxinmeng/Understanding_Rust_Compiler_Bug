rust
struct Foo {
    init: bool,
    data: MaybeUninit<SomeLargeStruct>,
}
const FOO: Foo = Foo { init: false, data: MaybeUninit::uninit() };
