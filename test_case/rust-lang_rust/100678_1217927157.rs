plain
failures:

---- [rustdoc-json] src/test/rustdoc-json/reexport/export_extern_crate_as_self.rs stdout ----

error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/reexport/export_extern_crate_as_self" "--template" "/checkout/src/test/rustdoc-json/reexport/export_extern_crate_as_self.rs"
stdout: none
--- stderr -------------------------------
Invalid command: Incorrect number of arguments to `@is` on line 10
Error: "Jsondocck failed for /checkout/src/test/rustdoc-json/reexport/export_extern_crate_as_self.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/reexport/export_extern_crate_as_self/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/reexport/export_extern_crate_as_self" "--deny" "warnings" "/checkout/src/test/rustdoc-json/reexport/export_extern_crate_as_self.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none


failures:
    [rustdoc-json] src/test/rustdoc-json/reexport/export_extern_crate_as_self.rs
