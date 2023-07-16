
(lldb) t 16
* thread #16, name = 't:tokio'
    frame #0: 0x00007fff7d5d3bf2 libsystem_kernel.dylib`kevent + 10
libsystem_kernel.dylib`kevent:
->  0x7fff7d5d3bf2 <+10>: jae    0x7fff7d5d3bfc            ; <+20>
    0x7fff7d5d3bf4 <+12>: movq   %rax, %rdi
    0x7fff7d5d3bf7 <+15>: jmp    0x7fff7d5c9b00            ; cerror_nocancel
    0x7fff7d5d3bfc <+20>: retq
(lldb) list
(lldb) bt
warning: (x86_64) /xxx/target/debug/deps/libmio-8461a2db86d1af74.rlib(mio-8461a2db86d1af74.mio8.rcgu.o) 0xfdc00000d4d: DW_TAG_member bitfield named "(null)" has invalid bit offset (0x100000000) member will be ignored. Please file a bug against the compiler and include the preprocessed output for /Users/cong/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.6.9/src/lib.rs/@/mio8

error: liblib_net-22a2ad34bc598d14.rlib(lib_net-22a2ad34bc598d14.35a0x8f7rhy3adti.rcgu.o) DWARF DIE at 0x0000195f (class Option<core::result::Result<(), ()>>) has a member variable 0x00001966 (RUST$ENCODED$ENUM$0$None) whose type is a forward declaration, not a complete definition.
Try compiling the source file with -fstandalone-debug
Segmentation fault: 11
