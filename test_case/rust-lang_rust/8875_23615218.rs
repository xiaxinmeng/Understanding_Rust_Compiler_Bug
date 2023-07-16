
$ gobjdump -t foo
BFD: foo: unknown load command 0x2a
BFD: foo: unknown load command 0x28
BFD: foo: unknown load command 0x29
BFD: foo: unknown load command 0x2b

foo:     file format mach-o-x86-64

SYMBOL TABLE:
0000000100000dc0 l       0e SECT   01 0000 [.text] _C<T>::baz_247-ve3fc7435c4ba24c0Op
0000000100000e17 g       1e SECT   01 0000 [.text] ___morestack
0000000100000ec0 l       0e SECT   04 0000 [.const] _foo::foo_constant-vd2f173dfc35a5d36
0000000100000ec8 l       0e SECT   04 0000 [.const] _A::foo::foo_constant-vd2f173dfc35a5d36RY
0000000100000ed8 l       0e SECT   04 0000 [.const] _C<T>::baz::baz_constant-vd2f173dfc35a5d36Op
0000000100001040 l       0e SECT   09 0000 [.bss] __rust_mod_map
0000000100000c20 g       0f SECT   01 0000 [.text] _A::foo-ve3fc7435c4ba24c0RYa7
0000000100000c80 g       0f SECT   01 0000 [.text] _B[for A]::bar-ve3fc7435c4ba24c0FNa8
0000000100000ed0 g       0f SECT   04 0000 [.const] _B[for A]::bar::bar_constant-vd2f173dfc35a5d36FN
0000000100000000 g       0f SECT   01 0010 [.text] __mh_execute_header
0000000100001020 g       0f SECT   08 0000 [.data] __rust_crate_map_toplevel
0000000100000bd0 g       0f SECT   01 0000 [.text] _foo-ve3fc7435c4ba24c0
0000000100000d60 g       0f SECT   01 0000 [.text] _main
0000000100000ce0 g       0f SECT   01 0000 [.text] _main-vb9fcaaf7d4ea848c
0000000100000ee0 g       0f SECT   04 0000 [.const] _rust_abi_version
0000000000000000 g       01 UND    00 0100 _upcall_del_stack
0000000000000000 g       01 UND    00 0100 _upcall_new_stack
0000000000000000 g       01 UND    00 0200 dyld_stub_binder
