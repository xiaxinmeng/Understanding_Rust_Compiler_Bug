plain
2020-03-02T13:33:43.3710377Z ========================== Starting Command Output ===========================
2020-03-02T13:33:43.3713760Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/41d663c1-e778-4803-bf25-e00752fe73df.sh
2020-03-02T13:33:43.3714053Z 
2020-03-02T13:33:43.3727791Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T13:33:43.3752140Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69636/merge to s
2020-03-02T13:33:43.3759289Z Task         : Get sources
2020-03-02T13:33:43.3759618Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T13:33:43.3759966Z Version      : 1.0.0
2020-03-02T13:33:43.3760181Z Author       : Microsoft
---
2020-03-02T13:33:44.3656596Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T13:33:44.3662575Z ##[command]git config gc.auto 0
2020-03-02T13:33:44.3665814Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T13:33:44.3669462Z ##[command]git config --get-all http.proxy
2020-03-02T13:33:44.3679640Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69636/merge:refs/remotes/pull/69636/merge
---
2020-03-02T15:36:09.5323912Z  finished in 66.423
2020-03-02T15:36:09.5333225Z Testing rustbook src/doc/nomicon
2020-03-02T15:36:16.9073053Z Error: Rustdoc returned an error: 
2020-03-02T15:36:16.9074067Z running 6 tests
2020-03-02T15:36:16.9076790Z test /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 128) ... FAILED
2020-03-02T15:36:16.9077751Z test /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 28) ... FAILED
2020-03-02T15:36:16.9078877Z test /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 110) ... ok
2020-03-02T15:36:16.9079676Z test /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 6) ... ignored
2020-03-02T15:36:16.9080479Z test /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 55) ... FAILED
2020-03-02T15:36:16.9081257Z test /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 96) ... ok
2020-03-02T15:36:16.9081966Z failures:
2020-03-02T15:36:16.9082185Z 
2020-03-02T15:36:16.9082185Z 
2020-03-02T15:36:16.9082689Z ---- /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 128) stdout ----
2020-03-02T15:36:16.9083192Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-02T15:36:16.9083821Z  --> /tmp/mdbook-YDrEtw/destructors.md:131:18
2020-03-02T15:36:16.9084443Z 4 | use std::alloc::{Alloc, GlobalAlloc, Global, Layout};
2020-03-02T15:36:16.9084700Z   |                  ^^^^^
2020-03-02T15:36:16.9085028Z   |                  |
2020-03-02T15:36:16.9085262Z   |                  no `Alloc` in `alloc`
2020-03-02T15:36:16.9085262Z   |                  no `Alloc` in `alloc`
2020-03-02T15:36:16.9085586Z   |                  help: a similar name exists in the module: `alloc`
2020-03-02T15:36:16.9085824Z 
2020-03-02T15:36:16.9086140Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T15:36:16.9086758Z   --> /tmp/mdbook-YDrEtw/destructors.md:142:20
2020-03-02T15:36:16.9087203Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-02T15:36:16.9087768Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T15:36:16.9092120Z    |
2020-03-02T15:36:16.9092638Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T15:36:16.9092638Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T15:36:16.9093105Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-02T15:36:16.9093362Z    |
2020-03-02T15:36:16.9093524Z 4  | use std::alloc::AllocRef;
2020-03-02T15:36:16.9093683Z    |
2020-03-02T15:36:16.9093775Z 
2020-03-02T15:36:16.9094071Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T15:36:16.9094730Z   --> /tmp/mdbook-YDrEtw/destructors.md:157:20
2020-03-02T15:36:16.9095165Z 30 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-02T15:36:16.9095524Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T15:36:16.9095765Z    |
2020-03-02T15:36:16.9096008Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-02T15:36:16.9097355Z 
2020-03-02T15:36:16.9097553Z Some errors have detailed explanations: E0432, E0599.
2020-03-02T15:36:16.9098046Z For more information about an error, try `rustc --explain E0432`.
2020-03-02T15:36:16.9098424Z Couldn't compile the test.
2020-03-02T15:36:16.9098883Z ---- /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 28) stdout ----
2020-03-02T15:36:16.9099198Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-02T15:36:16.9099708Z  --> /tmp/mdbook-YDrEtw/destructors.md:31:18
2020-03-02T15:36:16.9100691Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-03-02T15:36:16.9100961Z   |                  ^^^^^
2020-03-02T15:36:16.9101179Z   |                  |
2020-03-02T15:36:16.9101406Z   |                  no `Alloc` in `alloc`
2020-03-02T15:36:16.9101406Z   |                  no `Alloc` in `alloc`
2020-03-02T15:36:16.9101820Z   |                  help: a similar name exists in the module: `alloc`
2020-03-02T15:36:16.9102103Z 
2020-03-02T15:36:16.9102426Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T15:36:16.9103074Z   --> /tmp/mdbook-YDrEtw/destructors.md:42:20
2020-03-02T15:36:16.9103864Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>())
2020-03-02T15:36:16.9104219Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T15:36:16.9104476Z    |
2020-03-02T15:36:16.9104695Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-02T15:36:16.9106038Z 
2020-03-02T15:36:16.9106237Z Some errors have detailed explanations: E0432, E0599.
2020-03-02T15:36:16.9106724Z For more information about an error, try `rustc --explain E0432`.
2020-03-02T15:36:16.9107125Z Couldn't compile the test.
2020-03-02T15:36:16.9107558Z ---- /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 55) stdout ----
2020-03-02T15:36:16.9107872Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-02T15:36:16.9108320Z  --> /tmp/mdbook-YDrEtw/destructors.md:58:18
2020-03-02T15:36:16.9108705Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-03-02T15:36:16.9108960Z   |                  ^^^^^
2020-03-02T15:36:16.9109135Z   |                  |
2020-03-02T15:36:16.9109339Z   |                  no `Alloc` in `alloc`
2020-03-02T15:36:16.9109339Z   |                  no `Alloc` in `alloc`
2020-03-02T15:36:16.9109657Z   |                  help: a similar name exists in the module: `alloc`
2020-03-02T15:36:16.9109879Z 
2020-03-02T15:36:16.9110158Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T15:36:16.9110700Z   --> /tmp/mdbook-YDrEtw/destructors.md:69:20
2020-03-02T15:36:16.9111358Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-02T15:36:16.9111881Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T15:36:16.9112178Z    |
2020-03-02T15:36:16.9112428Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T15:36:16.9112428Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T15:36:16.9112824Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-02T15:36:16.9113117Z    |
2020-03-02T15:36:16.9113303Z 4  | use std::alloc::AllocRef;
2020-03-02T15:36:16.9113485Z    |
2020-03-02T15:36:16.9113606Z 
2020-03-02T15:36:16.9113932Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T15:36:16.9114833Z   --> /tmp/mdbook-YDrEtw/destructors.md:82:20
2020-03-02T15:36:16.9115280Z 28 |             Global.dealloc(c.cast::<u8>(), Layout::new::<T>());
2020-03-02T15:36:16.9115650Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T15:36:16.9115908Z    |
2020-03-02T15:36:16.9116126Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-02T15:36:16.9118259Z For more information about an error, try `rustc --explain E0432`.
2020-03-02T15:36:16.9118895Z Couldn't compile the test.
2020-03-02T15:36:16.9119037Z 
2020-03-02T15:36:16.9119162Z failures:
2020-03-02T15:36:16.9119669Z     /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 128)
2020-03-02T15:36:16.9120221Z     /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 28)
2020-03-02T15:36:16.9120723Z     /tmp/mdbook-YDrEtw/destructors.md - Destructors (line 55)
2020-03-02T15:36:16.9121193Z test result: FAILED. 2 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
2020-03-02T15:36:16.9121426Z 
2020-03-02T15:36:16.9121518Z 
2020-03-02T15:36:16.9121608Z 
---
2020-03-02T15:36:16.9141958Z  finished in 7.380
2020-03-02T15:36:16.9142188Z Testing rustbook src/doc/reference
2020-03-02T15:36:35.1732273Z Error: Rustdoc returned an error: 
2020-03-02T15:36:35.1732989Z running 2 tests
2020-03-02T15:36:35.1734550Z test /tmp/mdbook-DwSMls/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) ... FAILED
2020-03-02T15:36:35.1737612Z test /tmp/mdbook-DwSMls/expressions/array-expr.md - Array_and_array_index_expressions::Array_expressions (line 27) ... ok
2020-03-02T15:36:35.1738607Z failures:
2020-03-02T15:36:35.1738857Z 
2020-03-02T15:36:35.1738857Z 
2020-03-02T15:36:35.1739712Z ---- /tmp/mdbook-DwSMls/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) stdout ----
2020-03-02T15:36:35.1740315Z error: this operation will panic at runtime
2020-03-02T15:36:35.1740974Z   --> /tmp/mdbook-DwSMls/expressions/array-expr.md:72:9
2020-03-02T15:36:35.1741716Z 11 | let x = (["a", "b"])[10]; // warning: index out of bounds
2020-03-02T15:36:35.1742240Z    |         ^^^^^^^^^^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-03-02T15:36:35.1742629Z    |
2020-03-02T15:36:35.1743003Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-02T15:36:35.1743003Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-02T15:36:35.1743305Z 
2020-03-02T15:36:35.1743603Z error: this operation will panic at runtime
2020-03-02T15:36:35.1744255Z   --> /tmp/mdbook-DwSMls/expressions/array-expr.md:78:1
2020-03-02T15:36:35.1746133Z 17 | arr[10];                  // warning: index out of bounds
2020-03-02T15:36:35.1747178Z    | ^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-03-02T15:36:35.1747835Z 
2020-03-02T15:36:35.1748191Z error: aborting due to 2 previous errors
2020-03-02T15:36:35.1748191Z error: aborting due to 2 previous errors
2020-03-02T15:36:35.1748504Z 
2020-03-02T15:36:35.1749212Z Couldn't compile the test.
2020-03-02T15:36:35.1749516Z 
2020-03-02T15:36:35.1749761Z failures:
2020-03-02T15:36:35.1750692Z     /tmp/mdbook-DwSMls/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63)
2020-03-02T15:36:35.1751528Z test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-02T15:36:35.1751892Z 
2020-03-02T15:36:35.1752100Z 
2020-03-02T15:36:35.1752375Z 
---
2020-03-02T15:42:16.5774549Z  49 │ - [`StringReader`] from [libsyntax] integrates `rustc_lexer` with `rustc`
2020-03-02T15:42:16.5775158Z     │                         ^ Server responded with 404 Not Found
2020-03-02T15:42:16.5775608Z     │
2020-03-02T15:42:16.5775724Z 
2020-03-02T15:42:16.5779989Z error: The server responded with 404 Not Found for "***/tree/master/src/libsyntax"
2020-03-02T15:42:16.5780728Z     ┌── test-implementation.md:34:1 ───
2020-03-02T15:42:16.5781107Z     │
2020-03-02T15:42:16.5781616Z  34 │ [`libsyntax` crate][libsyntax]. Essentially, it's a fancy macro, that
2020-03-02T15:42:16.5782281Z     │ ^ Server responded with 404 Not Found
---
2020-03-02T15:42:16.5787193Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Ident.html"
2020-03-02T15:42:16.5787552Z 
2020-03-02T15:42:16.5787980Z     ┌── test-implementation.md:69:49 ───
2020-03-02T15:42:16.5788500Z     │
2020-03-02T15:42:16.5789063Z  69 │ not stored as a string, but rather as an opaque [Symbol][Symbol] which is
2020-03-02T15:42:16.5790059Z     │
2020-03-02T15:42:16.5790176Z 
2020-03-02T15:42:16.5790781Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_expand/mbe/macro_parser/fn.parse.html"
2020-03-02T15:42:16.5791145Z 
---
2020-03-02T15:42:16.5810287Z  135 │ [`NodeId`]. This returns a `Option<Node<'tcx>>`, where [`Node`] is an enum
2020-03-02T15:42:16.5811932Z      │ ^ Server responded with 404 Not Found
2020-03-02T15:42:16.5812385Z      │
2020-03-02T15:42:16.5812531Z 
2020-03-02T15:42:16.5813229Z error: The server responded with 404 Not Found for "***/tree/master/src/librustc/infer/higher_ranked/README.md"
2020-03-02T15:42:16.5814033Z     ┌── traits/hrtb.md:35:62 ───
2020-03-02T15:42:16.5814353Z     │
2020-03-02T15:42:16.5815318Z  35 │ to the subtyping for higher-ranked types (which is described [here][hrsubtype]
2020-03-02T15:42:16.5816126Z     │                                                              ^ Server responded with 404 Not Found
---
2020-03-02T15:42:16.5819802Z  57 │ it also looks like Rust source code. Checkout the documentation in the [manual for GDB/Rust].
2020-03-02T15:42:16.5820674Z     │                                                                        ^ https://sourceware.org/gdb/onlinedocs/gdb/Rust.html: timed out
2020-03-02T15:42:16.5821248Z     │
2020-03-02T15:42:16.5821389Z 
2020-03-02T15:42:16.5822212Z error: Unable to retrieve "https://sourceware.org/bugzilla/": https://sourceware.org/bugzilla/: timed out
2020-03-02T15:42:16.5822988Z     ┌── debugging-support-in-rustc.md:92:54 ───
2020-03-02T15:42:16.5823636Z     │
2020-03-02T15:42:16.5823636Z     │
2020-03-02T15:42:16.5824084Z  92 │ * This work is now upstream. Bugs can be reported in [GDB Bugzilla].
2020-03-02T15:42:16.5824726Z     │                                                      ^ https://sourceware.org/bugzilla/: timed out
2020-03-02T15:42:16.5825319Z 
2020-03-02T15:42:16.5825319Z 
2020-03-02T15:42:16.5826544Z error: The server responded with 404 Not Found for "***/tree/master/src/libsyntax"
2020-03-02T15:42:16.5827327Z     ┌── appendix/stupid-stats.md:64:43 ───
2020-03-02T15:42:16.5827662Z     │
2020-03-02T15:42:16.5827662Z     │
2020-03-02T15:42:16.5828398Z  64 │ The code for these first two phases is in [libsyntax](***/tree/master/src/libsyntax).
2020-03-02T15:42:16.5830344Z     │
2020-03-02T15:42:16.5830470Z 
2020-03-02T15:42:16.5830839Z error: Unable to retrieve "https://clang.llvm.org/docs/ThinLTO.html": https://clang.llvm.org/docs/ThinLTO.html: timed out
2020-03-02T15:42:16.5831185Z 
2020-03-02T15:42:16.5831185Z 
2020-03-02T15:42:16.5831620Z     ┌── appendix/glossary.md:43:521 ───
2020-03-02T15:42:16.5832101Z     │
2020-03-02T15:42:16.5834257Z  43 │ LTO                     |  Link-Time Optimizations. A set of optimizations offered by LLVM that occur just before the final binary is linked. These include optimizations like removing functions that are never used in the final program, for example. _ThinLTO_ is a variant of LTO that aims to be a bit more scalable and efficient, but possibly sacrifices some optimizations. You may also read issues in the Rust repo about "FatLTO", which is the loving nickname given to non-Thin LTO. LLVM documentation: [here][lto] and [here][thinlto]
2020-03-02T15:42:16.5839556Z     │
2020-03-02T15:42:16.5839668Z 
2020-03-02T15:42:16.5840795Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Crate.html"
2020-03-02T15:42:16.5841465Z 
---
2020-03-02T15:42:16.5855697Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ptr/struct.P.html"
2020-03-02T15:42:16.5856057Z 
2020-03-02T15:42:16.5856643Z     ┌── appendix/code-index.md:18:122 ───
2020-03-02T15:42:16.5857676Z     │
2020-03-02T15:42:16.5858794Z  18 │ `P` | struct | An owned immutable smart pointer. By contrast, `&T` is not owned, and `Box<T>` is not immutable. | None | [src/syntax/ptr.rs](https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ptr/struct.P.html)
2020-03-02T15:42:16.5862731Z     │
2020-03-02T15:42:16.5862858Z 
2020-03-02T15:42:16.5863726Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/tokenstream/struct.TokenStream.html"
2020-03-02T15:42:16.5864566Z 
2020-03-02T15:42:16.5864566Z 
2020-03-02T15:42:16.5865156Z     ┌── appendix/code-index.md:28:144 ───
2020-03-02T15:42:16.5865562Z     │
2020-03-02T15:42:16.5867466Z  28 │ `syntax::token_stream::TokenStream` | struct | An abstract sequence of tokens, organized into `TokenTree`s | [The parser], [Macro expansion] | [src/libsyntax/tokenstream.rs](https://doc.rust-lang.org/nightly/nightly-rustc/syntax/tokenstream/struct.TokenStream.html)
2020-03-02T15:42:16.5869849Z     │
2020-03-02T15:42:16.5869955Z 
2020-03-02T15:42:16.5871945Z error: Unable to retrieve "http://fitzgeraldnick.com/2018/12/13/rust-raps.html": https://fitzgeraldnick.com/2018/12/13/rust-raps.html: error trying to connect: error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:ssl/statem/statem_clnt.c:1915: (certificate has expired)
2020-03-02T15:42:16.5872757Z 
---
2020-03-02T16:32:10.5646064Z note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
2020-03-02T16:32:10.5646360Z 
2020-03-02T16:32:10.5654225Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-02T16:32:10.5654691Z 
2020-03-02T16:32:10.5661245Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-02T16:32:10.5665967Z note: rustc 1.43.0-nightly (2a4746ca1 2020-03-02) running on x86_64-unknown-linux-gnu
2020-03-02T16:32:10.5666295Z 
2020-03-02T16:32:10.5673655Z note: compiler flags: -Z always-encode-mir -Z mir-emit-retag -Z mir-opt-level=0 -Z force-unstable-if-unmarked -C opt-level=3 -C debug-assertions=on -C debug-assertions=y --crate-type lib
2020-03-02T16:32:10.5674148Z 
---
2020-03-02T16:32:10.5726173Z 
2020-03-02T16:32:10.5790266Z error: could not compile `panic_unwind`.
2020-03-02T16:32:10.5798620Z warning: build failed, waiting for other jobs to finish...
2020-03-02T16:32:11.4725934Z error: build failed
2020-03-02T16:32:11.4967321Z error: `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--release" "--manifest-path" "/tmp/xargo.HtWoCfbMX6Tl/Cargo.toml" "--target" "x86_64-unknown-linux-gnu" "-p" "std"` failed with exit code: Some(101)
2020-03-02T16:32:11.4969348Z stack backtrace:
2020-03-02T16:32:11.4969675Z    0: error_chain::make_backtrace::hd12e2434e1f5b043 (0x563ccaf6b3f3)
2020-03-02T16:32:11.4970138Z    1: <error_chain::State as core::default::Default>::default::hc4d1755cac7e56ce (0x563ccaf6b4d6)
2020-03-02T16:32:11.4972635Z    2: <std::process::Command as xargo::extensions::CommandExt>::run::hafafe89330991267 (0x563ccaf19991)
2020-03-02T16:32:11.4974174Z    3: xargo::sysroot::build::h3c41a175b12e33e8 (0x563ccaf2bf51)
2020-03-02T16:32:11.4974551Z    4: xargo::sysroot::update::hd92ac48f1d13ed0b (0x563ccaf2f045)
2020-03-02T16:32:11.4974932Z    5: xargo::main_inner::hffcbdf4f2dcccc99 (0x563ccaf015cf)
2020-03-02T16:32:11.4975319Z    6: std::rt::lang_start::{{closure}}::hb4446302ef2d4988 (0x563ccaeff0a3)
2020-03-02T16:32:11.4975726Z    7: std::panicking::try::do_call::hc6b2b30017398f7d (0x563ccaf9b6df)
2020-03-02T16:32:11.4976086Z    8: __rust_maybe_catch_panic (0x563ccafa6827)
2020-03-02T16:32:11.4976430Z    9: std::rt::lang_start_internal::h33e4af2a82949067 (0x563ccaf9c426)
2020-03-02T16:32:11.4976746Z   10: main (0x563ccaeff102)
2020-03-02T16:32:11.4977005Z   11: __libc_start_main (0x7fe466cd6830)
2020-03-02T16:32:11.4977251Z   12: _start (0x563ccaefef99)
2020-03-02T16:32:11.4977782Z fatal error: Failed to run xargo
2020-03-02T16:32:11.4977946Z 
2020-03-02T16:32:11.4978042Z 
2020-03-02T16:32:11.4979509Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
---
2020-03-02T16:32:13.6617901Z 
2020-03-02T16:32:13.6618351Z If you do intend to update 'miri', please check the error messages above and
2020-03-02T16:32:13.6618777Z commit another update.
2020-03-02T16:32:13.6618946Z 
2020-03-02T16:32:13.6619435Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2020-03-02T16:32:13.6620017Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2020-03-02T16:32:13.6620310Z proper steps.
2020-03-02T16:32:13.6628067Z Build completed unsuccessfully in 0:00:02
2020-03-02T16:32:13.6642091Z == clock drift check ==
2020-03-02T16:32:13.6659939Z   local time: Mon Mar  2 16:32:13 UTC 2020
2020-03-02T16:32:13.9597596Z   network time: Mon, 02 Mar 2020 16:32:13 GMT
2020-03-02T16:32:13.9597596Z   network time: Mon, 02 Mar 2020 16:32:13 GMT
2020-03-02T16:32:13.9597993Z == end clock drift check ==
2020-03-02T16:32:14.6290215Z 
2020-03-02T16:32:14.6371579Z ##[error]Bash exited with code '1'.
2020-03-02T16:32:14.6386968Z ##[section]Finishing: Run build
2020-03-02T16:32:14.6445005Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69636/merge to s
2020-03-02T16:32:14.6451495Z Task         : Get sources
2020-03-02T16:32:14.6451866Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T16:32:14.6452186Z Version      : 1.0.0
2020-03-02T16:32:14.6452409Z Author       : Microsoft
2020-03-02T16:32:14.6452409Z Author       : Microsoft
2020-03-02T16:32:14.6452789Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-02T16:32:14.6453201Z ==============================================================================
2020-03-02T16:32:15.0021301Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-02T16:32:15.0109712Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69636/merge to s
2020-03-02T16:32:15.0212263Z Cleaning up task key
2020-03-02T16:32:15.0213674Z Start cleaning up orphan processes.
2020-03-02T16:32:15.0400900Z Terminate orphan process: pid (3751) (python)
2020-03-02T16:32:15.0720366Z ##[section]Finishing: Finalize Job
