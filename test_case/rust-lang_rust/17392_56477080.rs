
#![feature(lang_items)]
#![no_std]

#[lang="sized"]
trait Sized {}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    0
}
