rust
// Add 5 to variable:
let mut var = 0;
unsafe {
    asm!("add $5, {}", inout(reg) var);
}

// Get L1 cache size
let ebx: i32;
let ecx: i32;
unsafe {
    asm!(r"
        mov $$4, %eax;
        xor %ecx, %ecx;
        cpuid;
        mov %ebx, {};",
        out(reg) ebx, out(ecx) ecx, clobber(eax, ebx, edx)
    );
}
println!("L1 Cache: {}", ((ebx >> 22) + 1)
    * (((ebx >> 12) & 0x3ff) + 1)
    * ((ebx & 0xfff) + 1) * (ecx + 1));
