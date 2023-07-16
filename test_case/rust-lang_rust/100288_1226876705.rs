rust
#[derive(Copy, Clone)]
pub struct ChildStdin {
    inner: AnonPipe,
}
#[derive(Copy, Clone)]
enum AnonPipe {}

const FOO: () = {
const FOO: () = {
    union Foo {
        a: ChildStdin,
        b: (),
    }
    let x = unsafe { Foo { b: () }.a };
    let x = &x.inner;
};
