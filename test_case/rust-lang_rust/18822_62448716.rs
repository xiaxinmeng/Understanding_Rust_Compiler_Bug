
...
rustdoc: doc/rustc/index.html
/contrib/projects/rust/rust/src/librustc/middle/trans/glue.rs:531:31: 531:35 warning: deprecated syntax, use `for` keyword now
/contrib/projects/rust/rust/src/librustc/middle/trans/glue.rs:531                      helper: <'blk, 'tcx> |Block<'blk, 'tcx>, ValueRef, ty::t|
                                                                                                ^~~~
/contrib/projects/rust/rust/src/librustc/middle/trans/base.rs:1802:39: 1802:43 warning: deprecated syntax, use `for` keyword now
/contrib/projects/rust/rust/src/librustc/middle/trans/base.rs:1802                      maybe_load_env: <'blk, 'tcx> |Block<'blk, 'tcx>, ScopeId|
                                                                                                         ^~~~
/contrib/projects/rust/rust/src/librustc/metadata/decoder.rs:633:35: 633:39 warning: deprecated syntax, use `for` keyword now
/contrib/projects/rust/rust/src/librustc/metadata/decoder.rs:633 pub type DecodeInlinedItem<'a> = <'tcx> |cdata: Cmd,
                                                                                                   ^~~~
<stdin>:2:8: 2:9 error: unknown start of token: \\
<stdin>:2        \        \    /  /
                 ^

task '<main>' panicked at 'Box<Any>', /contrib/projects/rust/rust/src/libsyntax/diagnostic.rs:86
