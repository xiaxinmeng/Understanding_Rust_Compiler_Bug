plain
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `unreachable`
  --> library/core/src/hint.rs:25:37
   |
25 | /// Otherwise, consider using the [`unreachable!`] macro, which does not allow
   |                                     ^^^^^^^^^^^^ no item named `unreachable` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `write`
   --> library/core/src/fmt/mod.rs:166:33
    |
    |
166 |     /// Glue for usage of the [`write!`] macro with implementors of this trait.
    |                                 ^^^^^^ this link resolves to the function `write`, which is not in the macro namespace
help: to link to the function, add parentheses
    |
    |
166 |     /// Glue for usage of the [`write!()`] macro with implementors of this trait.

error: unresolved link to `write`
   --> library/core/src/fmt/mod.rs:169:15
    |
    |
169 |     /// the [`write!`] macro itself.
    |               ^^^^^^ this link resolves to the function `write`, which is not in the macro namespace
help: to link to the function, add parentheses
    |
    |
169 |     /// the [`write!()`] macro itself.

error: unresolved link to `panic`
   --> library/core/src/macros/mod.rs:171:28
    |
    |
171 | /// This will invoke the [`panic!`] macro if the provided expression cannot be
    |                            ^^^^^^ no item named `panic` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `assert_eq`
    |
    |
223 | /// Unlike [`assert_eq!`], `debug_assert_eq!` statements are only enabled in non
    |              ^^^^^^^^^^ no item named `assert_eq` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `assert_ne`
    |
    |
248 | /// Unlike [`assert_ne!`], `debug_assert_ne!` statements are only enabled in non
    |              ^^^^^^^^^^ no item named `assert_ne` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `write`
   --> library/core/src/macros/mod.rs:487:33
    |
    |
487 | /// For more information, see [`write!`]. For information on the format string syntax, see
    |
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `panic`
   --> library/core/src/macros/mod.rs:548:45
    |
    |
548 | /// program immediately terminates with a [`panic!`].
    |                                             ^^^^^^ no item named `panic` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `panic`
   --> library/core/src/macros/mod.rs:557:24
    |
    |
557 | /// This will always [`panic!`] because `unreachable!` is just a shorthand for `panic!` with a
    |                        ^^^^^^ no item named `panic` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `todo`
    |
    |
609 | /// The difference between `unimplemented!` and [`todo!`] is that while `todo!`
    |                                                   ^^^^^ no item named `todo` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `panic`
   --> library/core/src/macros/mod.rs:616:24
    |
    |
616 | /// This will always [`panic!`] because `unimplemented!` is just a shorthand for `panic!` with a
    |                        ^^^^^^ no item named `panic` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `unimplemented`
    |
    |
687 | /// The difference between [`unimplemented!`] and `todo!` is that while `todo!` conveys
    |                              ^^^^^^^^^^^^^^ no item named `unimplemented` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `panic`
   --> library/core/src/macros/mod.rs:694:24
    |
    |
694 | /// This will always [`panic!`].
    |                        ^^^^^^ no item named `panic` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `panic`
   --> library/core/src/macros/mod.rs:753:91
    |
    |
753 |     /// better error messages for erroneous conditions. It's the compiler-level form of [`panic!`],
    |                                                                                           ^^^^^^ no item named `panic` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `write`
   --> library/core/src/macros/mod.rs:801:53
    |
    |
801 |     /// All other formatting macros ([`format!`], [`write!`], [`println!`], etc) are
    |
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `panic`
    --> library/core/src/macros/mod.rs:1282:32
     |
     |
1282 |     /// This will invoke the [`panic!`] macro if the provided expression cannot be
     |                                ^^^^^^ no item named `panic` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `debug_assert`
    --> library/core/src/macros/mod.rs:1288:28
     |
     |
1288 |     /// be disabled. See [`debug_assert!`] for assertions that are not enabled in
     |                            ^^^^^^^^^^^^^ no item named `debug_assert` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.59.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.59.0-nightly
  (b782ab7fa
  2021-12-01)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.59.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:31:53
