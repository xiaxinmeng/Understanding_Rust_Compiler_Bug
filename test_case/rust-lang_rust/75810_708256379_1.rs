rust
unsafe {
    // The beginning of the flash memory for the Non-Secure application
    let ns_vector_table = 0x00200000u32 as *const u32;
    cortex_m::register::msp::write_ns(*ns_vector_table);

    let ns_entry: extern "C" fn() = unsafe { std::mem::transmute(*(0x00200004u32 as *const u32)) };
    (#[cmse_nonsecure_call] ns_entry)()
}
