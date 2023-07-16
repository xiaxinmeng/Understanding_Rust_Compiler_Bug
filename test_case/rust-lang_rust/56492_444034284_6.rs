\n\nMust always be called with exactly two arguments, e.g. `f(2, \"test\")`.\n\nNote that Rust does not have a notion of optional function arguments or\nvariadic functions (except for its C-FFI).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_plugin_attr.rs","byte_start":2341,"byte_end":2347,"line_start":71,"line_end":71,"column_start":15,"column_end":21,"is_primary":true,"text":[{"text":"    trait_def.expand(cx, mitem, item, push)","highlight_start":15,"highlight_end":21}],"label":"expected 3 parameters","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0061]: thL" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/derive-totalsum/auxiliary"
[01:00:47] ------------------------------------------
[01:00:47] 
[01:00:47] ------------------------------------------
[01:00:47] stderr:
[01:00:47] stderr:
[01:00:47] ------------------------------------------
[01:00:47] {"message":"this function takes 3 parameters but 4 parameters were supplied","code":{"code":"E0061","explanation":"\nThe number of arguments passed to a function must match the number of arguments\nspecified in the function signature.\n\nFor example, a function like:\n\n