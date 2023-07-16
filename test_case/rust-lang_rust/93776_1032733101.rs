plain
....................................F............................................................... 8600/12631
...........................................i..................................................F..... 8700/12631
.................................................................................................... 8800/12631
.......................i............................................................................ 8900/12631
.......F........................................................................F.....F............. 9000/12631
.................................................................................................... 9200/12631
.................................................................................................... 9300/12631
.......................................................iiii.iiiii................................... 9400/12631
...............................ii...............i................................................... 9500/12631
---

---- [ui] ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs stdout ----
diff of stderr:

10 LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
12    |
12    |
- help: Unicode character '−' (Minus Sign) looks like '-' (Minus/Hyphen), but it is not
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ help: unexpected token '−': this Unicode character '−' (Minus Sign) might be confused with '-' (Minus/Hyphen)
14    |
15 LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³⋅kg⁻¹⋅s⁻²


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt/issue-49746-unicode-confusable-in-float-literal-expt.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt/issue-49746-unicode-confusable-in-float-literal-expt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected at least one digit in exponent
  --> /checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs:1:47
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²

error: unknown start of token: \u{2212}
  --> /checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs:1:53
   |
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
   |
   |
help: unexpected token '−': this Unicode character '−' (Minus Sign) might be confused with '-' (Minus/Hyphen)
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³⋅kg⁻¹⋅s⁻²


error[E0277]: cannot subtract `{integer}` from `{float}`
  --> /checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs:1:53
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
   |                                                     ^ no implementation for `{float} - {integer}`
   |
   = help: the trait `Sub<{integer}>` is not implemented for `{float}`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/parser/emoji-identifiers.rs stdout ----
diff of stderr:

4 LL |     let _ = i_like_to_😄_a_lot() ➖ 4;
6    |
6    |
- help: Unicode character '➖' (Heavy Minus Sign) looks like '-' (Minus/Hyphen), but it is not
+ help: unexpected token '➖': this Unicode character '➖' (Heavy Minus Sign) might be confused with '-' (Minus/Hyphen)
8    |
9 LL |     let _ = i_like_to_😄_a_lot() - 4;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/emoji-identifiers.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/emoji-identifiers.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/emoji-identifiers.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/emoji-identifiers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{2796}
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:13:33
   |
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |
   |
help: unexpected token '➖': this Unicode character '➖' (Heavy Minus Sign) might be confused with '-' (Minus/Hyphen)
   |
LL |     let _ = i_like_to_😄_a_lot() - 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope


error[E0425]: cannot find function `i_like_to_😄_a_lot` in this scope
   |
   |
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji
   | ----------------------------- similarly named function `i_like_to_😅_a_lot` defined here
...
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |             ^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `i_like_to_😅_a_lot`
error: Ferris cannot be used as an identifier
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:17:9
   |
   |
LL |     let 🦀 = 1;//~ ERROR Ferris cannot be used as an identifier
   |         ^^ help: try using their name instead: `ferris`
LL |     dbg!(🦀);


error: identifiers cannot contain emoji: `ABig👩👩👧👧Family`
   |
   |
LL | struct ABig👩👩👧👧Family; //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `👀`
   |
   |
LL | struct 👀; //~ ERROR identifiers cannot contain emoji
LL | impl 👀 {
   |      ^^
   |      ^^
LL |     fn full_of_✨() -> 👀 { //~ ERROR identifiers cannot contain emoji
LL |         👀
   |         ^^
...
...
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji
   |                            ^^
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`


error: identifiers cannot contain emoji: `full_of_✨`
   |
   |
LL |     fn full_of_✨() -> 👀 { //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `i_like_to_😅_a_lot`
   |
   |
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `full_of✨`
   |
   |
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`


error: identifiers cannot contain emoji: `i_like_to_😄_a_lot`
   |
   |
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope


error[E0599]: no function or associated item named `full_of✨` found for struct `👀` in the current scope
   |
   |
LL | struct 👀; //~ ERROR identifiers cannot contain emoji
   | ---------- function or associated item `full_of✨` not found for this
...
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`
   |         |
   |         function or associated item not found in `👀`
   |         function or associated item not found in `👀`
   |         help: there is an associated function with a similar name: `full_of_✨`
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
---

20 LL | enumem˂˂
21    |       ^
22    |
- help: Unicode character '˂' (Modifier Letter Left Arrowhead) looks like '<' (Less-Than Sign), but it is not
+ help: unexpected token '˂': this Unicode character '˂' (Modifier Letter Left Arrowhead) might be confused with '<' (Less-Than Sign)
25 LL | enumem<˂
26    |       ~

31 LL | enumem˂˂
31 LL | enumem˂˂
32    |        ^
33    |
- help: Unicode character '˂' (Modifier Letter Left Arrowhead) looks like '<' (Less-Than Sign), but it is not
+ help: unexpected token '˂': this Unicode character '˂' (Modifier Letter Left Arrowhead) might be confused with '<' (Less-Than Sign)
36 LL | enumem˂<
37    |        ~



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-68730/issue-68730.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issues/issue-68730.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-68730.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-68730" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-68730/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{0}
  --> /checkout/src/test/ui/parser/issues/issue-68730.rs:5:5
   |
LL | enumem˂˂
   |     ^
   |
   = help: source files must contain UTF-8 encoded text, unexpected null bytes might occur when a different encoding is used
error: unknown start of token: \u{0}
  --> /checkout/src/test/ui/parser/issues/issue-68730.rs:5:8
   |
LL | enumem˂˂
LL | enumem˂˂
   |       ^
   |
   = help: source files must contain UTF-8 encoded text, unexpected null bytes might occur when a different encoding is used

error: unknown start of token: \u{2c2}
   |
LL | enumem˂˂
   |       ^
   |
   |
help: unexpected token '˂': this Unicode character '˂' (Modifier Letter Left Arrowhead) might be confused with '<' (Less-Than Sign)
LL | enumem<˂
   |       ~


error: unknown start of token: \u{2c2}
   |
LL | enumem˂˂
   |        ^
   |
   |
help: unexpected token '˂': this Unicode character '˂' (Modifier Letter Left Arrowhead) might be confused with '<' (Less-Than Sign)
LL | enumem˂<
   |        ~


error: expected one of `#`, `>`, `const`, identifier, or lifetime, found `<`
   |
LL | enumem˂˂
LL | enumem˂˂
   |        ^ expected one of `#`, `>`, `const`, identifier, or lifetime
error: aborting due to 5 previous errors


------------------------------------------
---

4 LL |     println!("");
5    |                 ^
6    |
- help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
+ help: unexpected token ';': this Unicode character ';' (Greek Question Mark) might be confused with ';' (Semicolon)
9 LL |     println!("");
10    |                 ~



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-homoglyph/recover-from-homoglyph.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/recover-from-homoglyph.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-from-homoglyph.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-homoglyph" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-homoglyph/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{37e}
   |
   |
LL |     println!(""); //~ ERROR unknown start of token: \u{37e}
   |
   |
help: unexpected token ';': this Unicode character ';' (Greek Question Mark) might be confused with ';' (Semicolon)
   |
LL |     println!(""); //~ ERROR unknown start of token: \u{37e}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-from-homoglyph.rs:3:20
   |
   |
LL |     let x: usize = (); //~ ERROR mismatched types
   |            -----   ^^ expected `usize`, found `()`
   |            expected due to this

error: aborting due to 2 previous errors

---

4 LL |     let y = 0;
5    |              ^
6    |
- help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
+ help: unexpected token ';': this Unicode character ';' (Greek Question Mark) might be confused with ';' (Semicolon)
9 LL |     let y = 0;
10    |              ~



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars/unicode-chars.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/unicode-chars.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{37e}
   |
LL |     let y = 0;
   |              ^
   |
   |
help: unexpected token ';': this Unicode character ';' (Greek Question Mark) might be confused with ';' (Semicolon)
LL |     let y = 0;
   |              ~

error: aborting due to previous error
---

---- [ui] ui/parser/unicode-quote-chars.rs stdout ----
diff of stderr:

4 LL |     println!(“hello world”);
6    |
6    |
- help: Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
+ help: unexpected token '“': this Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) might be confused with '"' (Quotation Mark)
9 LL |     println!("hello world");
10    |              ~~~~~~~~~~~~~


15 LL |     println!(“hello world”);
17    |
17    |
- help: Unicode character '”' (Right Double Quotation Mark) looks like '"' (Quotation Mark), but it is not
+ help: unexpected token '”': this Unicode character '”' (Right Double Quotation Mark) might be confused with '"' (Quotation Mark)
19    |
20 LL |     println!(“hello world");


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/unicode-quote-chars.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/unicode-quote-chars.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/unicode-quote-chars.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-quote-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{201c}
   |
   |
LL |     println!(“hello world”);
   |
   |
help: unexpected token '“': this Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) might be confused with '"' (Quotation Mark)
LL |     println!("hello world");
   |              ~~~~~~~~~~~~~


error: unknown start of token: \u{201d}
   |
   |
LL |     println!(“hello world”);
   |
   |
help: unexpected token '”': this Unicode character '”' (Right Double Quotation Mark) might be confused with '"' (Quotation Mark)
   |
LL |     println!(“hello world");


error: expected `,`, found `world`
   |
   |
LL |     println!(“hello world”);
   |                     ^^^^^ expected `,`
error: aborting due to 3 previous errors


------------------------------------------
---

4 LL | fn x˂-
5    |     ^
6    |
- help: Unicode character '˂' (Modifier Letter Left Arrowhead) looks like '<' (Less-Than Sign), but it is not
+ help: unexpected token '˂': this Unicode character '˂' (Modifier Letter Left Arrowhead) might be confused with '<' (Less-Than Sign)
9 LL | fn x<-
10    |     ~



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-81800/issue-81800.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-81800.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-81800.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-81800" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-81800/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{2c2}
   |
   |
LL | fn x˂- //~ ERROR: unknown start of token
   |
   |
help: unexpected token '˂': this Unicode character '˂' (Modifier Letter Left Arrowhead) might be confused with '<' (Less-Than Sign)
   |
LL | fn x<- //~ ERROR: unknown start of token


error: expected one of `#`, `>`, `const`, identifier, or lifetime, found `-`
   |
   |
LL | fn x˂- //~ ERROR: unknown start of token
   |      ^ expected one of `#`, `>`, `const`, identifier, or lifetime
error: aborting due to 2 previous errors


------------------------------------------
