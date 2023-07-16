rust
#![feature(never_type)]
#![feature(raw_ref_macros)]
#![feature(raw_ref_op)]

extern {
    static FOO: !;
}

mod foo {
    #[no_mangle]
    static FOO: u32 = 5;
}

fn main() {
    dbg!(unsafe { core::ptr::raw_const!(FOO) });
    dbg!(unsafe { &raw const FOO });
}
