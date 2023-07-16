plain
.................................................................................................... 9300/11536
.................................................................................................... 9400/11536
......................................................................i......i...................... 9500/11536
.................................................................................................... 9600/11536
................iiiiiii..i.iiiiii................................................................... 9700/11536
.................................................................................................... 9900/11536
.................................................................................................... 10000/11536
.................................................................................................... 10100/11536
.................................................................................................... 10200/11536
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.061 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.21s

 finished in 2.274 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1300/2962
.................................................................................................... 1400/2962
.................................................................................................... 1500/2962
.................................................................................................... 1600/2962
...............................................................F...............F..............F..... 1700/2962
........F............F............F................................................................. 1800/2962
.................................................................................................... 2000/2962
.................................................................................................... 2100/2962
.................................................................................................... 2200/2962
.................................................................................................... 2300/2962
---
..............i.....................i.....................i.....................i................... 2900/2962
..............................................................
failures:

---- src/num/nonzero.rs - num::nonzero::NonZeroI128::wrapping_abs (line 593) stdout ----
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `once`
   |
   |
15 | FIXME: add once Neg is implemented?
   |      -     ^^^^ expected one of 8 possible tokens
   |      |
   |      tried to parse a type due to this type ascription
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI16::wrapping_abs (line 593) stdout ----
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `once`
   |
   |
15 | FIXME: add once Neg is implemented?
   |      -     ^^^^ expected one of 8 possible tokens
   |      |
   |      tried to parse a type due to this type ascription
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI32::wrapping_abs (line 593) stdout ----
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `once`
   |
   |
15 | FIXME: add once Neg is implemented?
   |      -     ^^^^ expected one of 8 possible tokens
   |      |
   |      tried to parse a type due to this type ascription
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI64::wrapping_abs (line 593) stdout ----
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `once`
   |
   |
15 | FIXME: add once Neg is implemented?
   |      -     ^^^^ expected one of 8 possible tokens
   |      |
   |      tried to parse a type due to this type ascription
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI8::wrapping_abs (line 593) stdout ----
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `once`
   |
   |
15 | FIXME: add once Neg is implemented?
   |      -     ^^^^ expected one of 8 possible tokens
   |      |
   |      tried to parse a type due to this type ascription
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroIsize::wrapping_abs (line 593) stdout ----
error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `once`
   |
   |
15 | FIXME: add once Neg is implemented?
   |      -     ^^^^ expected one of 8 possible tokens
   |      |
   |      tried to parse a type due to this type ascription
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to previous error

Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:56
