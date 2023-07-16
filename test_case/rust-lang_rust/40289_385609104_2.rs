 rust
// foo/src/lib.rs

#![feature(used)]
#![no_std] // not required but I'm using thumbv7m-none-eabi as the example target

pub mod foo {
    // `STATIC` needs to be an *exported* symbol; they only way to do that right now is to make the
    // item *both* public and reachable
    #[doc(hidden)] // don't show this in the API docs
    #[export_name = "STATIC"] // or you could use `no_mangle` and rename the item, same thing
    #[used] // required or the compiler will drop this symbol when optimizations are enabled
    pub static __HIDDEN: [u32; 10] = [1; 10];
}
