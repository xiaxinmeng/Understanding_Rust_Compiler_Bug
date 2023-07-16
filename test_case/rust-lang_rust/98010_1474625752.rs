rust
#![no_core]
#![feature(no_core, lang_items)]
#[lang = "sized"]
trait Sync {}
#[lang = "copy"]
trait Freeze {}

extern "C" {
    fn ExitProcess(uexitcode: u32) -> !;
}

fn exit_process(exit_code: u32) -> ! {
    unsafe {
        ExitProcess(exit_code);
    }
}
