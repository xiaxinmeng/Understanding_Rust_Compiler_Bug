rust
#[no_mangle]
fn unsafe extern "x86-interrupts" do_page_fault(exception_stack: ExceptionStackFrame) {
  CURRENT_PROCESS.mm.handle(&exception_stack);
}
