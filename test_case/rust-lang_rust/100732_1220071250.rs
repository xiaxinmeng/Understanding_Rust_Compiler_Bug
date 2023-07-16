plain
........................................................................................ 6688/13403
.......i.......................................................ii.ii........i....i...... 6776/13403
.........................................................i.............................. 6864/13403
........................................................................................ 6952/13403
.............................i....i..............F....F.....................i........... 7040/13403
........................................................................................ 7216/13403
..........i............................................................................. 7304/13403
........................................................................................ 7392/13403
........................................................................................ 7480/13403
---

---- [ui] src/test/ui/linkage-attr/link-attr-validation-early.rs stdout ----
diff of stderr:

- error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...")]`
+ error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
3    |
4 LL | #[link]

8    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
8    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
9    = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
10 
- error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...")]`
+ error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
13    |
13    |
14 LL | #[link = "foo"]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-early/link-attr-validation-early.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args linkage-attr/link-attr-validation-early.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage-attr/link-attr-validation-early.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-early" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-early/auxiliary"
stdout: none
--- stderr -------------------------------
error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
   |
   |
LL | #[link] //~ ERROR attribute must be of the form
   |
   = note: `#[deny(ill_formed_attribute_input)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>

error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
   |
   |
LL | #[link = "foo"] //~ ERROR attribute must be of the form
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>


error: aborting due to 2 previous errors
------------------------------------------


---- [ui] src/test/ui/linkage-attr/link-attr-validation-late.rs stdout ----
diff of stderr:

- error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module
+ error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module, import_name_type
3    |
3    |
4 LL | #[link(name = "...", "literal")]
5    |                      ^^^^^^^^^
6 
6 
- error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module
+ error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module, import_name_type
9    |
10 LL | #[link(name = "...", unknown)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-late/link-attr-validation-late.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args linkage-attr/link-attr-validation-late.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage-attr/link-attr-validation-late.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-late" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-late/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module, import_name_type
   |
   |
LL | #[link(name = "...", "literal")] //~ ERROR unexpected `#[link]` argument


error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module, import_name_type
   |
   |
LL | #[link(name = "...", unknown)] //~ ERROR unexpected `#[link]` argument


error: multiple `name` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "foo", name = "bar")] //~ ERROR multiple `name` arguments


error: multiple `kind` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "...", kind = "dylib", kind = "bar")] //~ ERROR multiple `kind` arguments


error: multiple `modifiers` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "...", modifiers = "+verbatim", modifiers = "bar")] //~ ERROR multiple `modifiers` arguments


error: multiple `cfg` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "...", cfg(FALSE), cfg(FALSE))] //~ ERROR multiple `cfg` arguments


error: multiple `wasm_import_module` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(wasm_import_module = "foo", wasm_import_module = "bar")] //~ ERROR multiple `wasm_import_module` arguments


error: link name must be of the form `name = "string"`
   |
   |
LL | #[link(name)] //~ ERROR link name must be of the form `name = "string"`


error[E0459]: `#[link]` attribute requires a `name = "string"` argument
   |
   |
LL | #[link(name)] //~ ERROR link name must be of the form `name = "string"`
   | ^^^^^^^^^^^^^ missing `name` argument

error: link name must be of the form `name = "string"`
   |
   |
LL | #[link(name())] //~ ERROR link name must be of the form `name = "string"`


error[E0459]: `#[link]` attribute requires a `name = "string"` argument
   |
   |
LL | #[link(name())] //~ ERROR link name must be of the form `name = "string"`
   | ^^^^^^^^^^^^^^^ missing `name` argument

error: link kind must be of the form `kind = "string"`
   |
   |
LL | #[link(name = "...", kind)] //~ ERROR link kind must be of the form `kind = "string"`


error: link kind must be of the form `kind = "string"`
   |
   |
LL | #[link(name = "...", kind())] //~ ERROR link kind must be of the form `kind = "string"`


error: link modifiers must be of the form `modifiers = "string"`
   |
   |
LL | #[link(name = "...", modifiers)] //~ ERROR link modifiers must be of the form `modifiers = "string"`


error: link modifiers must be of the form `modifiers = "string"`
   |
   |
LL | #[link(name = "...", modifiers())] //~ ERROR link modifiers must be of the form `modifiers = "string"`


error: link cfg must be of the form `cfg(/* predicate */)`
   |
   |
LL | #[link(name = "...", cfg)] //~ ERROR link cfg must be of the form `cfg(/* predicate */)`


error: link cfg must be of the form `cfg(/* predicate */)`
   |
   |
LL | #[link(name = "...", cfg = "literal")] //~ ERROR link cfg must be of the form `cfg(/* predicate */)`


error: link cfg must have a single predicate argument
   |
   |
LL | #[link(name = "...", cfg("literal"))] //~ ERROR link cfg must have a single predicate argument


error: wasm import module must be of the form `wasm_import_module = "string"`
   |
   |
LL | #[link(name = "...", wasm_import_module)] //~ ERROR wasm import module must be of the form `wasm_import_module = "string"`


error: wasm import module must be of the form `wasm_import_module = "string"`
   |
   |
LL | #[link(name = "...", wasm_import_module())] //~ ERROR wasm import module must be of the form `wasm_import_module = "string"`


error: invalid linking modifier syntax, expected '+' or '-' prefix before one of: bundle, verbatim, whole-archive, as-needed
   |
   |
LL | #[link(name = "...", modifiers = "")] //~ ERROR invalid linking modifier syntax, expected '+' or '-' prefix


error: invalid linking modifier syntax, expected '+' or '-' prefix before one of: bundle, verbatim, whole-archive, as-needed
   |
   |
LL | #[link(name = "...", modifiers = "no-plus-minus")] //~ ERROR invalid linking modifier syntax, expected '+' or '-' prefix


error: unknown linking modifier `unknown`, expected one of: bundle, verbatim, whole-archive, as-needed
   |
   |
LL | #[link(name = "...", modifiers = "+unknown")] //~ ERROR unknown linking modifier `unknown`


error: multiple `verbatim` modifiers in a single `modifiers` argument
   |
   |
LL | #[link(name = "...", modifiers = "+verbatim,+verbatim")] //~ ERROR multiple `verbatim` modifiers

error: aborting due to 24 previous errors

For more information about this error, try `rustc --explain E0459`.
---

26    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
27    = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
28 
- error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...")]`
+ error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
31    |
32 LL | #[link]

35    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
35    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
36    = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
37 
- error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...")]`
+ error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
40    |
41 LL | #[link = ""]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/malformed-regressions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args malformed/malformed-regressions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/malformed/malformed-regressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: attribute must be of the form `#[doc(hidden|inline|...)]` or `#[doc = "string"]`
   |
   |
LL | #[doc] //~ ERROR attribute must be of the form
   |
   = note: `#[deny(ill_formed_attribute_input)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>

error: attribute must be of the form `#[ignore]` or `#[ignore = "reason"]`
   |
   |
LL | #[ignore()] //~ ERROR attribute must be of the form
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>


error: attribute must be of the form `#[inline]` or `#[inline(always|never)]`
   |
   |
LL | #[inline = ""] //~ ERROR attribute must be of the form
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>


error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
   |
   |
LL | #[link] //~ ERROR attribute must be of the form
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>


error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ wasm_import_module = "...", /*opt*/ import_name_type = "decorated|noprefix|undecorated")]`
   |
   |
LL | #[link = ""] //~ ERROR attribute must be of the form
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>

