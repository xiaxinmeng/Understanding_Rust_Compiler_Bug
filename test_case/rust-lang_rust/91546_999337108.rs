plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.17s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `str::rsplit_inclusive`
    |
    |
474 |           $(#[$reverse_iterator_attribute])*
...
...
811 | / generate_pattern_iterators! {
813 | |     forward:
813 | |     forward:
814 | |         #[stable(feature = "split_inclusive", since = "1.51.0")]
829 | |     delegate double ended;
830 | | }
    | |_- in this macro invocation
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            [`rsplit_inclusive`]: str::rsplit_inclusive
                                  ^^^^^^^^^^^^^^^^^^^^^
    = note: the builtin type `str` has no associated item named `rsplit_inclusive`
    = note: this error originates in the macro `generate_pattern_iterators` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `str::rsplit_left_inclusive`
    |
    |
474 |           $(#[$reverse_iterator_attribute])*
...
...
842 | / generate_pattern_iterators! {
843 | |     forward:
844 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
845 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
859 | |     delegate double ended;
860 | | }
    | |_- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            [`rsplit_left_inclusive`]: str::rsplit_left_inclusive
                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: the builtin type `str` has no associated item named `rsplit_left_inclusive`
    = note: this error originates in the macro `generate_pattern_iterators` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `str::splitn_inclusive`
     |
     |
437  |           $(#[$forward_iterator_attribute])*
...
...
1035 | / generate_pattern_iterators! {
1036 | |     forward:
1037 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1038 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1052 | |     delegate single ended;
1053 | | }
     | |_- in this macro invocation
     |
     |
     = note: the link appears in this line:
             
             [`splitn_inclusive`]: str::splitn_inclusive
                                   ^^^^^^^^^^^^^^^^^^^^^
     = note: the builtin type `str` has no associated item named `splitn_inclusive`
     = note: this error originates in the macro `generate_pattern_iterators` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `str::rsplitn_inclusive`
     |
     |
474  |           $(#[$reverse_iterator_attribute])*
...
...
1035 | / generate_pattern_iterators! {
1036 | |     forward:
1037 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1038 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1052 | |     delegate single ended;
1053 | | }
     | |_- in this macro invocation
     |
     |
     = note: the link appears in this line:
             
             [`rsplitn_inclusive`]: str::rsplitn_inclusive
                                    ^^^^^^^^^^^^^^^^^^^^^^
     = note: the builtin type `str` has no associated item named `rsplitn_inclusive`
     = note: this error originates in the macro `generate_pattern_iterators` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `str::splitn_left_inclusive`
     |
     |
437  |           $(#[$forward_iterator_attribute])*
...
...
1062 | / generate_pattern_iterators! {
1063 | |     forward:
1064 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1065 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1079 | |     delegate single ended;
1080 | | }
     | |_- in this macro invocation
     |
     |
     = note: the link appears in this line:
             
             [`splitn_left_inclusive`]: str::splitn_left_inclusive
                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: the builtin type `str` has no associated item named `splitn_left_inclusive`
     = note: this error originates in the macro `generate_pattern_iterators` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `str::rsplitn_left_inclusive`
     |
     |
474  |           $(#[$reverse_iterator_attribute])*
...
...
1062 | / generate_pattern_iterators! {
1063 | |     forward:
1064 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1065 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1079 | |     delegate single ended;
1080 | | }
     | |_- in this macro invocation
     |
     |
     = note: the link appears in this line:
             
             [`rsplitn_left_inclusive`]: str::rsplitn_left_inclusive
                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: the builtin type `str` has no associated item named `rsplitn_left_inclusive`
     = note: this error originates in the macro `generate_pattern_iterators` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.59.0 --index-page /checkout/src/doc/index.md -C metadata=4599449636879869 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.59.0-nightly
  (01086579e
  2021-12-22)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.59.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:07
