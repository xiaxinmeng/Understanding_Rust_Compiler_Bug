c
#ifndef _WIN32
// __attribute__((destructor)) and destructors whose priorities are greater than
// 100 run before this function and can thus be tracked. The priority is
// compatible with GCC 7 onwards.
__attribute__((destructor(100)))
#endif
static void llvm_writeout_and_clear(void) {
  llvm_writeout_files();
  fn_list_remove(&writeout_fn_list);
}
