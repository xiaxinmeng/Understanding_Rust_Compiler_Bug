rust
// license here
// compile-flags: --target=sparc64-unknown-linux-gnu
#![feature(no_core, lang_items)]
#![no_core]

#[lang="sized"]
trait Sized { }
#[lang="freeze"]
trait Freeze { }
#[lang="copy"]
trait Copy { }
#[repr(C)]
pub struct Bool {
    b: bool,
}
#[no_mangle]
// CHECK: define i64 @structbool()
// CHECK-NEXT: start:
// CHECK-NEXT: ret i64 72057594037927936
pub extern "C" fn structbool() -> Bool {
    Bool { b: true }
}
