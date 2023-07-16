 rust
#[cold]
#[inline(never)]
fn fail_alloc() -> ! {
   rterrln!("failed allocation: aborting");
   abort()
}
