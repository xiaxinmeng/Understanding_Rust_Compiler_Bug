plain
   |
35 | mod shared_context;
   | ^^^^^^^^^^^^^^^^^^^
   |
   = help: to create the module `shared_context`, create file "src/librustdoc/html/render/shared_context.rs"
error[E0433]: failed to resolve: use of undeclared crate or module `layout`
  --> src/librustdoc/html/render/mod.rs:93:19
   |
93 |     crate layout: layout::Layout,
93 |     crate layout: layout::Layout,
   |                   ^^^^^^ use of undeclared crate or module `layout`

error[E0433]: failed to resolve: use of undeclared crate or module `markdown`
   --> src/librustdoc/html/render/mod.rs:124:24
    |
124 |     playground: Option<markdown::Playground>,
    |                        ^^^^^^^^ use of undeclared crate or module `markdown`

error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/render/mod.rs:99:26
   |
99 |     crate local_sources: FxHashMap<PathBuf, String>,
   | 
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-hash-1.1.0/src/lib.rs:47:1
   |
   |
47 | pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;
   | ----------------------------------------------------------------- similarly named type alias `FxHashSet` defined here
help: a type alias with a similar name exists
   |
   |
99 |     crate local_sources: FxHashSet<PathBuf, String>,
help: consider importing one of these items
   |
   |
38 | use crate::visit_ast::FxHashMap;
38 | use rustc_data_structures::stable_map::FxHashMap;
   |


error[E0412]: cannot find type `DocFS` in this scope
   --> src/librustdoc/html/render/mod.rs:120:15
    |
120 |     crate fs: DocFS,
    |
help: consider importing this struct
    |
    |
38  | use crate::docfs::DocFS;


error[E0412]: cannot find type `ErrorCodes` in this scope
   --> src/librustdoc/html/render/mod.rs:123:18
123 |     crate codes: ErrorCodes,
    |                  ^^^^^^^^^^ not found in this scope
    |
help: consider importing this enum
help: consider importing this enum
    |
38  | use crate::passes::calculate_doc_coverage::ErrorCodes;
    |

error[E0412]: cannot find type `Path` in this scope
   --> src/librustdoc/html/render/mod.rs:128:38
    |
128 |     crate fn ensure_dir(&self, dst: &Path) -> Result<(), Error> {
    |
help: consider importing one of these items
    |
38  | use crate::clean::Path;
---
    |
38  | use std::path::Path;
    |

error: unused import: `shared_context::*`
  --> src/librustdoc/html/render/mod.rs:39:11
   |
39 | crate use shared_context::*;
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0412, E0433, E0583.
For more information about an error, try `rustc --explain E0412`.
