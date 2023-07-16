
prepare: tmp/dist/rust-0.11-pre-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libgraphviz-*.rlib
prepare: tmp/dist/rust-0.11-pre-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-*.so
ls: cannot access i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-*.so: No such file or directory
install: missing destination file operand after 'tmp/dist/rust-0.11-pre-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/'
Try 'install --help' for more information.
/build/rust-git/src/rust/mk/dist.mk:245: recipe for target 'prepare-target-i686-unknown-linux-gnu-host-i686-unknown-linux-gnu-2-dir-i686-unknown-linux-gnu' failed
make: *** [prepare-target-i686-unknown-linux-gnu-host-i686-unknown-linux-gnu-2-dir-i686-unknown-linux-gnu] Error 1
