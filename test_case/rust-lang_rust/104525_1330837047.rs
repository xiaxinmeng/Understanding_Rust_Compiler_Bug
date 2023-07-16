plain
---- [rustdoc-json] src/test/rustdoc-json/traits/uses_extern_trait.rs stdout ----

error: jsondocck failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/uses_extern_trait" "--template" "/checkout/src/test/rustdoc-json/traits/uses_extern_trait.rs"
stdout: none
--- stderr -------------------------------
  left: `2`,
  left: `2`,
 right: `1`: Expected 1 match for `$.index[*][?(@.name=='Debug')].inner.items[*]` (because of @set): matched to [String("1:10659:650"), String("1:10660:27990")]', src/tools/jsondocck/src/main.rs:314:13
------------------------------------------

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/uses_extern_trait/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/uses_extern_trait" "--deny" "warnings" "/checkout/src/test/rustdoc-json/traits/uses_extern_trait.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none


failures:
    [rustdoc-json] src/test/rustdoc-json/traits/uses_extern_trait.rs
