plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 55ccbd090d96ec3bb28dbcb383e65bbfa3c293ff and 5bd3776e1d1f1ae774ab1fe4acb3e5e33608806e
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 error: trait objects without an explicit `dyn` are deprecated
-  --> $DIR/ice-3969.rs:25:17
+  --> $DIR/ice-3969.rs:27:16
    |
-LL |     for<'a> Dst<A + 'a>: Sized,
-   |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`
+LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
+   |                ^ help: use `dyn`: `dyn A`
    |
    = note: `-D bare-trait-objects` implied by `-D warnings`
    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
 
 error: trait objects without an explicit `dyn` are deprecated
-  --> $DIR/ice-3969.rs:27:16
-  --> $DIR/ice-3969.rs:27:16
+  --> $DIR/ice-3969.rs:27:57
    |
 LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
-   |                ^ help: use `dyn`: `dyn A`
+   |                                                         ^ help: use `dyn`: `dyn A`
    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
 
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
 error: trait objects without an explicit `dyn` are deprecated
-  --> $DIR/ice-3969.rs:27:57
+  --> $DIR/ice-3969.rs:25:17
    |
-LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
-   |                                                         ^ help: use `dyn`: `dyn A`
+LL |     for<'a> Dst<A + 'a>: Sized,
+   |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`
    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
 
 error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args crashes/ice-3969.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-3969.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-3969.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c86912d687f6474b.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bc8a65c262f269f6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-e338b85031c86fb1.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a607561cd2d7a9ff.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-32f055fbf7836c6f.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0add1c52618c6f7c.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-3969.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":623,"byte_end":624,"line_start":27,"line_end":27,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":623,"byte_end":624,"line_start":27,"line_end":27,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":"dyn A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:27:16\n   |\nLL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);\n   |                ^ help: use `dyn`: `dyn A`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>\n\n"}
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":664,"byte_end":665,"line_start":27,"line_end":27,"column_start":57,"column_end":58,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":57,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":664,"byte_end":665,"line_start":27,"line_end":27,"column_start":57,"column_end":58,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":57,"highlight_end":58}],"label":null,"suggested_replacement":"dyn A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:27:57\n   |\nLL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);\n   |                                                         ^ help: use `dyn`: `dyn A`\n   |\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>\n\n"}
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":"dyn A + 'a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:25:17\n   |\nLL |     for<'a> Dst<A + 'a>: Sized,\n   |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`\n   |\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
