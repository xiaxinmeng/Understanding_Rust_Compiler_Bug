plain
......................................iii........................................................... 12600/12643
...........................................
failures:

---- [ui] ui/proc-macro/pretty-print-hack-hide.rs stdout ----


2 PRINT-DERIVE INPUT (DEBUG): TokenStream [
4         ident: "enum",
4         ident: "enum",
-         span: #0 bytes(13346386..13346390),
+         span: #0 bytes(13346595..13346599),
7     Ident {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
8         ident: "ProceduralMasqueradeDummyType",

-         span: #0 bytes(13346391..13346420),
+         span: #0 bytes(13346600..13346629),
11     Group {
12         delimiter: Brace,


13         stream: TokenStream [
14             Ident {
15                 ident: "Input",
-                 span: #0 bytes(13346615..13346620),
+                 span: #0 bytes(13346824..13346829),
18         ],
18         ],
-         span: #0 bytes(13346421..13346622),
+         span: #0 bytes(13346630..13346831),
21 ]
22 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-hide/pretty-print-hack-hide.stdout
To only update this specific test, also pass `--test-args proc-macro/pretty-print-hack-hide.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/pretty-print-hack-hide.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-hide" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-hide/auxiliary"
------------------------------------------
------------------------------------------
PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "enum",
        ident: "enum",
        span: #0 bytes(13346595..13346599),
    Ident {
    Ident {
        ident: "ProceduralMasqueradeDummyType",
        span: #0 bytes(13346600..13346629),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "Input",
                span: #0 bytes(13346824..13346829),
        ],
        ],
        span: #0 bytes(13346630..13346831),
]

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/proc-macro/pretty-print-hack-show.rs stdout ----


3 PRINT-DERIVE INPUT (DEBUG): TokenStream [
5         ident: "enum",
5         ident: "enum",
-         span: #0 bytes(13346372..13346376),
+         span: #0 bytes(13346581..13346585),
8     Ident {
8     Ident {
9         ident: "ProceduralMasqueradeDummyType",

-         span: #0 bytes(13346377..13346406),
+         span: #0 bytes(13346586..13346615),
12     Group {
13         delimiter: Brace,


14         stream: TokenStream [
15             Ident {
16                 ident: "Input",
-                 span: #0 bytes(13346601..13346606),
+                 span: #0 bytes(13346810..13346815),
19         ],
19         ],
-         span: #0 bytes(13346407..13346608),
+         span: #0 bytes(13346616..13346817),
22 ]
23 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-show/pretty-print-hack-show.stdout
To only update this specific test, also pass `--test-args proc-macro/pretty-print-hack-show.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/pretty-print-hack-show.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-show" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-show/auxiliary"
------------------------------------------
------------------------------------------
PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input, }
PRINT-DERIVE RE-COLLECTED (DISPLAY): enum ProceduralMasqueradeDummyType { Input }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "enum",
        ident: "enum",
        span: #0 bytes(13346581..13346585),
    Ident {
    Ident {
        ident: "ProceduralMasqueradeDummyType",
        span: #0 bytes(13346586..13346615),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "Input",
                span: #0 bytes(13346810..13346815),
        ],
        ],
        span: #0 bytes(13346616..13346817),
]

------------------------------------------
stderr:
stderr:
------------------------------------------
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[deny(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6
error: aborting due to 4 previous errors


Future incompatibility report: Future breakage diagnostic:
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[deny(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6
Future breakage diagnostic:
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6
Future breakage diagnostic:
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6
Future breakage diagnostic:
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6

------------------------------------------


