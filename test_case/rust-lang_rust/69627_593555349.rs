plain
2020-03-02T17:53:47.8938074Z [TIMING] BookTest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, path: "src/doc/book", name: "book", is_ext_doc: true } -- 61.591
2020-03-02T17:53:47.8945142Z Testing rustbook src/doc/nomicon
2020-03-02T17:53:54.5415177Z Error: Rustdoc returned an error: 
2020-03-02T17:53:54.5415589Z running 6 tests
2020-03-02T17:53:54.5416748Z test /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 128) ... FAILED
2020-03-02T17:53:54.5417307Z test /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 28) ... FAILED
2020-03-02T17:53:54.5418006Z test /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 110) ... ok
2020-03-02T17:53:54.5418508Z test /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 6) ... ignored
2020-03-02T17:53:54.5419001Z test /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 55) ... FAILED
2020-03-02T17:53:54.5419498Z test /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 96) ... ok
2020-03-02T17:53:54.5419831Z failures:
2020-03-02T17:53:54.5419929Z 
2020-03-02T17:53:54.5419929Z 
2020-03-02T17:53:54.5420309Z ---- /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 128) stdout ----
2020-03-02T17:53:54.5420645Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-02T17:53:54.5421082Z  --> /tmp/mdbook-nFaEfA/destructors.md:131:18
2020-03-02T17:53:54.5421499Z 4 | use std::alloc::{Alloc, GlobalAlloc, Global, Layout};
2020-03-02T17:53:54.5421739Z   |                  ^^^^^
2020-03-02T17:53:54.5421935Z   |                  |
2020-03-02T17:53:54.5422255Z   |                  no `Alloc` in `alloc`
2020-03-02T17:53:54.5422255Z   |                  no `Alloc` in `alloc`
2020-03-02T17:53:54.5422582Z   |                  help: a similar name exists in the module: `alloc`
2020-03-02T17:53:54.5422798Z 
2020-03-02T17:53:54.5423093Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T17:53:54.5423643Z   --> /tmp/mdbook-nFaEfA/destructors.md:142:20
2020-03-02T17:53:54.5424722Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-02T17:53:54.5425105Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T17:53:54.5425387Z    |
2020-03-02T17:53:54.5425624Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T17:53:54.5425624Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T17:53:54.5426012Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-02T17:53:54.5426280Z    |
2020-03-02T17:53:54.5426475Z 4  | use std::alloc::AllocRef;
2020-03-02T17:53:54.5426656Z    |
2020-03-02T17:53:54.5426774Z 
2020-03-02T17:53:54.5427075Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T17:53:54.5427818Z   --> /tmp/mdbook-nFaEfA/destructors.md:157:20
2020-03-02T17:53:54.5428423Z 30 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-02T17:53:54.5428799Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T17:53:54.5429412Z    |
2020-03-02T17:53:54.5429646Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-02T17:53:54.5431016Z 
2020-03-02T17:53:54.5431231Z Some errors have detailed explanations: E0432, E0599.
2020-03-02T17:53:54.5431734Z For more information about an error, try `rustc --explain E0432`.
2020-03-02T17:53:54.5432124Z Couldn't compile the test.
2020-03-02T17:53:54.5432545Z ---- /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 28) stdout ----
2020-03-02T17:53:54.5432872Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-02T17:53:54.5433289Z  --> /tmp/mdbook-nFaEfA/destructors.md:31:18
2020-03-02T17:53:54.5433695Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-03-02T17:53:54.5434131Z   |                  ^^^^^
2020-03-02T17:53:54.5434512Z   |                  |
2020-03-02T17:53:54.5434723Z   |                  no `Alloc` in `alloc`
2020-03-02T17:53:54.5434723Z   |                  no `Alloc` in `alloc`
2020-03-02T17:53:54.5435053Z   |                  help: a similar name exists in the module: `alloc`
2020-03-02T17:53:54.5435380Z 
2020-03-02T17:53:54.5435688Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T17:53:54.5436262Z   --> /tmp/mdbook-nFaEfA/destructors.md:42:20
2020-03-02T17:53:54.5436716Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>())
2020-03-02T17:53:54.5437100Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T17:53:54.5437355Z    |
2020-03-02T17:53:54.5437806Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-02T17:53:54.5439327Z 
2020-03-02T17:53:54.5439541Z Some errors have detailed explanations: E0432, E0599.
2020-03-02T17:53:54.5440015Z For more information about an error, try `rustc --explain E0432`.
2020-03-02T17:53:54.5440403Z Couldn't compile the test.
2020-03-02T17:53:54.5440911Z ---- /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 55) stdout ----
2020-03-02T17:53:54.5441235Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-02T17:53:54.5441694Z  --> /tmp/mdbook-nFaEfA/destructors.md:58:18
2020-03-02T17:53:54.5442102Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-03-02T17:53:54.5442337Z   |                  ^^^^^
2020-03-02T17:53:54.5442529Z   |                  |
2020-03-02T17:53:54.5442728Z   |                  no `Alloc` in `alloc`
2020-03-02T17:53:54.5442728Z   |                  no `Alloc` in `alloc`
2020-03-02T17:53:54.5443039Z   |                  help: a similar name exists in the module: `alloc`
2020-03-02T17:53:54.5443252Z 
2020-03-02T17:53:54.5443542Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T17:53:54.5444471Z   --> /tmp/mdbook-nFaEfA/destructors.md:69:20
2020-03-02T17:53:54.5444948Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-02T17:53:54.5445321Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T17:53:54.5445592Z    |
2020-03-02T17:53:54.5447187Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T17:53:54.5447187Z    = help: items from traits can only be used if the trait is in scope
2020-03-02T17:53:54.5448828Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-02T17:53:54.5449073Z    |
2020-03-02T17:53:54.5449250Z 4  | use std::alloc::AllocRef;
2020-03-02T17:53:54.5449416Z    |
2020-03-02T17:53:54.5449523Z 
2020-03-02T17:53:54.5449798Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-02T17:53:54.5450449Z   --> /tmp/mdbook-nFaEfA/destructors.md:82:20
2020-03-02T17:53:54.5450890Z 28 |             Global.dealloc(c.cast::<u8>(), Layout::new::<T>());
2020-03-02T17:53:54.5451280Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-02T17:53:54.5451520Z    |
2020-03-02T17:53:54.5451758Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-02T17:53:54.5454192Z For more information about an error, try `rustc --explain E0432`.
2020-03-02T17:53:54.5454875Z Couldn't compile the test.
2020-03-02T17:53:54.5455015Z 
2020-03-02T17:53:54.5455163Z failures:
2020-03-02T17:53:54.5455566Z     /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 128)
2020-03-02T17:53:54.5456067Z     /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 28)
2020-03-02T17:53:54.5456874Z     /tmp/mdbook-nFaEfA/destructors.md - Destructors (line 55)
2020-03-02T17:53:54.5457326Z test result: FAILED. 2 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
2020-03-02T17:53:54.5457892Z 
2020-03-02T17:53:54.5458167Z 
2020-03-02T17:53:54.5458257Z 
---
2020-03-02T17:53:54.5461254Z [TIMING] BookTest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, path: "src/doc/nomicon", name: "nomicon", is_ext_doc: true } -- 6.647
2020-03-02T17:53:54.5461842Z Testing rustbook src/doc/reference
2020-03-02T17:54:10.5916301Z Error: Rustdoc returned an error: 
2020-03-02T17:54:10.5917150Z running 2 tests
2020-03-02T17:54:10.5919027Z test /tmp/mdbook-jYA3gm/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) ... FAILED
2020-03-02T17:54:10.5920455Z test /tmp/mdbook-jYA3gm/expressions/array-expr.md - Array_and_array_index_expressions::Array_expressions (line 27) ... ok
2020-03-02T17:54:10.5921454Z failures:
2020-03-02T17:54:10.5921757Z 
2020-03-02T17:54:10.5921757Z 
2020-03-02T17:54:10.5923017Z ---- /tmp/mdbook-jYA3gm/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) stdout ----
2020-03-02T17:54:10.5923792Z error: this operation will panic at runtime
2020-03-02T17:54:10.5925001Z   --> /tmp/mdbook-jYA3gm/expressions/array-expr.md:72:9
2020-03-02T17:54:10.5925980Z 11 | let x = (["a", "b"])[10]; // warning: index out of bounds
2020-03-02T17:54:10.5926683Z    |         ^^^^^^^^^^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-03-02T17:54:10.5927259Z    |
2020-03-02T17:54:10.5927732Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-02T17:54:10.5927732Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-02T17:54:10.5928193Z 
2020-03-02T17:54:10.5928882Z error: this operation will panic at runtime
2020-03-02T17:54:10.5929527Z   --> /tmp/mdbook-jYA3gm/expressions/array-expr.md:78:1
2020-03-02T17:54:10.5933075Z 17 | arr[10];                  // warning: index out of bounds
2020-03-02T17:54:10.5934023Z    | ^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-03-02T17:54:10.5934243Z 
2020-03-02T17:54:10.5934674Z error: aborting due to 2 previous errors
2020-03-02T17:54:10.5934674Z error: aborting due to 2 previous errors
2020-03-02T17:54:10.5935005Z 
2020-03-02T17:54:10.5935807Z Couldn't compile the test.
2020-03-02T17:54:10.5935955Z 
2020-03-02T17:54:10.5936104Z failures:
2020-03-02T17:54:10.5936739Z     /tmp/mdbook-jYA3gm/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63)
2020-03-02T17:54:10.5937344Z test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-02T17:54:10.5937587Z 
2020-03-02T17:54:10.5937679Z 
2020-03-02T17:54:10.5937773Z 
---
2020-03-02T17:58:59.9212763Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Ident.html"
2020-03-02T17:58:59.9213137Z 
2020-03-02T17:58:59.9223019Z     ┌── test-implementation.md:69:49 ───
2020-03-02T17:58:59.9226646Z     │
2020-03-02T17:58:59.9227604Z  69 │ not stored as a string, but rather as an opaque [Symbol][Symbol] which is
2020-03-02T17:58:59.9229779Z     │
2020-03-02T17:58:59.9230038Z 
2020-03-02T17:58:59.9231084Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_expand/mbe/macro_parser/fn.parse.html"
2020-03-02T17:58:59.9231616Z 
---
2020-03-02T17:58:59.9261375Z error: Unable to retrieve "https://clang.llvm.org/docs/UsersManual.html": https://clang.llvm.org/docs/UsersManual.html: timed out
2020-03-02T17:58:59.9261734Z 
2020-03-02T17:58:59.9262127Z     ┌── profile-guided-optimization.md:23:15 ───
2020-03-02T17:58:59.9262496Z     │
2020-03-02T17:58:59.9262922Z  23 │ LLVM actually [supports multiple forms][clang-pgo] of PGO:
2020-03-02T17:58:59.9263507Z     │               ^ https://clang.llvm.org/docs/UsersManual.html: timed out
2020-03-02T17:58:59.9264038Z 
2020-03-02T17:58:59.9264584Z error: The server responded with 404 Not Found for "https://github.com/rust-lang/rust/tree/master/src/libsyntax"
2020-03-02T17:58:59.9264916Z 
2020-03-02T17:58:59.9265605Z     ┌── appendix/stupid-stats.md:64:43 ───
---
2020-03-02T17:58:59.9293178Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ptr/struct.P.html"
2020-03-02T17:58:59.9293755Z 
2020-03-02T17:58:59.9294409Z     ┌── appendix/code-index.md:18:122 ───
2020-03-02T17:58:59.9294767Z     │
2020-03-02T17:58:59.9296874Z  18 │ `P` | struct | An owned immutable smart pointer. By contrast, `&T` is not owned, and `Box<T>` is not immutable. | None | [src/syntax/ptr.rs](https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ptr/struct.P.html)
2020-03-02T17:58:59.9299342Z     │
2020-03-02T17:58:59.9299469Z 
2020-03-02T17:58:59.9300399Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/tokenstream/struct.TokenStream.html"
2020-03-02T17:58:59.9301223Z 
2020-03-02T17:58:59.9301223Z 
2020-03-02T17:58:59.9301747Z     ┌── appendix/code-index.md:28:144 ───
2020-03-02T17:58:59.9302117Z     │
2020-03-02T17:58:59.9303082Z  28 │ `syntax::token_stream::TokenStream` | struct | An abstract sequence of tokens, organized into `TokenTree`s | [The parser], [Macro expansion] | [src/libsyntax/tokenstream.rs](https://doc.rust-lang.org/nightly/nightly-rustc/syntax/tokenstream/struct.TokenStream.html)
2020-03-02T17:58:59.9305591Z     │
2020-03-02T17:58:59.9305709Z 
2020-03-02T17:58:59.9306006Z 
2020-03-02T17:58:59.9306107Z 
---
2020-03-02T18:10:47.3851040Z 
2020-03-02T18:10:47.3851400Z ------------------------------------------
2020-03-02T18:10:47.3851594Z stderr:
2020-03-02T18:10:47.3851979Z ------------------------------------------
2020-03-02T18:10:47.3857812Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":174,"byte_end":179,"line_start":9,"line_end":9,"column_start":1,"column_end":6,"is_primary":false,"text":[{"text":"const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable","highlight_start":1,"highlight_end":6}],"label":"make this a static item (maybe with lazy_static)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":174,"byte_end":222,"line_start":9,"line_end":9,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable","highlight_start":1,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::declare-interior-mutable-const` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:9:1\n   |\nLL | const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable\n   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   | |\n   | make this a static item (maybe with lazy_static)\n   |\n   = note: `-D clippy::declare-interior-mutable-const` implied by `-D warnings`\n\n"}
2020-03-02T18:10:47.3866262Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":250,"byte_end":255,"line_start":10,"line_end":10,"column_start":1,"column_end":6,"is_primary":false,"text":[{"text":"const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable","highlight_start":1,"highlight_end":6}],"label":"make this a static item (maybe with lazy_static)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":250,"byte_end":289,"line_start":10,"line_end":10,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:10:1\n   |\nLL | const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable\n   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   | |\n   | make this a static item (maybe with lazy_static)\n\n"}
2020-03-02T18:10:47.3874173Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":317,"byte_end":322,"line_start":11,"line_end":11,"column_start":1,"column_end":6,"is_primary":false,"text":[{"text":"const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);","highlight_start":1,"highlight_end":6}],"label":"make this a static item (maybe with lazy_static)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":317,"byte_end":406,"line_start":11,"line_end":11,"column_start":1,"column_end":90,"is_primary":true,"text":[{"text":"const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);","highlight_start":1,"highlight_end":90}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:11:1\n   |\nLL | const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);\n   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   | |\n   | make this a static item (maybe with lazy_static)\n\n"}
2020-03-02T18:10:47.3883616Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":514,"byte_end":536,"line_start":16,"line_end":16,"column_start":9,"column_end":31,"is_primary":true,"text":[{"text":"        const $name: $ty = $e;","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":546,"byte_end":588,"line_start":19,"line_end":19,"column_start":1,"column_end":43,"is_primary":false,"text":[{"text":"declare_const!(_ONCE: Once = Once::new()); //~ ERROR interior mutable","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"declare_const!","def_site_span":{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":436,"byte_end":545,"line_start":14,"line_end":18,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! declare_const {","highlight_start":1,"highlight_end":29},{"text":"    ($name:ident: $ty:ty = $e:expr) => {","highlight_start":1,"highlight_end":41},{"text":"        const $name: $ty = $e;","highlight_start":1,"highlight_end":31},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:16:9\n   |\nLL |         const $name: $ty = $e;\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n...\nLL | declare_const!(_ONCE: Once = Once::new()); //~ ERROR interior mutable\n   | ------------------------------------------ in this macro invocation\n   |\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
2020-03-02T18:10:47.3891335Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":1191,"byte_end":1217,"line_start":40,"line_end":40,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    const ATOMIC: AtomicUsize; //~ ERROR interior mutable","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:40:5\n   |\nLL |     const ATOMIC: AtomicUsize; //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3895474Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":1335,"byte_end":1350,"line_start":44,"line_end":44,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    const INPUT: T;","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:44:5\n   |\nLL |     const INPUT: T;\n   |     ^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3899607Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":1437,"byte_end":1468,"line_start":47,"line_end":47,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    const ASSOC: Self::NonCopyType;","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:47:5\n   |\nLL |     const ASSOC: Self::NonCopyType;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3903128Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":1586,"byte_end":1618,"line_start":51,"line_end":51,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    const AN_INPUT: T = Self::INPUT;","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:51:5\n   |\nLL |     const AN_INPUT: T = Self::INPUT;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3911786Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":514,"byte_end":536,"line_start":16,"line_end":16,"column_start":9,"column_end":31,"is_primary":true,"text":[{"text":"        const $name: $ty = $e;","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":1706,"byte_end":1753,"line_start":54,"line_end":54,"column_start":5,"column_end":52,"is_primary":false,"text":[{"text":"    declare_const!(ANOTHER_INPUT: T = Self::INPUT); //~ ERROR interior mutable","highlight_start":5,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"declare_const!","def_site_span":{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":436,"byte_end":545,"line_start":14,"line_end":18,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! declare_const {","highlight_start":1,"highlight_end":29},{"text":"    ($name:ident: $ty:ty = $e:expr) => {","highlight_start":1,"highlight_end":41},{"text":"        const $name: $ty = $e;","highlight_start":1,"highlight_end":31},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:16:9\n   |\nLL |         const $name: $ty = $e;\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n...\nLL |     declare_const!(ANOTHER_INPUT: T = Self::INPUT); //~ ERROR interior mutable\n   |     ----------------------------------------------- in this macro invocation\n   |\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
2020-03-02T18:10:47.3918969Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":1829,"byte_end":1848,"line_start":60,"line_end":60,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    const SELF_2: Self;","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:60:5\n   |\nLL |     const SELF_2: Self;\n   |     ^^^^^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3922639Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":2435,"byte_end":2485,"line_start":81,"line_end":81,"column_start":5,"column_end":55,"is_primary":true,"text":[{"text":"    const ASSOC_3: AtomicUsize = AtomicUsize::new(14); //~ ERROR interior mutable","highlight_start":5,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:81:5\n   |\nLL |     const ASSOC_3: AtomicUsize = AtomicUsize::new(14); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3926763Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":2610,"byte_end":2638,"line_start":84,"line_end":84,"column_start":5,"column_end":33,"is_primary":true,"text":[{"text":"    const U_SELF: U = U::SELF_2;","highlight_start":5,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:84:5\n   |\nLL |     const U_SELF: U = U::SELF_2;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3930572Z {"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const.rs","byte_start":2725,"byte_end":2766,"line_start":87,"line_end":87,"column_start":5,"column_end":46,"is_primary":true,"text":[{"text":"    const T_ASSOC: T::NonCopyType = T::ASSOC;","highlight_start":5,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const.rs:87:5\n   |\nLL |     const T_ASSOC: T::NonCopyType = T::ASSOC;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2020-03-02T18:10:47.3933623Z 
2020-03-02T18:10:47.3934061Z ------------------------------------------
2020-03-02T18:10:47.3934424Z 
2020-03-02T18:10:47.3934695Z test [ui] ui/declare_interior_mutable_const.rs ... FAILED
---
2020-03-02T18:43:36.6771529Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2020-03-02T18:43:36.6771799Z 
2020-03-02T18:43:36.6772280Z We detected that this PR updated 'clippy-driver', but its tests failed.
2020-03-02T18:43:36.6772557Z 
2020-03-02T18:43:36.6773078Z If you do intend to update 'clippy-driver', please check the error messages above and
2020-03-02T18:43:36.6773451Z commit another update.
2020-03-02T18:43:36.6773607Z 
2020-03-02T18:43:36.6774291Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2020-03-02T18:43:36.6775342Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2020-03-02T18:43:36.6775681Z proper steps.
2020-03-02T18:43:36.6777532Z Build completed unsuccessfully in 0:00:01
2020-03-02T18:43:36.6825758Z == clock drift check ==
2020-03-02T18:43:36.6838843Z   local time: Mon Mar  2 18:43:36 UTC 2020
2020-03-02T18:43:36.9769687Z   network time: Mon, 02 Mar 2020 18:43:36 GMT
2020-03-02T18:43:36.9769687Z   network time: Mon, 02 Mar 2020 18:43:36 GMT
2020-03-02T18:43:36.9773796Z == end clock drift check ==
2020-03-02T18:43:37.6782260Z 
2020-03-02T18:43:37.6855690Z ##[error]Bash exited with code '1'.
2020-03-02T18:43:37.6918520Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-02T18:43:37.6923056Z ==============================================================================
2020-03-02T18:43:37.6923333Z Task         : Get sources
2020-03-02T18:43:37.6923633Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
