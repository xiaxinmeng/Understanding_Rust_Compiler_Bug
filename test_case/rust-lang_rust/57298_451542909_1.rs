
/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy 
$ time cargo install --path . --force
...
    Finished release [optimized] target(s) in 8m 40s
   Replacing /home/xftroxgpx/.cargo/bin/cargo-clippy
   Replacing /home/xftroxgpx/.cargo/bin/clippy-driver

real	8m40.501s
user	24m2.846s
sys	0m11.862s

/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy 
$ time ./target/release/cargo-clippy 
...
    Finished dev [unoptimized + debuginfo] target(s) in 7m 53s

real	7m54.015s
user	20m18.805s
sys	0m20.590s

/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy 
$ cargo test --release -- --test-threads 4
...
    Finished release [optimized] target(s) in 7m 39s
     Running target/release/deps/compile_test-9cf56f1de0290b37

running 1 test

running 30 tests
test [run-pass] run-pass/ice-1588.rs ... ok
test [run-pass] run-pass/cc_seme.rs ... ok
test [run-pass] run-pass/enum-glob-import-crate.rs ... ok
test [run-pass] run-pass/associated-constant-ice.rs ... ok
test [run-pass] run-pass/ice-1969.rs ... ok
test [run-pass] run-pass/ice-1782.rs ... ok
test [run-pass] run-pass/ice-2499.rs ... ok
test [run-pass] run-pass/ice-2594.rs ... ok
test [run-pass] run-pass/ice-2727.rs ... ok
test [run-pass] run-pass/ice-2760.rs ... ok
test [run-pass] run-pass/ice-2865.rs ... ok
test [run-pass] run-pass/ice-2774.rs ... ok
test [run-pass] run-pass/ice-3462.rs ... ok
test [run-pass] run-pass/ice-3151.rs ... ok
test [run-pass] run-pass/ice-700.rs ... ok
test [run-pass] run-pass/ice_exacte_size.rs ... ok
test [run-pass] run-pass/if_same_then_else.rs ... ok
test [run-pass] run-pass/issue-2862.rs ... ok
test [run-pass] run-pass/issue-825.rs ... ok
test [run-pass] run-pass/issues_loop_mut_cond.rs ... ok
test [run-pass] run-pass/match_same_arms_const.rs ... ok
test [run-pass] run-pass/mut_mut_macro.rs ... ok
test [run-pass] run-pass/needless_borrow_fp.rs ... ok
test [run-pass] run-pass/needless_lifetimes_impl_trait.rs ... ok
test [run-pass] run-pass/procedural_macro.rs ... ok
test [run-pass] run-pass/regressions.rs ... ok
test [run-pass] run-pass/used_underscore_binding_macro.rs ... FAILED
test [run-pass] run-pass/returns.rs ... ok
test [run-pass] run-pass/single-match-else.rs ... ok
test [run-pass] run-pass/whitelist/conf_whitelisted.rs ... ok

failures:

---- [run-pass] run-pass/used_underscore_binding_macro.rs stdout ----

error: compilation failed!
status: exit code: 1
command: "target/release/clippy-driver" "tests/run-pass/used_underscore_binding_macro.rs" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/test_build_base/used_underscore_binding_macro.stage-id" "-L" "target/release" "-L" "target/release/deps" "-Dwarnings" "-Zui-testing" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/test_build_base/used_underscore_binding_macro.stage-id.aux"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{"message":"multiple matching crates for `serde_derive`","code":{"code":"E0464","explanation":null},"level":"error","spans":[{"file_name":"tests/run-pass/used_underscore_binding_macro.rs","byte_start":543,"byte_end":569,"line_start":14,"line_end":14,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate serde_derive;","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"candidates:\ncrate `serde_derive`: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde_derive-3f71afdce6a2028c.so\ncrate `serde_derive`: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde_derive-17bc36f26561caa5.so","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0464]: multiple matching crates for `serde_derive`\n  --> tests/run-pass/used_underscore_binding_macro.rs:14:1\n   |\nLL | extern crate serde_derive;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: candidates:\n           crate `serde_derive`: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde_derive-3f71afdce6a2028c.so\n           crate `serde_derive`: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde_derive-17bc36f26561caa5.so\n\n"}
{"message":"can't find crate for `serde_derive`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n