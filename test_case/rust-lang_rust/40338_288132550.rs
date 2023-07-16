text
make[1]: Entering directory '/checkout/src/test/run-make/sysroot-crates-are-unstable'

verifying getopts is an unstable crate

crate getopts is not unstable

error[E0464]: multiple matching crates for `getopts`

 --> <anon>:1:1

  |

1 | extern crate getopts;

  | ^^^^^^^^^^^^^^^^^^^^^

  |

  = note: candidates:

  = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-a80f7c589731de53.so

  = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-a80f7c589731de53.rlib

  = note: crate name: getopts

  = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-ccd85c85236add46.rlib

  = note: crate name: getopts
