rust
unsafe {
    // The beginning of the flash memory for the Non-Secure application
    let ns_vector_table = 0x00200000u32 as *const u32;
    // Set the last bit to zero to show a transition from Secure to Non-Secure
    let ns_reset_vector = *(0x00200004u32 as *const u32) & !1u32;
    // Switch the stack pointer to the Main Stack Pointer from Non-Secure world
    cortex_m::register::msp::write_ns(*ns_vector_table);
    // Branch to the Non-Secure reset vector
    cortex_m::asm::bx_ns(ns_reset_vector);
}
