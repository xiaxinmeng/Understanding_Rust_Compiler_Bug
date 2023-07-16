plain
Set({"library/std"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 2.856 seconds
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/checkout/library/std)
error[E0005]: refutable pattern in local binding: `ScmCredentials(_)` not covered
    |
    |
510 |       let AncillaryData::ScmRights(scm_rights) = ancillary_data_vec.pop().unwrap().unwrap();
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `ScmCredentials(_)` not covered
   ::: library/std/src/os/./unix/net/ancillary.rs:275:1
    |
    |
275 | / pub enum AncillaryData<'a> {
276 | |     ScmRights(ScmRights<'a>),
277 | |     #[cfg(any(doc, target_os = "android", target_os = "linux",))]
278 | |     ScmCredentials(ScmCredentials<'a>),
279 | | }
279 | | }
    | |_- `ancillary::AncillaryData` defined here
    |
    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
    = note: the matched value is of type `ancillary::AncillaryData`
help: you might want to use `if let` to ignore the variant that isn't matched
    |
510 |     if let AncillaryData::ScmRights(scm_rights) = ancillary_data_vec.pop().unwrap().unwrap() { /* */ }


error[E0005]: refutable pattern in local binding: `ScmCredentials(_)` not covered
    |
    |
628 |       let AncillaryData::ScmRights(scm_rights) = ancillary_data_vec.pop().unwrap().unwrap();
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `ScmCredentials(_)` not covered
   ::: library/std/src/os/./unix/net/ancillary.rs:275:1
    |
    |
275 | / pub enum AncillaryData<'a> {
276 | |     ScmRights(ScmRights<'a>),
277 | |     #[cfg(any(doc, target_os = "android", target_os = "linux",))]
278 | |     ScmCredentials(ScmCredentials<'a>),
279 | | }
279 | | }
    | |_- `ancillary::AncillaryData` defined here
    |
    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
    = note: the matched value is of type `ancillary::AncillaryData`
help: you might want to use `if let` to ignore the variant that isn't matched
    |
628 |     if let AncillaryData::ScmRights(scm_rights) = ancillary_data_vec.pop().unwrap().unwrap() { /* */ }

For more information about this error, try `rustc --explain E0005`.
error: could not compile `std` due to 2 previous errors



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:18:57
