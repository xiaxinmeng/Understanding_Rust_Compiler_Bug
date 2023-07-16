
~:45$ rustc --crate-name hello_world src/main.rs --crate-type bin --emit=link -C debuginfo=2 -Zprofile -Copt-level=1 -Clink-dead-code -Ccodegen-units=1 -Zno-landing-pads -C metadata=e6641c514587dc22 -C extra-filename=-e6641c514587dc22 --out-dir ~/hello_world/target/debug/deps 

~:46$ ./target/debug/deps/hello_world-e6641c514587dc22
Hello, world!

~:47$ rustc --crate-name hello_world src/main.rs --crate-type bin --emit=link -C debuginfo=2 -Zprofile -Copt-level=1 -Clink-dead-code -Ccodegen-units=1 -Zno-landing-pads -C metadata=e6641c514587dc22 -C extra-filename=-e6641c514587dc22 --out-dir ~/hello_world/target/debug/deps -C incremental=~/hello_world/target/debug/incremental

~:48$ ./target/debug/deps/hello_world-e6641c514587dc22
Hello, world!
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000001)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x24246437)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous run count: corrupt object tag (0x24643775)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x39383038)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous run count: corrupt object tag (0x32626265)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x39383038)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous run count: corrupt object tag (0x62333333)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x38327472)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x34737973)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous run count: corrupt object tag (0x78696e75)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x246e6f69)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous run count: corrupt object tag (0x36245447)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous GCDA file: corrupt arc tag (0x38306266)
profiling: ~/hello_world/target/debug/deps/hello_world-e6641c514587dc22.gcda: cannot merge previous run count: corrupt object tag (0x00004538)
