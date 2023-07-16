 rust
pub fn exit(n: uint) -> ! {
    static NR_EXIT: uint = 60;
    unsafe {
        asm!("syscall"
            :: "{rax}"(NR_EXIT), "{rdi}"(n));
        intrinsics::unreachable()
    }
}
