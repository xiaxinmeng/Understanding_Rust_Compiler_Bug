
// build-pass
// only-linux

// Run the test for both i686 and x86_64
// revisions: i686 x86_64
// [i686]compile-flags:--target=i686-pc-windows-gnu
// [x86_64]compile-flags:--target=x86_64-pc-windows-gnu

#![feature(raw_dylib)]
#![feature(no_core, lang_items)]
#![no_std]
#![no_core]
#![crate_type = "lib"]

// This is needed because of #![no_core]:
#[lang = "sized"]
trait Sized {}

// The DLL does not actually exist anywhere, but we don't care.
// We just want rustc to successfully build an import library
// from the extern block.
#[link(name = "extern_1", kind = "raw-dylib")]
extern "C" {
    fn extern_fn_2();
    fn print_extern_variable();
    static mut extern_variable: i32;
    #[link_name = "extern_fn_4"]
    fn extern_fn_4_renamed();
}

pub unsafe fn foo() {
    extern_fn_2();
    print_extern_variable();
    extern_variable = 123;
    extern_fn_4_renamed();
}
