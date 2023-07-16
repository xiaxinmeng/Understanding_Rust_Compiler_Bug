
(lldb) frame sel 10
frame #10: 0x00000008119e613c libxul.so`linked_hash_map::LinkedHashMap$LT$K$C$V$C$S$GT$::ensure_guard_node::ha74213912ea83131 [inlined] core::mem::uninitialized::hfd5548125f921871 at mod.rs:659:9
   656  pub unsafe fn uninitialized<T>() -> T {
   657      // SAFETY: the caller must guarantee that an unitialized value is valid for `T`.
   658      unsafe {
-> 659          intrinsics::assert_uninit_valid::<T>();
   660          MaybeUninit::uninit().assume_init()
   661      }
   662  }
(lldb) frame sel 11
frame #11: 0x00000008119e6123 libxul.so`linked_hash_map::LinkedHashMap$LT$K$C$V$C$S$GT$::ensure_guard_node::ha74213912ea83131(self=<unavailable>) at lib.rs:174
   171          if self.head.is_null() {
   172              // allocate the guard node if not present
   173              unsafe {
-> 174                  self.head = Box::into_raw(Box::new(mem::uninitialized()));
   175                  (*self.head).next = self.head;
   176                  (*self.head).prev = self.head;
   177              }
