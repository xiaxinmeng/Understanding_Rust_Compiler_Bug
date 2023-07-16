rust
#[no_mangle]
fn unsafe extern "x86-interrupts" do_pic_irq(irq_stack: IrqStackFrame) {
  sched.timer(&irq_stack);
}
