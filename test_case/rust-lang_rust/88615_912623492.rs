
---- compile_test stdout ----
thread 'compile_test' panicked at '
----------------------------------------------------------------------
ERROR: Found multiple rlibs for crates: `itertools`
Try removing the stageN-tools directory or remove the following files:

/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-25dc0cde4ae95530.rlib \
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-a462d7304c071790.rlib \
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-079b879cf9789047.rlib
