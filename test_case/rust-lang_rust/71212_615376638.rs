rust
// Compile on stable and work as expected.
const NULL: *mut i32 = ptr::null_mut(); 
const A: *const i32 = &4;

// Must not compile.
const B: *mut i32 = &mut 4;

fn main() {
    println!("{}", unsafe { *A });
    // unsafe { *B = 4 } // Bad news
}
