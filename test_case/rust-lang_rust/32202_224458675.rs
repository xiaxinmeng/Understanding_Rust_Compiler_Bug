
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\libstd\sys/windows\fs.rs:122:13: 122:24 error: expected an array or slice, found `&[u16]` [E0529]
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\libstd\sys/windows\fs.rs:122             [46, 0, ..] |
                                                                                              ^~~~~~~~~~~
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\libstd\sys/windows\fs.rs:122:13: 122:24 help: the semantics of slice patterns changed recently; see issue #23121
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\libstd\sys/windows\fs.rs:123:13: 123:28 error: expected an array or slice, found `&[u16]` [E0529]
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\libstd\sys/windows\fs.rs:123             [46, 46, 0, ..] => return None,
                                                                                              ^~~~~~~~~~~~~~~
C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\libstd\sys/windows\fs.rs:123:13: 123:28 help: the semantics of slice patterns changed recently; see issue #23121
error: aborting due to 2 previous errors
