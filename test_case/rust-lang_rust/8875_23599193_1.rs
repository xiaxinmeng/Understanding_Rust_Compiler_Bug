
$ ./x86_64-apple-darwin/stage0/bin/rustc foo.rs -o foo && gobjdump -t foo | c++filt
warning: no debug symbols in executable (-arch x86_64)
BFD: foo: unknown load command 0x2a
BFD: foo: unknown load command 0x28
BFD: foo: unknown load command 0x29
BFD: foo: unknown load command 0x2b

foo:     file format mach-unsigned __int128-x86-64

SYMBOL TABLE:
0000000100000da0 long       0e SECT   01 0000 [.text] __extensions__::baz_216::_b12014513859d86::_0$x2e0
0000000100000de6 __float128       e SECT   01 0000 [.text] ___morestack
0000000100000e90 long       0e SECT   04 0000 [.const] foo::foo_constant::_15ddc065e7e4c082::_0$x2e0
0000000100000e98 long       0e SECT   04 0000 [.const] __extensions__::foo_constant::_15ddc065e7e4c082::_0$x2e0
0000000100000ea8 long       0e SECT   04 0000 [.const] __extensions__::baz_constant::_15ddc065e7e4c082::_0$x2e0
0000000100001040 long       0e SECT   09 0000 [.bss] __rust_mod_map
0000000100000ea0 __float128       0f SECT   04 0000 [.const] __extensions__::bar_constant::_15ddc065e7e4c082::_0$x2e0
0000000100000c10 __float128       0f SECT   01 0000 [.text] __extensions__::meth_211::foo::_b12014513859d86::_0$x2e0
0000000100000c70 __float128       0f SECT   01 0000 [.text] __extensions__::meth_213::bar::_b12014513859d86::_0$x2e0
0000000100000bd0 __float128       0f SECT   01 0000 [.text] foo::_b12014513859d86::_0$x2e0
0000000100000cd0 __float128       0f SECT   01 0000 [.text] main::_2de6c7ff3a55374a::_0$x2e0
0000000100000000 __float128       0f SECT   01 0010 [.text] __mh_execute_header
0000000100001020 __float128       0f SECT   08 0000 [.data] __rust_crate_map_toplevel
0000000100000d40 __float128       0f SECT   01 0000 [.text] _main
0000000100000eb0 __float128       0f SECT   04 0000 [.const] _rust_abi_version
0000000000000000 __float128       01 UND    00 0100 _upcall_del_stack
0000000000000000 __float128       01 UND    00 0100 _upcall_new_stack
0000000000000000 __float128       01 UND    00 0200 dyld_stub_binder
