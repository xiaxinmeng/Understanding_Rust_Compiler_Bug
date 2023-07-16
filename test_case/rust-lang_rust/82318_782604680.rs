plain
   Compiling rustc-ap-rustc_parse v705.0.0
   Compiling rustc-ap-rustc_attr v705.0.0
   Compiling rustc-ap-rustc_ast_passes v705.0.0
   Compiling racer v2.1.44
error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/ast.rs:564:46
    |
564 |   ...                   let ty = try_continue!(ty).dereference();
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/ast.rs:567:45
    |
567 |   ...                   try_continue!(gen.search_param_by_path(&paths.path));
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:844:15
842 | / macro_rules! try_vec {
843 | |     ($res: expr) => {
843 | |     ($res: expr) => {
844 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
845 | |             Ok(o) => o,
848 | |     };
849 | | }
    | |_- in this expansion of `try_vec!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/fileres.rs:38:25
    |
38  |       let manifest_path = try_vec!(session.project_model.discover_project_manifest(file_path));
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:109:26
    |
109 |           let trait_path = try_continue!(header.trait_path());
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:129:27
    |
129 |           let trait_match = try_continue!(header.resolve_trait(session, &ImportInfo::default()));
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:279:24
    |
279 |               let name = try_continue!(name);
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:280:25
    |
280 |               let type_ = try_continue!(type_);
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:338:31
    |
338 |               let impl_header = try_continue!(ast::parse_impl(
339 | |                 decl,
340 | |                 filepath,
340 | |                 filepath,
341 | |                 blob_range.start + scope_start,
342 | |                 local,
343 | |                 start + n.into(),
    | |______________- in this macro invocation
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:388:31
    |
388 |               let impl_header = try_continue!(ast::parse_impl(
389 | |                 decl,
390 | |                 filepath,
390 | |                 filepath,
391 | |                 blob_range.start + scope_start,
392 | |                 local,
393 | |                 start + n.into(),
    | |______________- in this macro invocation
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:404:21
    |
404 |                       try_continue!(impl_header.trait_path().and_then(|tpath| tpath.name()));
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832  | / macro_rules! try_continue {
833  | |     ($res: expr) => {
833  | |     ($res: expr) => {
834  | |         match ::std::ops::Try::into_result($res) {
     | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835  | |             Ok(o) => o,
838  | |     };
839  | | }
     | |_- in this expansion of `try_continue!`
     | 
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:1723:27
     |
1723 |           let trait_match = try_continue!(header.resolve_trait(session, import_info));
     |
     = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
     = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
     = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:844:15
842  | / macro_rules! try_vec {
843  | |     ($res: expr) => {
843  | |     ($res: expr) => {
844  | |         match ::std::ops::Try::into_result($res) {
     | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
845  | |             Ok(o) => o,
848  | |     };
849  | | }
     | |_- in this expansion of `try_vec!`
     | 
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:2680:25
     |
2680 |                   let m = try_vec!(typeinf::get_type_of_typedef(&m, session));
     |
     = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
     = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
     = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/typeinf.rs:230:31
    |
230 |                       let ret = try_continue!(resolve_lvalue_ty(p, t, query, fpath, pos, session,));
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/typeinf.rs:255:29
    |
255 |   ...                   try_continue!(resolve_lvalue_ty(pat, t, query, fpath, pos, session));
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'try_trait_v2'
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/typeinf.rs:277:29
    |
277 |   ...                   try_continue!(resolve_lvalue_ty(pat, t, query, fpath, pos, session));
    |
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = note: see issue #42327 <https://github.com/rust-lang/rust/issues/42327> for more information
    = help: add `#![feature(try_trait_v2)]` to the crate attributes to enable

error[E0038]: the trait `Try2021` cannot be made into an object
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Try2021` cannot be made into an object
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/ast.rs:564:46
    |
564 |   ...                   let ty = try_continue!(ty).dereference();
    |
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

error[E0038]: the trait `Try2021` cannot be made into an object
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Try2021` cannot be made into an object
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/ast.rs:567:45
    |
567 |   ...                   try_continue!(gen.search_param_by_path(&paths.path));
    |
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

error[E0038]: the trait `Try2021` cannot be made into an object
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:844:15
842 | / macro_rules! try_vec {
843 | |     ($res: expr) => {
843 | |     ($res: expr) => {
844 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Try2021` cannot be made into an object
845 | |             Ok(o) => o,
848 | |     };
849 | | }
    | |_- in this expansion of `try_vec!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/fileres.rs:38:25
    |
38  |       let manifest_path = try_vec!(session.project_model.discover_project_manifest(file_path));
    |
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/fileres.rs:38:9
   |
38 |     let manifest_path = try_vec!(session.project_model.discover_project_manifest(file_path));
   |         ^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: within `std::path::Path`, the trait `Sized` is not implemented for `[u8]`
   = note: required because it appears within the type `std::path::Path`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:845:16
842 | / macro_rules! try_vec {
843 | |     ($res: expr) => {
843 | |     ($res: expr) => {
844 | |         match ::std::ops::Try::into_result($res) {
845 | |             Ok(o) => o,
    | |                ^ doesn't have a size known at compile-time
848 | |     };
849 | | }
    | |_- in this expansion of `try_vec!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/fileres.rs:38:25
    |
38  |       let manifest_path = try_vec!(session.project_model.discover_project_manifest(file_path));
    |
    |
    = help: within `std::path::Path`, the trait `Sized` is not implemented for `[u8]`
    = note: required because it appears within the type `std::path::Path`
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature

error[E0038]: the trait `Try2021` cannot be made into an object
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Try2021` cannot be made into an object
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:109:26
    |
109 |           let trait_path = try_continue!(header.trait_path());
    |
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

error[E0038]: the trait `Try2021` cannot be made into an object
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Try2021` cannot be made into an object
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:129:27
    |
129 |           let trait_match = try_continue!(header.resolve_trait(session, &ImportInfo::default()));
    |
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

error[E0038]: the trait `Try2021` cannot be made into an object
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Try2021` cannot be made into an object
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/nameres.rs:279:24
    |
279 |               let name = try_continue!(name);
    |
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

error[E0038]: the trait `Try2021` cannot be made into an object
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/racer-2.1.44/src/racer/util.rs:834:15
832 | / macro_rules! try_continue {
833 | |     ($res: expr) => {
833 | |     ($res: expr) => {
834 | |         match ::std::ops::Try::into_result($res) {
    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Try2021` cannot be made into an object
835 | |             Ok(o) => o,
838 | |     };
839 | | }
    | |_- in this expansion of `try_continue!`
    | 
    | 
---
   Compiling hir_ty v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir_ty)
   Compiling hir v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir)
   Compiling ide_db v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide_db)
   Compiling assists v0.0.0 (/checkout/src/tools/rust-analyzer/crates/assists)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   --> crates/assists/src/handlers/inline_local_variable.rs:67:26
    |
30  |  / pub(crate) fn inline_local_variable(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
31  |  |     let let_stmt = ctx.find_node_at_offset::<ast::LetStmt>()?;
32  |  |     let bind_pat = match let_stmt.pat()? {
33  |  |         ast::Pat::IdentPat(pat) => pat,
67  |  |     let wrap_in_parens = usages
    |  |__________________________^
68  | ||         .references
69  | ||         .iter()
69  | ||         .iter()
70  | ||         .map(|(&file_id, refs)| {
112 | ||         })
112 | ||         })
113 | ||         .collect::<Result<FxHashMap<_, Vec<_>>, _>>()?;
    | ||______________________________________________________^ cannot use the `?` operator in a function that returns `std::option::Option<()>`
141 |  |     )
142 |  | }
142 |  | }
    |  |_- this function should return `Result` or `Option` to accept `?`
    |
    = help: the trait `FromResidual<std::result::Result<!, _>>` is not implemented for `std::option::Option<()>`
    = note: required by `from_residual`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
error: could not compile `assists`
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-analyzer/crates/rust-analyzer/Cargo.toml" "--message-format" "json-render-diagnostics"
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-analyzer/crates/rust-analyzer/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'rust-analyzer always builds', src/bootstrap/dist.rs:1083:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:19:32
