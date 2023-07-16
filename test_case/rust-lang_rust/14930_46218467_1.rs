 rust
/// Returns task stack pointer (PSP).
#[cfg(not(test))]
#[inline(always)]
pub fn get_task_stack_pointer() -> u32 {
  let mut val: u32;
  unsafe { asm!("mrs $0, psp" : "=r"(val)) };
  val
}
