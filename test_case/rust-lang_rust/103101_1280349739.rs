
PanicGuard = unsafe { deallocate memory on drop }

f() {
  let guard = PanicGuard(memory);
  do_stuff();
  // Yay, no panic
  forget(guard)
}
