plain
failures:

---- [rustdoc-json] src/test/rustdoc-json/primitive.rs stdout ----

error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/primitive" "--template" "/checkout/src/test/rustdoc-json/primitive.rs"
stdout: none
--- stderr -------------------------------
Failed check: `@has - $.index[*][?(@.name=='log10')]` didn't match when it should on line 10
Error: "Jsondocck failed for /checkout/src/test/rustdoc-json/primitive.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/primitive/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/primitive" "--deny" "warnings" "/checkout/src/test/rustdoc-json/primitive.rs" "--edition=2018" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none


failures:
    [rustdoc-json] src/test/rustdoc-json/primitive.rs
