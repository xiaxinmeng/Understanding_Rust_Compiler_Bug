 Rust
    unsafe {
        asm!("int $$0x29" :: "{ecx}"(7) ::: volatile); // 7 is FAST_FAIL_FATAL_APP_EXIT
        std::intrinsics::unreachable();
    }
