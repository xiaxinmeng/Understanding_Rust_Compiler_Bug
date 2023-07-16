rust
pub extern "x86-interrupt" fn handler(mut stack: InterruptStackFrame) {
    stack.ip = 0;
}
