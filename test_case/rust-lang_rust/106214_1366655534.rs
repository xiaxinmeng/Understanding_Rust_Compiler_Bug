plain

---- compile_test stdout ----
diff of stderr:

 error: an async construct yields a type which is itself awaitable
    |
 LL |        let _h = async {
    |  _____________________-
-LL | |          async {
-LL | |          async {
-   | | _________^
+LL | |/         async {
 LL | ||             3
 LL | ||         }
    | ||_________^ awaitable value not awaited
 LL | |      };
    | |______- outer async construct
    |
    = note: `-D clippy::async-yields-async` implied by `-D warnings`
 help: consider awaiting this value
 LL ~         async {
 LL +             3
 LL +         }.await
    |
    |
 
 error: an async construct yields a type which is itself awaitable
   --> $DIR/async_yields_async.rs:44:9
    |
 LL |       let _i = async {
    |  ____________________-
    |  ____________________-
 LL | |         CustomFutureType
    | |         |
    | |         |
    | |         awaitable value not awaited
    | |         help: consider awaiting this value: `CustomFutureType.await`
 LL | |     };
    | |_____- outer async construct
 
 error: an async construct yields a type which is itself awaitable
    |
    |
 LL |        let _j = async || {
-LL | |          async {
-   | | _________^
-   | | _________^
+LL | |/         async {
 LL | ||             3
 LL | ||         }
    | ||_________^ awaitable value not awaited
 LL | |      };
    | |______- outer async construct
 help: consider awaiting this value
    |
 LL ~         async {
 LL +             3
 LL +             3
 LL +         }.await
    |
 
 error: an async construct yields a type which is itself awaitable
    |
    |
 LL |       let _k = async || {
    |  _______________________-
 LL | |         CustomFutureType
    | |         |
    | |         |
    | |         awaitable value not awaited
    | |         help: consider awaiting this value: `CustomFutureType.await`
 LL | |     };
    | |_____- outer async construct
 
 error: an async construct yields a type which is itself awaitable
    |
    |
 LL |     let _l = async || CustomFutureType;
    |                       |
    |                       outer async construct
    |                       outer async construct
    |                       awaitable value not awaited
    |                       help: consider awaiting this value: `CustomFutureType.await`
 
 error: an async construct yields a type which is itself awaitable
    |
    |
 LL |       let _m = async || {
    |  _______________________-
 LL | |         println!("I'm bored");
 LL | |         // Some more stuff
 LL | |
 LL | |         // Finally something to await
 LL | |         CustomFutureType
    | |         |
    | |         |
    | |         awaitable value not awaited
    | |         help: consider awaiting this value: `CustomFutureType.await`
 LL | |     };
    | |_____- outer async construct
 error: aborting due to 6 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/async_yields_async.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async_yields_async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/async_yields_async.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/async_yields_async.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/async_yields_async.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"an async construct yields a type which is itself awaitable","code":{"code":"clippy::async_yields_async","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":725,"byte_end":772,"line_start":38,"line_end":42,"column_start":20,"column_end":6,"is_primary":false,"text":[{"text":"    let _h = async {","highlight_start":20,"highlight_end":21},{"text":"        async {","highlight_start":1,"highlight_end":16},{"text":"            3","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"outer async construct","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/async_yields_async.rs","byte_start":735,"byte_end":766,"line_start":39,"line_end":41,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        async {","highlight_start":9,"highlight_end":16},{"text":"            3","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":"awaitable value not awaited","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::async-yields-async` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider awaiting this value","code":null,"level":"help","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":735,"byte_end":766,"line_start":39,"line_end":41,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        async {","highlight_start":9,"highlight_end":16},{"text":"            3","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":"async {\n            3\n        }.await","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: an async construct yields a type which is itself awaitable\n  --> tests/ui/async_yields_async.rs:39:9\n   |\nLL |        let _h = async {\n   |  _____________________-\nLL | |/         async {\nLL | ||             3\nLL | ||         }\n   | ||_________^ awaitable value not awaited\nLL | |      };\n   | |______- outer async construct\n   |\n   = note: `-D clippy::async-yields-async` implied by `-D warnings`\nhelp: consider awaiting this value\n   |\nLL ~         async {\nLL +             3\nLL +         }.await\n   |\n\n"}
{"message":"an async construct yields a type which is itself awaitable","code":{"code":"clippy::async_yields_async","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":793,"byte_end":825,"line_start":43,"line_end":45,"column_start":20,"column_end":6,"is_primary":false,"text":[{"text":"    let _i = async {","highlight_start":20,"highlight_end":21},{"text":"        CustomFutureType","highlight_start":1,"highlight_end":25},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"outer async construct","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/async_yields_async.rs","byte_start":803,"byte_end":819,"line_start":44,"line_end":44,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        CustomFutureType","highlight_start":9,"highlight_end":25}],"label":"awaitable value not awaited","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider awaiting this value","code":null,"level":"help","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":803,"byte_end":819,"line_start":44,"line_end":44,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        CustomFutureType","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":"CustomFutureType.await","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: an async construct yields a type which is itself awaitable\n  --> tests/ui/async_yields_async.rs:44:9\n   |\nLL |       let _i = async {\n   |  ____________________-\nLL | |         CustomFutureType\n   | |         ^^^^^^^^^^^^^^^^\n   | |         |\n   | |         awaitable value not awaited\n   | |         help: consider awaiting this value: `CustomFutureType.await`\nLL | |     };\n   | |_____- outer async construct\n\n"}
{"message":"an async construct yields a type which is itself awaitable","code":{"code":"clippy::async_yields_async","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":890,"byte_end":937,"line_start":49,"line_end":53,"column_start":23,"column_end":6,"is_primary":false,"text":[{"text":"    let _j = async || {","highlight_start":23,"highlight_end":24},{"text":"        async {","highlight_start":1,"highlight_end":16},{"text":"            3","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"outer async construct","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/async_yields_async.rs","byte_start":900,"byte_end":931,"line_start":50,"line_end":52,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        async {","highlight_start":9,"highlight_end":16},{"text":"            3","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":"awaitable value not awaited","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider awaiting this value","code":null,"level":"help","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":900,"byte_end":931,"line_start":50,"line_end":52,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        async {","highlight_start":9,"highlight_end":16},{"text":"            3","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":"async {\n            3\n        }.await","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: an async construct yields a type which is itself awaitable\n  --> tests/ui/async_yields_async.rs:50:9\n   |\nLL |        let _j = async || {\n   |  ________________________-\nLL | |/         async {\nLL | ||             3\nLL | ||         }\n   | ||_________^ awaitable value not awaited\nLL | |      };\n   | |______- outer async construct\n   |\nhelp: consider awaiting this value\n   |\nLL ~         async {\nLL +             3\nLL +         }.await\n   |\n\n"}
{"message":"an async construct yields a type which is itself awaitable","code":{"code":"clippy::async_yields_async","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":961,"byte_end":993,"line_start":54,"line_end":56,"column_start":23,"column_end":6,"is_primary":false,"text":[{"text":"    let _k = async || {","highlight_start":23,"highlight_end":24},{"text":"        CustomFutureType","highlight_start":1,"highlight_end":25},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"outer async construct","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/async_yields_async.rs","byte_start":971,"byte_end":987,"line_start":55,"line_end":55,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        CustomFutureType","highlight_start":9,"highlight_end":25}],"label":"awaitable value not awaited","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider awaiting this value","code":null,"level":"help","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":971,"byte_end":987,"line_start":55,"line_end":55,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        CustomFutureType","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":"CustomFutureType.await","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: an async construct yields a type which is itself awaitable\n  --> tests/ui/async_yields_async.rs:55:9\n   |\nLL |       let _k = async || {\n   |  _______________________-\nLL | |         CustomFutureType\n   | |         ^^^^^^^^^^^^^^^^\n   | |         |\n   | |         awaitable value not awaited\n   | |         help: consider awaiting this value: `CustomFutureType.await`\nLL | |     };\n   | |_____- outer async construct\n\n"}
{"message":"an async construct yields a type which is itself awaitable","code":{"code":"clippy::async_yields_async","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":1017,"byte_end":1033,"line_start":57,"line_end":57,"column_start":23,"column_end":39,"is_primary":true,"text":[{"text":"    let _l = async || CustomFutureType;","highlight_start":23,"highlight_end":39}],"label":"outer async construct","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/async_yields_async.rs","byte_start":1017,"byte_end":1033,"line_start":57,"line_end":57,"column_start":23,"column_end":39,"is_primary":true,"text":[{"text":"    let _l = async || CustomFutureType;","highlight_start":23,"highlight_end":39}],"label":"awaitable value not awaited","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider awaiting this value","code":null,"level":"help","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":1017,"byte_end":1033,"line_start":57,"line_end":57,"column_start":23,"column_end":39,"is_primary":true,"text":[{"text":"    let _l = async || CustomFutureType;","highlight_start":23,"highlight_end":39}],"label":null,"suggested_replacement":"CustomFutureType.await","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: an async construct yields a type which is itself awaitable\n  --> tests/ui/async_yields_async.rs:57:23\n   |\nLL |     let _l = async || CustomFutureType;\n   |                       ^^^^^^^^^^^^^^^^\n   |                       |\n   |                       outer async construct\n   |                       awaitable value not awaited\n   |                       help: consider awaiting this value: `CustomFutureType.await`\n\n"}
{"message":"an async construct yields a type which is itself awaitable","code":{"code":"clippy::async_yields_async","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":1057,"byte_end":1186,"line_start":58,"line_end":64,"column_start":23,"column_end":6,"is_primary":false,"text":[{"text":"    let _m = async || {","highlight_start":23,"highlight_end":24},{"text":"        println!(\"I'm bored\");","highlight_start":1,"highlight_end":31},{"text":"        // Some more stuff","highlight_start":1,"highlight_end":27},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        // Finally something to await","highlight_start":1,"highlight_end":38},{"text":"        CustomFutureType","highlight_start":1,"highlight_end":25},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"outer async construct","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/async_yields_async.rs","byte_start":1164,"byte_end":1180,"line_start":63,"line_end":63,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        CustomFutureType","highlight_start":9,"highlight_end":25}],"label":"awaitable value not awaited","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider awaiting this value","code":null,"level":"help","spans":[{"file_name":"tests/ui/async_yields_async.rs","byte_start":1164,"byte_end":1180,"line_start":63,"line_end":63,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        CustomFutureType","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":"CustomFutureType.await","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: an async construct yields a type which is itself awaitable\n  --> tests/ui/async_yields_async.rs:63:9\n   |\nLL |       let _m = async || {\n   |  _______________________-\nLL | |         println!(\"I'm bored\");\nLL | |         // Some more stuff\nLL | |\nLL | |         // Finally something to await\nLL | |         CustomFutureType\n   | |         ^^^^^^^^^^^^^^^^\n   | |         |\n   | |         awaitable value not awaited\n   | |         help: consider awaiting this value: `CustomFutureType.await`\nLL | |     };\n   | |_____- outer async construct\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
    |
    |
 LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
    |
    = note: `-D clippy::result-map-unit-fn` implied by `-D warnings`
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
    |
    |
 LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
    |
    |
-LL |        x.field.map(|value| {
-   | | _____|
-   | ||
-   | ||
+LL | //     x.field.map(|value| {
 LL | ||         do_nothing(value);
 LL | ||         do_nothing(value)
 LL | ||     });
    | ||______^- help: try this: `if let Ok(value) = x.field { ... }`
    | 
 
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
    |
    |
 LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
 
 error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`
    |
    |
 LL |     "12".parse::<i32>().map(diverge);
    |     |
    |     |
    |     help: try this: `if let Ok(a) = "12".parse::<i32>() { diverge(a) }`
 
 error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`
    |
    |
 LL |     y.map(do_nothing);
    |     |
    |     |
    |     help: try this: `if let Ok(_y) = y { do_nothing(_y) }`
 error: aborting due to 6 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/result_map_unit_fn_unfixable.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args result_map_unit_fn_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/result_map_unit_fn_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/result_map_unit_fn_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/result_map_unit_fn_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":348,"byte_end":409,"line_start":23,"line_end":23,"column_start":5,"column_end":66,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::result-map-unit-fn` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":348,"byte_end":410,"line_start":23,"line_end":23,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:23:5\n   |\nLL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n   |\n   = note: `-D clippy::result-map-unit-fn` implied by `-D warnings`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":416,"byte_end":490,"line_start":25,"line_end":25,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":416,"byte_end":491,"line_start":25,"line_end":25,"column_start":5,"column_end":80,"is_primary":true,"text":[{"text":"    x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":80}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:25:5\n   |\nLL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":627,"byte_end":708,"line_start":29,"line_end":32,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    x.field.map(|value| {","highlight_start":5,"highlight_end":26},{"text":"        do_nothing(value);","highlight_start":1,"highlight_end":27},{"text":"        do_nothing(value)","highlight_start":1,"highlight_end":26},{"text":"    });","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":627,"byte_end":709,"line_start":29,"line_end":32,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    x.field.map(|value| {","highlight_start":5,"highlight_end":26},{"text":"        do_nothing(value);","highlight_start":1,"highlight_end":27},{"text":"        do_nothing(value)","highlight_start":1,"highlight_end":26},{"text":"    });","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:29:5\n   |\nLL | //     x.field.map(|value| {\nLL | ||         do_nothing(value);\nLL | ||         do_nothing(value)\nLL | ||     });\n   | ||______^- help: try this: `if let Ok(value) = x.field { ... }`\n   |  |______|\n   | \n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":714,"byte_end":776,"line_start":33,"line_end":33,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value); });","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":714,"byte_end":777,"line_start":33,"line_end":33,"column_start":5,"column_end":68,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value); });","highlight_start":5,"highlight_end":68}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:33:5\n   |\nLL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":958,"byte_end":990,"line_start":37,"line_end":37,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    \"12\".parse::<i32>().map(diverge);","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":958,"byte_end":991,"line_start":37,"line_end":37,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    \"12\".parse::<i32>().map(diverge);","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":"if let Ok(a) = \"12\".parse::<i32>() { diverge(a) }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:37:5\n   |\nLL |     \"12\".parse::<i32>().map(diverge);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(a) = \"12\".parse::<i32>() { diverge(a) }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":1190,"byte_end":1207,"line_start":43,"line_end":43,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    y.map(do_nothing);","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":1190,"byte_end":1208,"line_start":43,"line_end":43,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    y.map(do_nothing);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"if let Ok(_y) = y { do_nothing(_y) }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:43:5\n   |\nLL |     y.map(do_nothing);\n   |     ^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(_y) = y { do_nothing(_y) }`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/lib.rs:111:22
