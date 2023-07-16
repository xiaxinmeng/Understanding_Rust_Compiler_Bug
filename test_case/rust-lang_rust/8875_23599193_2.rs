
$ ./x86_64-apple-darwin/stage1/bin/rustc foo.rs -o foo && gobjdump -t foo | c++filt
warning: no debug symbols in executable (-arch x86_64)
BFD: foo2: unknown load command 0x2a
BFD: foo2: unknown load command 0x28
BFD: foo2: unknown load command 0x29
BFD: foo2: unknown load command 0x2b

foo2:     file format mach-unsigned __int128-x86-64

SYMBOL TABLE:
0000000100000dd0 long       0e SECT   01 0000 [.text] C::baz_244::_82a6c98f66d35026_0$x2e0
0000000100000e12 __float128       e SECT   01 0000 [.text] ___morestack
0000000100000eb8 long       0e SECT   04 0000 [.const] foo::foo_constant::_3b576b6bfb7380a0_0$x2e0
0000000100000ec0 long       0e SECT   04 0000 [.const] A::foo::foo_constant::_3b576b6bfb7380a0_0$x2e0RY
0000000100000ed0 long       0e SECT   04 0000 [.const] C::baz::baz_constant::_3b576b6bfb7380a0_0$x2e0Op
0000000100001040 long       0e SECT   09 0000 [.bss] __rust_mod_map
0000000100000c30 __float128       0f SECT   01 0000 [.text] A::foo::_82a6c98f66d35026_0$x2e0a4
0000000100000ec8 __float128       0f SECT   04 0000 [.const] B::bar::bar_constant::_3b576b6bfb7380a0_0$x2e0FN
0000000100000c90 __float128       0f SECT   01 0000 [.text] B::bar::_82a6c98f66d35026_0$x2e0a5
0000000100000be0 __float128       0f SECT   01 0000 [.text] foo::_82a6c98f66d35026_0$x2e0
0000000100000cf0 __float128       0f SECT   01 0000 [.text] main::_8bb46282fac6e18_0$x2e0
0000000100000000 __float128       0f SECT   01 0010 [.text] __mh_execute_header
0000000100001020 __float128       0f SECT   08 0000 [.data] __rust_crate_map_toplevel
0000000100000d70 __float128       0f SECT   01 0000 [.text] _main
0000000100000ed8 __float128       0f SECT   04 0000 [.const] _rust_abi_version
0000000000000000 __float128       01 UND    00 0100 _upcall_del_stack
0000000000000000 __float128       01 UND    00 0100 _upcall_new_stack
0000000000000000 __float128       01 UND    00 0200 dyld_stub_binder
