 rust
struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        static mut DROPPED: bool = false;
        unsafe {
            assert!(!DROPPED);
            DROPPED = true;
        }
    }
}

struct Empty;

fn empty() -> Empty { Empty }

fn should_panic(_: Foo, _: Empty) {
    panic!("test panic");
}

fn main() {
    should_panic(Foo(1), empty());
}
