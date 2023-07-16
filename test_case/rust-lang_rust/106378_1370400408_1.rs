
---- [rustdoc-json] src/test/rustdoc-json/intra-doc-links/prim_method.rs stdout ----

error: jsondoclint failed!
status: exit status: 1
command: "/home/nixon/dev/rust/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondoclint" "/home/nixon/dev/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-json/intra-doc-links/prim_method/prim_method.json"
stdout: none
--- stderr -------------------------------
2:2323:1834 not in index or paths, but refered to at '$.index["0:0:1572"].links["i32::max"]'
5:6414:911 not in index or paths, but refered to at '$.index["0:0:1572"].links["Vec::new"]'
Error: Errors validating json /home/nixon/dev/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-json/intra-doc-links/prim_method/prim_method.json
------------------------------------------
