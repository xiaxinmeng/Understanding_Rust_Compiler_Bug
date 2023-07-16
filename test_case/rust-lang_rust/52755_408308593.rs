
[01:21:27] failures:
[01:21:27] 
[01:21:27] ---- [ui] ui/redundant_field_names.rs stdout ----
[01:21:27] normalized stderr:
[01:21:27] error[E0464]: multiple matching crates for `derive_new`
[01:21:27]  --> $DIR/redundant_field_names.rs:6:1
[01:21:27]   |
[01:21:27] 6 | extern crate derive_new;
[01:21:27]   | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:21:27]   |
[01:21:27]   = note: candidates:
[01:21:27]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:21:27]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:21:27] 
[01:21:27] error[E0463]: can't find crate for `derive_new`
[01:21:27]  --> $DIR/redundant_field_names.rs:6:1
[01:21:27]   |
[01:21:27] 6 | extern crate derive_new;
[01:21:27]   | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:21:27] 
[01:21:27] error: aborting due to 2 previous errors
[01:21:27] 
[01:21:27] Some errors occurred: E0463, E0464.
[01:21:27] For more information about an error, try `rustc --explain E0463`.
[01:21:27] 
[01:21:27] 
[01:21:27] expected stderr:
[01:21:27] error: redundant field names in struct initialization
[01:21:27]   --> $DIR/redundant_field_names.rs:34:9
[01:21:27]    |
[01:21:27] 34 |         gender: gender,
[01:21:27]    |         ^^^^^^^^^^^^^^ help: replace it with: `gender`
[01:21:27]    |
[01:21:27]    = note: `-D redundant-field-names` implied by `-D warnings`
[01:21:27] 
[01:21:27] error: redundant field names in struct initialization
[01:21:27]   --> $DIR/redundant_field_names.rs:35:9
[01:21:27]    |
[01:21:27] 35 |         age: age,
[01:21:27]    |         ^^^^^^^^ help: replace it with: `age`
[01:21:27] 
[01:21:27] error: redundant field names in struct initialization
[01:21:27]   --> $DIR/redundant_field_names.rs:53:25
[01:21:27]    |
[01:21:27] 53 |     let _ = RangeFrom { start: start };
[01:21:27]    |                         ^^^^^^^^^^^^ help: replace it with: `start`
[01:21:27] 
[01:21:27] error: redundant field names in struct initialization
[01:21:27]   --> $DIR/redundant_field_names.rs:54:23
[01:21:27]    |
[01:21:27] 54 |     let _ = RangeTo { end: end };
[01:21:27]    |                       ^^^^^^^^ help: replace it with: `end`
[01:21:27] 
[01:21:27] error: redundant field names in struct initialization
[01:21:27]   --> $DIR/redundant_field_names.rs:55:21
[01:21:27]    |
[01:21:27] 55 |     let _ = Range { start: start, end: end };
[01:21:27]    |                     ^^^^^^^^^^^^ help: replace it with: `start`
[01:21:27] 
[01:21:27] error: redundant field names in struct initialization
[01:21:27]   --> $DIR/redundant_field_names.rs:55:35
[01:21:27]    |
[01:21:27] 55 |     let _ = Range { start: start, end: end };
[01:21:27]    |                                   ^^^^^^^^ help: replace it with: `end`
[01:21:27] 
[01:21:27] error: redundant field names in struct initialization
[01:21:27]   --> $DIR/redundant_field_names.rs:57:32
[01:21:27]    |
[01:21:27] 57 |     let _ = RangeToInclusive { end: end };
[01:21:27]    |                                ^^^^^^^^ help: replace it with: `end`
[01:21:27] 
[01:21:27] error: aborting due to 7 previous errors
[01:21:27] 
[01:21:27] 
[01:21:27] 
[01:21:27] diff of stderr:
[01:21:27] 
[01:21:27] -error: redundant field names in struct initialization
[01:21:27] -  --> $DIR/redundant_field_names.rs:34:9
[01:21:27] -   |
[01:21:27] -34 |         gender: gender,
[01:21:27] -   |         ^^^^^^^^^^^^^^ help: replace it with: `gender`
[01:21:27] -   |
[01:21:27] -   = note: `-D redundant-field-names` implied by `-D warnings`
[01:21:27] +error[E0464]: multiple matching crates for `derive_new`
[01:21:27] + --> $DIR/redundant_field_names.rs:6:1
[01:21:27] +  |
[01:21:27] +6 | extern crate derive_new;
[01:21:27] +  | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:21:27] +  |
[01:21:27] +  = note: candidates:
[01:21:27] +          crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:21:27] +          crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:21:27]  
[01:21:27] -error: redundant field names in struct initialization
[01:21:27] -  --> $DIR/redundant_field_names.rs:35:9
[01:21:27] -   |
[01:21:27] -35 |         age: age,
[01:21:27] -   |         ^^^^^^^^ help: replace it with: `age`
[01:21:27] +error[E0463]: can't find crate for `derive_new`
[01:21:27] + --> $DIR/redundant_field_names.rs:6:1
[01:21:27] +  |
[01:21:27] +6 | extern crate derive_new;
[01:21:27] +  | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:21:27]  
[01:21:27] -error: redundant field names in struct initialization
[01:21:27] -  --> $DIR/redundant_field_names.rs:53:25
[01:21:27] -   |
[01:21:27] -53 |     let _ = RangeFrom { start: start };
[01:21:27] -   |                         ^^^^^^^^^^^^ help: replace it with: `start`
[01:21:27] +error: aborting due to 2 previous errors
[01:21:27]  
[01:21:27] -error: redundant field names in struct initialization
[01:21:27] -  --> $DIR/redundant_field_names.rs:54:23
[01:21:27] -   |
[01:21:27] -54 |     let _ = RangeTo { end: end };
[01:21:27] -   |                       ^^^^^^^^ help: replace it with: `end`
[01:21:27] -
[01:21:27] -error: redundant field names in struct initialization
[01:21:27] -  --> $DIR/redundant_field_names.rs:55:21
[01:21:27] -   |
[01:21:27] -55 |     let _ = Range { start: start, end: end };
[01:21:27] -   |                     ^^^^^^^^^^^^ help: replace it with: `start`
[01:21:27] -
[01:21:27] -error: redundant field names in struct initialization
[01:21:27] -  --> $DIR/redundant_field_names.rs:55:35
[01:21:27] -   |
[01:21:27] -55 |     let _ = Range { start: start, end: end };
[01:21:27] -   |                                   ^^^^^^^^ help: replace it with: `end`
[01:21:27] -
[01:21:27] -error: redundant field names in struct initialization
[01:21:27] -  --> $DIR/redundant_field_names.rs:57:32
[01:21:27] -   |
[01:21:27] -57 |     let _ = RangeToInclusive { end: end };
[01:21:27] -   |                                ^^^^^^^^ help: replace it with: `end`
[01:21:27] -
[01:21:27] -error: aborting due to 7 previous errors
[01:21:27] -
[01:21:27] +Some errors occurred: E0463, E0464.
[01:21:27] +For more information about an error, try `rustc --explain E0463`.
[01:21:27]  
[01:21:27] 
[01:21:27] The actual stderr differed from the expected stderr.
[01:21:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/redundant_field_names.stderr
[01:21:27] To update references, run this command from build directory:
[01:21:27] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base' 'redundant_field_names.rs'
[01:21:27] 
[01:21:27] error: 1 errors occurred comparing output.
[01:21:27] status: exit code: 101
[01:21:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/redundant_field_names.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/redundant_field_names.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f23daa848079a0d5/out/test_build_base/redundant_field_names.stage-id.aux" "-A" "unused"
[01:21:27] stdout:
[01:21:27] ------------------------------------------
[01:21:27] 
[01:21:27] ------------------------------------------
[01:21:27] stderr:
[01:21:27] ------------------------------------------
[01:21:27] error[E0464]: multiple matching crates for `derive_new`
[01:21:27]  --> tests/ui/redundant_field_names.rs:6:1
[01:21:27]   |
[01:21:27] 6 | extern crate derive_new;
[01:21:27]   | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:21:27]   |
[01:21:27]   = note: candidates:
[01:21:27]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:21:27]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:21:27] 
[01:21:27] error[E0463]: can't find crate for `derive_new`
[01:21:27]  --> tests/ui/redundant_field_names.rs:6:1
[01:21:27]   |
[01:21:27] 6 | extern crate derive_new;
[01:21:27]   | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:21:27] 
[01:21:27] error: aborting due to 2 previous errors
[01:21:27] 
[01:21:27] Some errors occurred: E0463, E0464.
[01:21:27] For more information about an error, try `rustc --explain E0463`.
[01:21:27] 
[01:21:27] ------------------------------------------
[01:21:27] 
[01:21:27] thread '[ui] ui/redundant_field_names.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.11/src/runtest.rs:2544:9
[01:21:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:21:27] 
[01:21:27] 
[01:21:27] failures:
[01:21:27]     [ui] ui/redundant_field_names.rs
[01:21:27] 
[01:21:27] test result: FAILED. 200 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:21:27] 
[01:21:27] test compile_test ... FAILED
[01:21:27] 
[01:21:27] failures:
[01:21:27] 
[01:21:27] ---- compile_test stdout ----
[01:21:27] thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.11/src/lib.rs:89:22
[01:21:27] 
[01:21:27] 
[01:21:27] failures:
[01:21:27]     compile_test
[01:21:27] 
[01:21:27] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:21:27] 
[01:21:27] [0m[0m[1m[31merror:[0m test failed, to rerun pass '--test compile-test'
[01:21:27] 
[01:21:27] 
[01:21:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml"
[01:21:27] expected success, got: exit code: 101
[01:21:27] 
[01:21:27] 
[01:21:27] 
[01:21:27] 1 command(s) did not execute successfully:
[01:21:27] 
[01:21:27]   - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml"
[01:21:27] 
[01:21:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/tools/rls src/tools/rustfmt src/tools/miri src/tools/clippy
[01:21:27] Build completed unsuccessfully in 1:18:30
[01:21:27] {"rustfmt":"test-pass","clippy-driver":"test-fail","book":"test-pass","miri":"build-fail","rls":"build-fail","nomicon":"test-pass","reference":"test-pass","rust-by-example":"test-pass"}
