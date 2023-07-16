plain
failures:

---- [rustdoc-json] rustdoc-json/type/fn_lifetime.rs stdout ----

error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/type/fn_lifetime" "--template" "/checkout/src/test/rustdoc-json/type/fn_lifetime.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Failed check: `@has - $.index[*][?(@.name=='GenericFn')].inner.generics.params[*].kind.lifetime` didn't match when it should on line 7
Failed check: `@has - $.index[*][?(@.name=='ForAll')].inner.type.inner.generic_params[*].kind.lifetime` didn't match when it should on line 23
Error: "Jsondocck failed for /checkout/src/test/rustdoc-json/type/fn_lifetime.rs"
------------------------------------------

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/type/fn_lifetime/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/type/fn_lifetime" "--deny" "warnings" "/checkout/src/test/rustdoc-json/type/fn_lifetime.rs" "--output-format" "json" "-Zunstable-options"
------------------------------------------

------------------------------------------
stderr:
