plain
.iii.................................................................................... 12936/12956
....................
failures:

---- [ui] src/test/ui/proc-macro/pretty-print-hack-hide.rs stdout ----


2 PRINT-DERIVE INPUT (DEBUG): TokenStream [
4         ident: "enum",
4         ident: "enum",
-         span: #0 bytes(13955561..13955565),
+         span: #0 bytes(13420865..13420869),
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
7     Ident {
7     Ident {
8         ident: "ProceduralMasqueradeDummyType",

-         span: #0 bytes(13955566..13955595),
+         span: #0 bytes(13420870..13420899),
11     Group {
12         delimiter: Brace,


13         stream: TokenStream [
14             Ident {
15                 ident: "Input",
-                 span: #0 bytes(13955790..13955795),
+                 span: #0 bytes(13421094..13421099),
18         ],
18         ],
-         span: #0 bytes(13955596..13955797),
+         span: #0 bytes(13420900..13421101),
21 ]
22 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-hide/pretty-print-hack-hide.stdout
To only update this specific test, also pass `--test-args proc-macro/pretty-print-hack-hide.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/pretty-print-hack-hide.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-hide" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-hide/auxiliary"
--- stdout -------------------------------
PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "enum",
        ident: "enum",
        span: #0 bytes(13420865..13420869),
    Ident {
    Ident {
        ident: "ProceduralMasqueradeDummyType",
        span: #0 bytes(13420870..13420899),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "Input",
                span: #0 bytes(13421094..13421099),
        ],
        ],
        span: #0 bytes(13420900..13421101),
]
------------------------------------------
stderr: none



---- [ui] src/test/ui/proc-macro/pretty-print-hack-show.rs stdout ----


3 PRINT-DERIVE INPUT (DEBUG): TokenStream [
5         ident: "enum",
5         ident: "enum",
-         span: #0 bytes(13955547..13955551),
+         span: #0 bytes(13420851..13420855),
8     Ident {
8     Ident {
9         ident: "ProceduralMasqueradeDummyType",

-         span: #0 bytes(13955552..13955581),
+         span: #0 bytes(13420856..13420885),
12     Group {
13         delimiter: Brace,


14         stream: TokenStream [
15             Ident {
16                 ident: "Input",
-                 span: #0 bytes(13955776..13955781),
+                 span: #0 bytes(13421080..13421085),
19         ],
19         ],
-         span: #0 bytes(13955582..13955783),
+         span: #0 bytes(13420886..13421087),
22 ]
23 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-show/pretty-print-hack-show.stdout
To only update this specific test, also pass `--test-args proc-macro/pretty-print-hack-show.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/pretty-print-hack-show.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-show" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/pretty-print-hack-show/auxiliary"
--- stdout -------------------------------
PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input, }
PRINT-DERIVE RE-COLLECTED (DISPLAY): enum ProceduralMasqueradeDummyType { Input }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "enum",
        ident: "enum",
        span: #0 bytes(13420851..13420855),
    Ident {
    Ident {
        ident: "ProceduralMasqueradeDummyType",
        span: #0 bytes(13420856..13420885),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "Input",
                span: #0 bytes(13421080..13421085),
        ],
        ],
        span: #0 bytes(13420886..13421087),
]
------------------------------------------
--- stderr -------------------------------
error: using an old version of `rental`
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[deny(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
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
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
Future breakage diagnostic:
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[deny(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
Future breakage diagnostic:
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[deny(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
Future breakage diagnostic:
error: using an old version of `rental`
  --> /checkout/src/test/ui/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[deny(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust;please update to `rental` v0.5.6, or switch to one of the `rental` alternatives



failures:
