\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/crashes/used_underscore_binding_macro.rs","byte_start":245,"byte_end":256,"line_start":11,"line_end":11,"column_start":10,"column_end":21,"is_primary":true,"text":[{"text":"#[derive(Deserialize)]","highlight_start":10,"highlight_end":21}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/crashes/used_underscore_binding_macro.rs","byte_start":245,"byte_end":256,"line_start":11,"line_end":11,"column_start":10,"column_end":21,"is_primary":false,"text":[{"text":"#[derive(Deserialize)]","highlight_start":10,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Deserialize)]","def_site_span":{"file_name":"/cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.81/src/lib.rs","byte_start":1923,"byte_end":2146,"line_start":85,"line_end":90,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0463]: can't find crate for `serde`\n  --> tests/ui/crashes/used_underscore_binding_macro.rs:11:10\n   |\nLL | #[derive(Deserialize)]\n   |          ^^^^^^^^^^^ can't find crate\n\n"}
2019-11-10T03:15:22.2671955Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-10T03:15:22.2672011Z 
2019-11-10T03:15:22.2672187Z ------------------------------------------
2019-11-10T03:15:22.2672212Z 
---
2019-11-10T03:15:22.2675200Z For more information about this error, try `rustc --explain E0463`.
2019-11-10T03:15:22.2675227Z 
2019-11-10T03:15:22.2675245Z 
2019-11-10T03:15:22.2675276Z expected stderr:
2019-11-10T03:15:22.2675329Z error: you should not implement `visit_string` without also implementing `visit_str`
2019-11-10T03:15:22.2675528Z    |
2019-11-10T03:15:22.2675528Z    |
2019-11-10T03:15:22.2675738Z LL | /     fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
2019-11-10T03:15:22.2675775Z LL | |     where
2019-11-10T03:15:22.2675813Z LL | |         E: serde::de::Error,
2019-11-10T03:15:22.2675893Z LL | |         unimplemented!()
2019-11-10T03:15:22.2675924Z LL | |     }
2019-11-10T03:15:22.2675953Z    | |_____^
2019-11-10T03:15:22.2676001Z    |
---
2019-11-10T03:15:22.2676408Z 
2019-11-10T03:15:22.2676426Z 
2019-11-10T03:15:22.2676455Z diff of stderr:
2019-11-10T03:15:22.2676492Z 
2019-11-10T03:15:22.2676728Z -error: you should not implement `visit_string` without also implementing `visit_str`
2019-11-10T03:15:22.2676961Z +error[E0465]: multiple rlib candidates for `serde` found
2019-11-10T03:15:22.2677154Z +  --> $DIR/serde.rs:4:1
2019-11-10T03:15:22.2677190Z     |
2019-11-10T03:15:22.2677190Z     |
2019-11-10T03:15:22.2677407Z -LL | /     fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
2019-11-10T03:15:22.2677597Z -LL | |     where
2019-11-10T03:15:22.2677781Z -LL | |         E: serde::de::Error,
2019-11-10T03:15:22.2678142Z -LL | |         unimplemented!()
2019-11-10T03:15:22.2678311Z -LL | |     }
2019-11-10T03:15:22.2678482Z -   | |_____^
2019-11-10T03:15:22.2678519Z +LL | extern crate serde;
2019-11-10T03:15:22.2678519Z +LL | extern crate serde;
2019-11-10T03:15:22.2678568Z +   | ^^^^^^^^^^^^^^^^^^^
2019-11-10T03:15:22.2678598Z     |
2019-11-10T03:15:22.2678980Z -   = note: `-D clippy::serde-api-misuse` implied by `-D warnings`
2019-11-10T03:15:22.2679490Z +note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-3f5da7051f958e60.rlib
2019-11-10T03:15:22.2679728Z +   |
2019-11-10T03:15:22.2679781Z +LL | extern crate serde;
2019-11-10T03:15:22.2679817Z +   | ^^^^^^^^^^^^^^^^^^^
2019-11-10T03:15:22.2679817Z +   | ^^^^^^^^^^^^^^^^^^^
2019-11-10T03:15:22.2680123Z +note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ce92f2ea308a129e.rlib
2019-11-10T03:15:22.2680374Z +   |
2019-11-10T03:15:22.2680409Z +LL | extern crate serde;
2019-11-10T03:15:22.2680463Z +   | ^^^^^^^^^^^^^^^^^^^
2019-11-10T03:15:22.2680498Z  
---
2019-11-10T03:15:22.2682750Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'serde.rs'
2019-11-10T03:15:22.2682792Z 
2019-11-10T03:15:22.2682830Z error: 1 errors occurred comparing output.
2019-11-10T03:15:22.2682885Z status: exit code: 1
2019-11-10T03:15:22.2684571Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/serde.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/serde.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-3f5da7051f958e60.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-9a05b9c0002591cb.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ce92f2ea308a129e.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-b2f045757547a7b1.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/serde.stage-id.aux" "-A" "unused"
2019-11-10T03:15:22.2685302Z ------------------------------------------
2019-11-10T03:15:22.2685331Z 
2019-11-10T03:15:22.2685542Z ------------------------------------------
2019-11-10T03:15:22.2685581Z stderr:
2019-11-10T03:15:22.2685581Z stderr:
2019-11-10T03:15:22.2685771Z ------------------------------------------
2019-11-10T03:15:22.2688059Z {"message":"multiple rlib candidates for `serde` found","code":{"code":"E0465","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/serde.rs","byte_start":57,"byte_end":76,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate serde;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-3f5da7051f958e60.rlib","code":null,"level":"note","spans":[{"file_name":"tests/ui/serde.rs","byte_start":57,"byte_end":76,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate serde;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ce92f2ea308a129e.rlib","code":null,"level":"note","spans":[{"file_name":"tests/ui/serde.rs","byte_start":57,"byte_end":76,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate serde;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0465]: multiple rlib candidates for `serde` found\n  --> tests/ui/serde.rs:4:1\n   |\nLL | extern crate serde;\n   | ^^^^^^^^^^^^^^^^^^^\n   |\nnote: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-3f5da7051f958e60.rlib\n  --> tests/ui/serde.rs:4:1\n   |\nLL | extern crate serde;\n   | ^^^^^^^^^^^^^^^^^^^\nnote: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ce92f2ea308a129e.rlib\n  --> tests/ui/serde.rs:4:1\n   |\nLL | extern crate serde;\n   | ^^^^^^^^^^^^^^^^^^^\n\n"}
2019-11-10T03:15:22.2689669Z {"message":"can't find crate for `serde`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n