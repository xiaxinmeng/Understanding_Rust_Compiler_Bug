plain
[RUSTC-TIMING] ignore test:false 19.150
error[E0599]: no method named `expect_none` found for enum `Option<Fingerprint>` in the current scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/lib.rs:2003:48
     |
2003 |                 cache[index].replace(sub_hash).expect_none("Cache slot was filled");

error[E0599]: no method named `expect_none` found for enum `Option<u32>` in the current scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/hygiene.rs:121:54
    |
---
   Compiling rls-ipc v0.1.0 (/checkout/src/tools/rls/rls-ipc)
error[E0599]: no method named `expect_none` found for enum `Option<&u32>` in the current scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/hygiene.rs:1421:22
     |
1421 |                     .expect_none("Hash collision after disambiguator update!");
     |                      ^^^^^^^^^^^ method not found in `Option<&u32>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] rustc_ap_rustc_span test:false 1.869
---
   Compiling rustc-ap-rustc_span v705.0.0
error[E0599]: no method named `expect_none` found for enum `Option<Fingerprint>` in the current scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/lib.rs:2003:48
     |
2003 |                 cache[index].replace(sub_hash).expect_none("Cache slot was filled");

[RUSTC-TIMING] rustc_ap_rustc_data_structures test:false 3.923
error[E0599]: no method named `expect_none` found for enum `Option<u32>` in the current scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/hygiene.rs:121:54
---

error[E0599]: no method named `expect_none` found for enum `Option<&u32>` in the current scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/hygiene.rs:1421:22
     |
1421 |                     .expect_none("Hash collision after disambiguator update!");
     |                      ^^^^^^^^^^^ method not found in `Option<&u32>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] rustc_ap_rustc_span test:false 1.896
---
Verifying status of rustfmt...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` has regressed from test-pass to build-fail during beta week.
error: Tool `rustfmt` has regressed from test-pass to build-fail during beta week.
{"rust-by-example":"test-pass","rustfmt":"build-fail","book":"test-pass","edition-guide":"test-pass","cargo-miri":"test-fail","rls":"build-fail","nomicon":"test-pass","embedded-book":"test-pass","reference":"test-pass","rustbook":"test-fail","miri":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
