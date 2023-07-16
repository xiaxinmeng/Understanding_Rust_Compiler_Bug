plain
failures:

---- [rustdoc-json] rustdoc-json/fns/generic_args.rs stdout ----

error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args" "--template" "/checkout/src/test/rustdoc-json/fns/generic_args.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.params[0].kind matched to [Object({"type": Object({"bounds": Array([]), "default": Null, "synthetic": Bool(false)})})], but expected Object({"type": Object({"bounds": Array([]), "default": Null})}) on line 34
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[0].bound_predicate.ty matched to [], but expected Object({"inner": String("F"), "kind": String("generic")}) on line 40
Error: "Jsondocck failed for /checkout/src/test/rustdoc-json/fns/generic_args.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args" "--deny" "warnings" "/checkout/src/test/rustdoc-json/fns/generic_args.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none


failures:
    [rustdoc-json] rustdoc-json/fns/generic_args.rs
