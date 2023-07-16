plain
---- [rustdoc-json] tests/rustdoc-json/traits/automatically_derived_attr.rs stdout ----

error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/automatically_derived_attr" "--template" "/checkout/tests/rustdoc-json/traits/automatically_derived_attr.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.inner.for.inner.name == "Derived")].attrs matched to [], but expected Array [String("#[automaticly_derived")] on line 4
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/traits/automatically_derived_attr.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/automatically_derived_attr/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/automatically_derived_attr" "--deny" "warnings" "/checkout/tests/rustdoc-json/traits/automatically_derived_attr.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none


failures:
    [rustdoc-json] tests/rustdoc-json/traits/automatically_derived_attr.rs
