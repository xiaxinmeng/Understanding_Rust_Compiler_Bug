rust
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

pub fn plainbool() -> bool {
    true
}


pub fn structbool() -> Bool {
    Bool { b: true }
}
