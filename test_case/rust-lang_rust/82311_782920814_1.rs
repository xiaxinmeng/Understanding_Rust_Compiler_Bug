
---- [rustdoc-json] rustdoc-json/reexport/failing.rs stdout ----

error: jsondocck failed!
status: exit code: 1
command: "/home/nixon/upstreams/rust/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/home/nixon/upstreams/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-json/reexport/failing" "--template" "/home/nixon/upstreams/rust/rust/src/test/rustdoc-json/reexport/failing.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Failed check: `@set var = failing.json foo.bar.baz` didn't match when it should on line 4
Failed check: `@is failing.json root not_root` didn't match when it should on line 5
Error: "Jsondocck failed for /home/nixon/upstreams/rust/rust/src/test/rustdoc-json/reexport/failing.rs"

------------------------------------------
