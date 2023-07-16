plain
...........................................................iii.......................... 13024/13101
.............................................................................
failures:

---- [ui] src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-blanket-impl-local-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-blanket-impl-local-trait/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:11:24
   |
   |
LL | impl LocalTraitTwo for LocalTraitOne {}
   |
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - impl LocalTraitTwo for LocalTraitOne {}
LL + impl LocalTraitTwo for dyn LocalTraitOne {}
   |
help: use a blanket implementation to implement LocalTraitTwo for all types that also implement LocalTraitOne
   |
LL | impl <T: LocalTraitOne> for T {}
   |      ~~~          ++++++++++~+++
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:15:23
   |
   |
LL | impl fmt::Display for LocalTraitOne { //~ WARNING use of dyn syntax
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - impl fmt::Display for LocalTraitOne { //~ WARNING use of dyn syntax
LL + impl fmt::Display for dyn LocalTraitOne { //~ WARNING use of dyn syntax

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:21:23
   |
   |
LL | impl fmt::Display for LocalTraitTwo + Send { //~ WARNING use of dyn syntax
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - impl fmt::Display for LocalTraitTwo + Send { //~ WARNING use of dyn syntax
LL + impl fmt::Display for dyn LocalTraitTwo + Send { //~ WARNING use of dyn syntax

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:11:24
   |
   |
LL | impl LocalTraitTwo for LocalTraitOne {}
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - impl LocalTraitTwo for LocalTraitOne {}
LL + impl LocalTraitTwo for dyn LocalTraitOne {}
   |
help: use a blanket implementation to implement LocalTraitTwo for all types that also implement LocalTraitOne
   |
LL | impl <T: LocalTraitOne> for T {}
   |      ~~~          ++++++++++~+++
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:15:23
   |
   |
LL | impl fmt::Display for LocalTraitOne { //~ WARNING use of dyn syntax
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - impl fmt::Display for LocalTraitOne { //~ WARNING use of dyn syntax
LL + impl fmt::Display for dyn LocalTraitOne { //~ WARNING use of dyn syntax

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:21:23
   |
   |
LL | impl fmt::Display for LocalTraitTwo + Send { //~ WARNING use of dyn syntax
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - impl fmt::Display for LocalTraitTwo + Send { //~ WARNING use of dyn syntax
LL + impl fmt::Display for dyn LocalTraitTwo + Send { //~ WARNING use of dyn syntax

warning: 6 warnings emitted
------------------------------------------

