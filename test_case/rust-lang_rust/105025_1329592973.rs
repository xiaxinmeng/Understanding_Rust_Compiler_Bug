
---- [rustdoc-json] src/test/rustdoc-json/intra-doc-links/foreign_variant.rs stdout ----

error: jsondoclint failed!
status: exit status: 1
command: "/home/nixon/dev/rust/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondoclint" "/home/nixon/dev/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-json/intra-doc-links/foreign_variant/foreign_variant.json"
stdout: none
--- stderr -------------------------------
20:6:65935 not in index or paths, but refered to at '$.index["20:4:4198"].links["`Enum::Variant`"]'
Error: Errors validating json /home/nixon/dev/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-json/intra-doc-links/foreign_variant/foreign_variant.json
------------------------------------------
