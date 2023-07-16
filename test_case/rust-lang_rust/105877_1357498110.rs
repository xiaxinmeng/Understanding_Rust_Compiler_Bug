plain
---- [ui] src/test/ui/inference/issue-72616.rs stdout ----
diff of stderr:

8    |
9    = note: multiple `impl`s satisfying `String: PartialEq<_>` found in the following crates: `alloc`, `std`:
10            - impl PartialEq for String;
-     - impl PartialEq<Path> for String;
-     - impl PartialEq<PathBuf> for String;
+            - impl PartialEq<Path> for String;
+            - impl PartialEq<PathBuf> for String;
13            - impl<'a, 'b> PartialEq<&'a str> for String;
14            - impl<'a, 'b> PartialEq<Cow<'a, str>> for String;
15            - impl<'a, 'b> PartialEq<str> for String;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/issue-72616.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-72616.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-72616.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/issue-72616.rs:22:37
   |
   |
LL |         if String::from("a") == "a".try_into().unwrap() {}
   |                              |
   |                              type must be known at this point
   |
   |
   = note: multiple `impl`s satisfying `String: PartialEq<_>` found in the following crates: `alloc`, `std`:
           - impl PartialEq for String;
           - impl PartialEq<Path> for String;
           - impl PartialEq<PathBuf> for String;
           - impl<'a, 'b> PartialEq<&'a str> for String;
           - impl<'a, 'b> PartialEq<Cow<'a, str>> for String;
           - impl<'a, 'b> PartialEq<str> for String;
   |
   |
LL |         if String::from("a") == <&str as TryInto<T>>::try_into("a").unwrap() {}
   |                                 +++++++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72616.rs:22:37
   |
   |
LL |         if String::from("a") == "a".try_into().unwrap() {}
   |
   |
   = note: multiple `impl`s satisfying `_: TryFrom<&str>` found in the following crates: `core`, `std`:
           - impl<> TryFrom<&str> for std::sys_common::net::LookupHost;
           - impl<T, U> TryFrom<U> for T
             where U: Into<T>;
   = note: required for `&str` to implement `TryInto<_>`
   |
   |
LL |         if String::from("a") == <&str as TryInto<T>>::try_into("a").unwrap() {}
   |                                 +++++++++++++++++++++++++++++++   ~
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lexer/lex-bad-char-literals-6.rs stdout ----
diff of stderr:

39    |
40    = help: the trait `PartialEq<char>` is not implemented for `&str`
41    = help: the following other types implement trait `PartialEq<Rhs>`:
-       <&'a str as PartialEq<OsString>>
+              <&'a str as PartialEq<OsString>>
43              <&'a str as PartialEq<String>>
44              <&'b str as PartialEq<Cow<'a, str>>>
45              <str as PartialEq<Cow<'a, str>>>

47              <str as PartialEq<OsString>>
48              <str as PartialEq<Path>>
49              <str as PartialEq<PathBuf>>
50 
51 error[E0308]: mismatched types
52   --> $DIR/lex-bad-char-literals-6.rs:15:20


72              <str as PartialEq<OsString>>
73              <str as PartialEq<Path>>
74              <str as PartialEq<PathBuf>>
75 
76 error: aborting due to 6 previous errors
77 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6/lex-bad-char-literals-6.stderr
To only update this specific test, also pass `--test-args lexer/lex-bad-char-literals-6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6/auxiliary"
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
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs:4:19
   |
   |
LL |     let y: char = 'cd';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let y: char = "cd";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs:6:13
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
             <&'b str as PartialEq<Cow<'a, str>>>
             <str as PartialEq<Cow<'a, str>>>
             <str as PartialEq<OsStr>>
             <str as PartialEq<OsString>>
             <str as PartialEq<Path>>
             <str as PartialEq<PathBuf>>

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
             <&'b str as PartialEq<Cow<'a, str>>>
             <str as PartialEq<Cow<'a, str>>>
             <str as PartialEq<OsStr>>
             <str as PartialEq<OsString>>
             <str as PartialEq<Path>>
             <str as PartialEq<PathBuf>>

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0277, E0308.
