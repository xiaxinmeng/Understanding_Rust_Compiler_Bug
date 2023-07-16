
$ make -j 20 check
cfg: shell host triple x86_64-unknown-linux-gnu
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: unix-y environment
cfg: using gcc
cfg: no llnextgen found, omitting grammar-verification
cfg: including dist rules
cfg: including test rules
check: formatting
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore.so
extract: tutorial tests
extract: tutorial-ffi tests
extract: tutorial-macros tests
extract: tutorial-borrowed-ptr tests
extract: tutorial-tasks tests
extract: ref tests
/home/graydon/src/rust/src/libcore/core.rs:76: tab character
/home/graydon/src/rust/src/libcore/core.rs:76: trailing whitespace
make: *** [tidy] Error 123
make: *** Waiting for unfinished jobs....
$
