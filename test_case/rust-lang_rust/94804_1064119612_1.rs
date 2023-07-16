
error: any use of this value will cause an error
    |
   ::: src/lib.rs:3:1
    |
3   | / pub const C: () = {
4   | |     unsafe { f(std::mem::transmute(&0)) };
5   | | };
    | |__-
    |
    = note: `#[deny(const_err)]` on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, [see issue #71800 <https://github.com/rust-lang/rust/issues/71800>](https://github.com/rust-lang/rust/issues/71800)
