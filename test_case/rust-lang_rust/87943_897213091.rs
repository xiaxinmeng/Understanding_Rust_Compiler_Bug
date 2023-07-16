plain

 error: trait objects without an explicit `dyn` are deprecated
   --> $DIR/ice-3969.rs:25:17
    |
 LL |     for<'a> Dst<A + 'a>: Sized,
    |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`
    |
    = note: `-D bare-trait-objects` implied by `-D warnings`
-   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>
+   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
 
 error: trait objects without an explicit `dyn` are deprecated
 error: trait objects without an explicit `dyn` are deprecated
   --> $DIR/ice-3969.rs:27:16
    |
 LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
    |                ^ help: use `dyn`: `dyn A`
    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
-   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>
+   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
 
 
 error: trait objects without an explicit `dyn` are deprecated
   --> $DIR/ice-3969.rs:27:57
    |
 LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
    |                                                         ^ help: use `dyn`: `dyn A`
    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
-   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>
+   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
 
---
To only update this specific test, also pass `--test-args crashes/ice-3969.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/crashes/ice-3969.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-3969.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a4851abe566518b2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-3969.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":"dyn A + 'a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:25:17\n   |\nLL |     for<'a> Dst<A + 'a>: Sized,\n   |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>\n\n"}
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":623,"byte_end":624,"line_start":27,"line_end":27,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":623,"byte_end":624,"line_start":27,"line_end":27,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":"dyn A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:27:16\n   |\nLL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);\n   |                ^ help: use `dyn`: `dyn A`\n   |\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>\n\n"}
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":664,"byte_end":665,"line_start":27,"line_end":27,"column_start":57,"column_end":58,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":57,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":664,"byte_end":665,"line_start":27,"line_end":27,"column_start":57,"column_end":58,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":57,"highlight_end":58}],"label":null,"suggested_replacement":"dyn A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:27:57\n   |\nLL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);\n   |                                                         ^ help: use `dyn`: `dyn A`\n   |\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
