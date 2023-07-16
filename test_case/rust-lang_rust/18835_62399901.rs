 rust
#![feature(unboxed_closures, tuple_indexing)]

struct Chunk<T>(Box<FnOnce<(), T>+'static>);

impl<T> FnOnce<(), T> for Chunk<T> {
    extern "rust-call" fn call_once(self, arg: ()) -> T {
        self.0.call_once(arg)
    }
}
