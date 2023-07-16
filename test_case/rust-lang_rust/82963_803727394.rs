plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0255]: the name `SharedContext` is defined multiple times
  --> src/librustdoc/html/render/context.rs:82:1
   |
20 |     print_sidebar, settings, AllTypes, NameDoc, SharedContext, StylePath, BASIC_KEYWORDS,
   |                                                 ------------- previous import of the type `SharedContext` here
...
82 | crate struct SharedContext<'tcx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SharedContext` redefined here
   |
   = note: `SharedContext` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
   |
20 |     print_sidebar, settings, AllTypes, NameDoc, SharedContext as OtherSharedContext, StylePath, BASIC_KEYWORDS,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/context.rs:103:27
    |
103 |     created_dirs: RefCell<FxHashSet<PathBuf>>,
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-hash-1.1.0/src/lib.rs:43:1
    |
    |
43  | pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
    | ----------------------------------------------------------------------- similarly named type alias `FxHashMap` defined here
help: a type alias with a similar name exists
    |
    |
103 |     created_dirs: RefCell<FxHashMap<PathBuf>>,
help: consider importing one of these items
    |
    |
1   | use crate::html::render::FxHashSet;
1   | use rustc_data_structures::stable_set::FxHashSet;
    |

error[E0412]: cannot find type `Receiver` in this scope
---
    |
1   | use std::sync::mpsc::Receiver;
    |

error[E0412]: cannot find type `Path` in this scope
   --> src/librustdoc/html/render/context.rs:132:38
    |
132 |     crate fn ensure_dir(&self, dst: &Path) -> Result<(), Error> {
    |
help: consider importing one of these items
    |
1   | use crate::clean::Path;
---

error: unused import: `SharedContext`
  --> src/librustdoc/html/render/context.rs:20:49
   |
20 |     print_sidebar, settings, AllTypes, NameDoc, SharedContext, StylePath, BASIC_KEYWORDS,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: unused import: `RefCell`
  --> src/librustdoc/html/render/mod.rs:38:23
   |
38 | use std::cell::{Cell, RefCell};

error: unused import: `Path`
  --> src/librustdoc/html/render/mod.rs:42:17
   |
   |
42 | use std::path::{Path, PathBuf};

error: unused import: `std::sync::mpsc::Receiver`
  --> src/librustdoc/html/render/mod.rs:45:5
   |
   |
45 | use std::sync::mpsc::Receiver;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `FxHashMap`
  --> src/librustdoc/html/render/mod.rs:50:33
   |
50 | use rustc_data_structures::fx::{FxHashMap, FxHashSet};

error: unused import: `rustc_middle::ty::TyCtxt`
  --> src/librustdoc/html/render/mod.rs:56:5
   |
---
   |
57 | use rustc_span::edition::Edition;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `DocFS`
  --> src/librustdoc/html/render/mod.rs:63:20
   |
63 | use crate::docfs::{DocFS, PathError};

error: unused import: `crate::html::layout`
  --> src/librustdoc/html/render/mod.rs:73:5
   |
   |
73 | use crate::html::layout;
   |     ^^^^^^^^^^^^^^^^^^^

error: unused imports: `ErrorCodes`, `self`
  --> src/librustdoc/html/render/mod.rs:74:29
   |
74 | use crate::html::markdown::{self, ErrorCodes, Markdown, MarkdownHtml, MarkdownSummaryLine};
   |                             ^^^^  ^^^^^^^^^^

error[E0616]: field `codes` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:570:23
    |
570 |             cx.shared.codes,

error[E0616]: field `playground` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:572:24
    |
    |
572 |             &cx.shared.playground


error[E0616]: field `codes` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:696:33
    |
696 |     let error_codes = cx.shared.codes;

error[E0616]: field `playground` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:726:28
    |
    |
726 |                 &cx.shared.playground,


error[E0616]: field `issue_tracker_base_url` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:748:55
    |
748 |         if let (Some(url), Some(issue)) = (&cx.shared.issue_tracker_base_url, issue) {

error[E0616]: field `playground` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:768:32
    |
    |
768 |                     &cx.shared.playground,


error[E0616]: field `codes` of struct `context::SharedContext` is private
    --> src/librustdoc/html/render/mod.rs:1414:31
     |
1414 |                     cx.shared.codes,

error[E0616]: field `playground` of struct `context::SharedContext` is private
    --> src/librustdoc/html/render/mod.rs:1416:32
     |
     |
1416 |                     &cx.shared.playground


error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/librustdoc/html/render/context.rs:595:56
    |
595 |         let nb_errors = self.shared.errors.iter().map(|err| diag.struct_err(&err).emit()).count();
    |                                                        ^^^ doesn't have a size known at compile-time
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = help: unsized fn params are gated as an unstable feature
help: function arguments must have a statically known size, borrowed types always have a known size
    |
595 |         let nb_errors = self.shared.errors.iter().map(|&err| diag.struct_err(&err).emit()).count();


error[E0616]: field `sort_modules_alphabetically` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/print_item.rs:184:18
    |
184 |     if cx.shared.sort_modules_alphabetically {

error: aborting due to 24 previous errors

Some errors have detailed explanations: E0255, E0277, E0412, E0616.
