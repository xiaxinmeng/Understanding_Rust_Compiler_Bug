rust
pub extern "x86-interrupt" fn handler(mut stack: InterruptStackFrame) {
    unsafe {
        addr_of_mut!(stack.ip).write_volatile(0x0);
    }
}
