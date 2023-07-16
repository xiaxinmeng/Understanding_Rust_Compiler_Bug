
---- src/test/rustdoc/issue-43153.rs - Foo (line 6) stdout ----
"/home/andjo403/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--cfg" "rustdoc" "--cfg" "doctest" "--edition" "2015" "-o" "/tmp/rustdoctestVAr91J/rust_out" "-L" "/home/andjo403/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/andjo403/rust/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-"
Output { status: ExitStatus(ExitStatus(256)), stdout: "", stderr: "error: couldn\'t read \"src/test/rustdoc/auxiliary/empty.rs: No such file or directory (os error 2)\n --> \"src/test/rustdoc/issue-43153.rs\":7:1\n  |\n2 | include!(\"auxiliary/empty.rs\");\n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\nerror: aborting due to previous error\n\n" }
Couldn't compile the test.
