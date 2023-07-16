plain

---- [ui] src/test/ui/associated-types/issue-85103.rs stdout ----
diff of stderr:

- error: layout error: NormalizationFailure(<[E] as std::borrow::ToOwned>::Owned, Type(<[E] as std::borrow::ToOwned>::Owned))
+ error: layout error: NormalizationFailure(std::borrow::Cow<'a, [E]>, Type(std::borrow::Cow<[E]>))
3    |
3    |
4 LL | type Edges<'a, E> = Cow<'a, [E]>;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-85103/issue-85103.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-85103.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-85103.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-85103" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-85103/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error: layout error: NormalizationFailure(std::borrow::Cow<'a, [E]>, Type(std::borrow::Cow<[E]>))
   |
   |
LL | type Edges<'a, E> = Cow<'a, [E]>;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-22629.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22629.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22629/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22629/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-22629.rs:13:17
   |
   |
LL |     assert_send(Cow::Borrowed("foo"));
   |                 ^^^^^^^^^^^^^ cannot infer type for type parameter `O` declared on the enum `Cow`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lexer/lex-bad-char-literals-6.rs stdout ----
diff of stderr:

41    = help: the following other types implement trait `PartialEq<Rhs>`:
42              <&'a str as PartialEq<OsString>>
43              <&'a str as PartialEq<String>>
-              <&'b str as PartialEq<Cow<'a, str>>>
+              <&'b str as PartialEq<Cow<'a, str, String>>>
45              <String as PartialEq<&'a str>>
-              <String as PartialEq<Cow<'a, str>>>
+              <String as PartialEq<Cow<'a, str, String>>>
47              <String as PartialEq<str>>
48              <String as PartialEq>
-              <str as PartialEq<Cow<'a, str>>>
+              <str as PartialEq<Cow<'a, str, String>>>
51 
52 error[E0308]: mismatched types


67    = help: the following other types implement trait `PartialEq<Rhs>`:
68              <&'a str as PartialEq<OsString>>
69              <&'a str as PartialEq<String>>
-              <&'b str as PartialEq<Cow<'a, str>>>
+              <&'b str as PartialEq<Cow<'a, str, String>>>
71              <String as PartialEq<&'a str>>
-              <String as PartialEq<Cow<'a, str>>>
+              <String as PartialEq<Cow<'a, str, String>>>
73              <String as PartialEq<str>>
74              <String as PartialEq>
-              <str as PartialEq<Cow<'a, str>>>
+              <str as PartialEq<Cow<'a, str, String>>>
77 
78 error: aborting due to 6 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6/lex-bad-char-literals-6.stderr
To only update this specific test, also pass `--test-args lexer/lex-bad-char-literals-6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6/auxiliary"
stdout: none
--- stderr -------------------------------
error: character literal may only contain one codepoint
   |
   |
LL |     let x: &str = 'ab';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let x: &str = "ab";


error: character literal may only contain one codepoint
   |
   |
LL |     let y: char = 'cd';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let y: char = "cd";


error: character literal may only contain one codepoint
   |
   |
LL |     let z = 'ef';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let z = "ef";


error[E0277]: can't compare `&str` with `char`
   |
   |
LL |     if x == y {}
   |          ^^ no implementation for `&str == char`
   |
   = help: the trait `PartialEq<char>` is not implemented for `&str`
   = help: the following other types implement trait `PartialEq<Rhs>`:
             <&'a str as PartialEq<OsString>>
             <&'a str as PartialEq<String>>
             <&'b str as PartialEq<Cow<'a, str, String>>>
             <String as PartialEq<&'a str>>
             <String as PartialEq<Cow<'a, str, String>>>
             <String as PartialEq<str>>
             <String as PartialEq>
             <str as PartialEq<Cow<'a, str, String>>>

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs:15:20
   |
   |
LL |     let a: usize = "";
   |            -----   ^^ expected `usize`, found `&str`
   |            expected due to this


error[E0277]: can't compare `&str` with `char`
   |
   |
LL |     if x == z {}
   |          ^^ no implementation for `&str == char`
   |
   = help: the trait `PartialEq<char>` is not implemented for `&str`
   = help: the following other types implement trait `PartialEq<Rhs>`:
             <&'a str as PartialEq<OsString>>
             <&'a str as PartialEq<String>>
             <&'b str as PartialEq<Cow<'a, str, String>>>
             <String as PartialEq<&'a str>>
             <String as PartialEq<Cow<'a, str, String>>>
             <String as PartialEq<str>>
             <String as PartialEq>
             <str as PartialEq<Cow<'a, str, String>>>

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0277, E0308.
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/suggestions/into-str.rs stdout ----
diff of stderr:

12              <String as From<&mut str>>
13              <String as From<&str>>
14              <String as From<Box<str>>>
-              <String as From<Cow<'a, str>>>
+              <String as From<Cow<'a, str, String>>>
16              <String as From<char>>
17    = note: required because of the requirements on the impl of `Into<&str>` for `String`
18 note: required by a bound in `foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/into-str.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/into-str.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/into-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&str: From<String>` is not satisfied
   |
LL |     foo(String::new());
LL |     foo(String::new());
   |     --- ^^^^^^^^^^^^^ the trait `From<String>` is not implemented for `&str`
   |     required by a bound introduced by this call
   |
   |
   = note: to coerce a `String` into a `&str`, use `&*` as a prefix
   = help: the following other types implement trait `From<T>`:
             <String as From<&String>>
             <String as From<&mut str>>
             <String as From<&str>>
             <String as From<Box<str>>>
             <String as From<Cow<'a, str, String>>>
             <String as From<char>>
   = note: required because of the requirements on the impl of `Into<&str>` for `String`
note: required by a bound in `foo`
   |
   |
LL | fn foo<'a, T>(_t: T) where T: Into<&'a str> {}
   |                               ^^^^^^^^^^^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/typeck/issue-90101.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `PathBuf: From<Cow<'_, str>>` is not satisfied
+ error[E0277]: the trait bound `PathBuf: From<Cow<'_, str, String>>` is not satisfied
3    |
3    |
4 LL |     func(Path::new("hello").to_path_buf().to_string_lossy(), "world")

-    |     ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<Cow<'_, str>>` is not implemented for `PathBuf`
+    |     ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<Cow<'_, str, String>>` is not implemented for `PathBuf`
7    |     required by a bound introduced by this call
8    |


9    = help: the following other types implement trait `From<T>`:
10              <PathBuf as From<&T>>
11              <PathBuf as From<Box<Path>>>
-              <PathBuf as From<Cow<'a, Path>>>
+              <PathBuf as From<Cow<'a, Path, PathBuf>>>
13              <PathBuf as From<OsString>>
14              <PathBuf as From<String>>
-    = note: required because of the requirements on the impl of `Into<PathBuf>` for `Cow<'_, str>`
+    = note: required because of the requirements on the impl of `Into<PathBuf>` for `Cow<'_, str, String>`
16 note: required by a bound in `func`
18    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-90101/issue-90101.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-90101.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-90101.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-90101" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-90101/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `PathBuf: From<Cow<'_, str, String>>` is not satisfied
   |
   |
LL |     func(Path::new("hello").to_path_buf().to_string_lossy(), "world")
   |     ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<Cow<'_, str, String>>` is not implemented for `PathBuf`
   |     required by a bound introduced by this call
   |
   |
   = help: the following other types implement trait `From<T>`:
             <PathBuf as From<&T>>
             <PathBuf as From<Box<Path>>>
             <PathBuf as From<Cow<'a, Path, PathBuf>>>
             <PathBuf as From<OsString>>
             <PathBuf as From<String>>
   = note: required because of the requirements on the impl of `Into<PathBuf>` for `Cow<'_, str, String>`
note: required by a bound in `func`
   |
   |
LL | fn func(path: impl Into<PathBuf>, code: impl Into<String>) {}
   |                    ^^^^^^^^^^^^^ required by this bound in `func`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
