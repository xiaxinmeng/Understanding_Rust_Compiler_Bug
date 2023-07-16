
thread 'rustc' panicked at 'get_index_type_name(clean_type: ResolvedPath { path: Path { global: false, def: TyParam(DefId(0/1:10)), segments: [] }, typarams: None, did: DefId(0/1:1
0), is_generic: true }, accept_generic: true) had length zero path', src\librustdoc\html\render.rs:4133:77
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys_common::backtrace::_print
   1: std::panicking::Location::column
   2: std::panicking::Location::column
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic
   5: std::panicking::begin_panic_fmt
   6: rustdoc::html::render::get_index_type_name
             at .\src\librustdoc\html\render.rs:4145
   7: rustdoc::html::render::get_index_type
             at .\src\librustdoc\html\render.rs:4123
   8: alloc::vec::{{impl}}::from_iter<rustdoc::html::render::Type,core::iter::Map<core::slice::Iter<rustdoc::clean::Argument>, closure>>
             at .\src\liballoc\vec.rs:1825
   9: rustdoc::html::render::{{impl}}::fold_item
             at .\src\librustdoc\html\render.rs:1269
  10: alloc::vec::{{impl}}::from_iter<rustdoc::clean::Item,core::iter::FilterMap<alloc::vec::IntoIter<rustdoc::clean::Item>, closure>>
             at .\src\liballoc\vec.rs:1795
  11: rustdoc::fold::DocFolder::fold_inner_recur<rustdoc::html::render::Cache>
             at .\src\librustdoc\fold.rs:42
  12: rustdoc::html::render::{{impl}}::fold_item
             at .\src\librustdoc\html\render.rs:1367
  13: rustdoc::html::render::run
             at .\src\librustdoc\html\render.rs:636


------------------------------------------

thread '[rustdoc] rustdoc\issue-46976.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2776:8
