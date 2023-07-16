
(gdb) p f
$1 = u::Foo {
  ptr: core::ptr::Unique<u::Bar> {
    pointer: core::nonzero::NonZero<*const u::Bar> (
      0x7fffffffe198
    ),
    _marker: core::marker::PhantomData<u::Bar>
  }
}
