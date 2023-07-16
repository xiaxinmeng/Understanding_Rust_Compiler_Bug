rust
$ cat test.rs
#![crate_type = "lib"]

pub fn mut_raw_then_mut_shr() -> (i32, i32) {
    let mut x = 2;
    let xref = &mut x;
    let xraw = &mut *xref as *mut _;
    let xshr = &*xref;
    let a = *xshr;
    unsafe {
        *xraw = 4;
    }
    (a, x)
}
$ rustc +master -Zdump-mir=all test.rs -Zmir-opt-level=0 -Zmir-enable-passes=+ReferencePropagation -Cdebug-assertions=no
