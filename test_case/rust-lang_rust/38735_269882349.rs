rust
fn lidt(idt_desc : CPUTableDescriptor) {
    unsafe {
        asm!(
            "lidt $0"
            :
            : "*m"(&idt_desc)
            :
            );
    }
}
