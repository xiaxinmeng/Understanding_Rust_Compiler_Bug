
syntax-65954e2afb111716.dll!core::ptr::drop_in_place<syntax::ext::tt::macro_parser::MatcherPosHandle>(syntax::ext::tt::macro_parser::MatcherPosHandle *) Line 59
	at C:\Users\Peter\Code\rust\src\libcore\ptr.rs(59)
[Inline Frame] syntax-65954e2afb111716.dll!core::ptr::drop_in_place(mut slice<syntax::ext::tt::macro_parser::MatcherPosHandle>*) Line 59
	at C:\Users\Peter\Code\rust\src\libcore\ptr.rs(59)
[Inline Frame] syntax-65954e2afb111716.dll!alloc::vec::{{impl}}::drop(alloc::vec::Vec<syntax::ext::tt::macro_parser::MatcherPosHandle> * self) Line 2186
	at C:\Users\Peter\Code\rust\src\liballoc\vec.rs(2186)
[Inline Frame] syntax-65954e2afb111716.dll!core::ptr::drop_in_place(alloc::vec::Vec<syntax::ext::tt::macro_parser::MatcherPosHandle> *) Line 59
	at C:\Users\Peter\Code\rust\src\libcore\ptr.rs(59)
[Inline Frame] syntax-65954e2afb111716.dll!core::ptr::drop_in_place(rustc_data_structures::accumulate_vec::AccumulateVec<[syntax::ext::tt::macro_parser::MatcherPosHandle; 1]> *) Line 59
	at C:\Users\Peter\Code\rust\src\libcore\ptr.rs(59)
[Inline Frame] syntax-65954e2afb111716.dll!core::ptr::drop_in_place(rustc_data_structures::small_vec::SmallVec<[syntax::ext::tt::macro_parser::MatcherPosHandle; 1]> *) Line 59
	at C:\Users\Peter\Code\rust\src\libcore\ptr.rs(59)
[Inline Frame] syntax-65954e2afb111716.dll!core::mem::drop(rustc_data_structures::small_vec::SmallVec<[syntax::ext::tt::macro_parser::MatcherPosHandle; 1]>) Line 795
	at C:\Users\Peter\Code\rust\src\libcore\mem.rs(795)
syntax-65954e2afb111716.dll!syntax::ext::tt::macro_parser::parse(syntax::parse::ParseSess * sess, syntax::tokenstream::TokenStream tts, slice<syntax::ext::tt::quoted::TokenTree>* ms, core::option::Option<syntax::parse::Directory> directory, bool) Line 703
	at C:\Users\Peter\Code\rust\src\libsyntax\ext\tt\macro_parser.rs(703)
syntax-65954e2afb111716.dll!syntax::ext::tt::macro_rules::compile(syntax::parse::ParseSess * sess, syntax::feature_gate::Features * features, syntax::ast::Item * def, syntax_pos::edition::Edition edition) Line 224
	at C:\Users\Peter\Code\rust\src\libsyntax\ext\tt\macro_rules.rs(224)
rustc_resolve-0b651c3d6100cc9d.dll!rustc_resolve::Resolver::define_macro(syntax::ast::Item * self, syntax_pos::hygiene::Mark item, rustc_resolve::macros::LegacyScope * expansion) Line 1024
	at C:\Users\Peter\Code\rust\src\librustc_resolve\macros.rs(1024)
[Inline Frame] rustc_resolve-0b651c3d6100cc9d.dll!syntax::visit::walk_mod(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * visitor, syntax::ast::Mod * module) Line 166
	at C:\Users\Peter\Code\rust\src\libsyntax\visit.rs(166)
[Inline Frame] rustc_resolve-0b651c3d6100cc9d.dll!syntax::visit::Visitor::visit_mod(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * self, syntax::ast::Mod * m, syntax_pos::span_encoding::Span _s, slice<syntax::ast::Attribute>* _n, syntax::ast::NodeId) Line 60
	at C:\Users\Peter\Code\rust\src\libsyntax\visit.rs(60)
rustc_resolve-0b651c3d6100cc9d.dll!syntax::visit::walk_item<rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor>(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * visitor, syntax::ast::Item * item) Line 245
	at C:\Users\Peter\Code\rust\src\libsyntax\visit.rs(245)
rustc_resolve-0b651c3d6100cc9d.dll!rustc_resolve::build_reduced_graph::{{impl}}::visit_item(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * self, syntax::ast::Item * item) Line 833
	at C:\Users\Peter\Code\rust\src\librustc_resolve\build_reduced_graph.rs(833)
[Inline Frame] rustc_resolve-0b651c3d6100cc9d.dll!syntax::visit::walk_mod(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * visitor, syntax::ast::Mod * module) Line 166
	at C:\Users\Peter\Code\rust\src\libsyntax\visit.rs(166)
[Inline Frame] rustc_resolve-0b651c3d6100cc9d.dll!syntax::visit::Visitor::visit_mod(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * self, syntax::ast::Mod * m, syntax_pos::span_encoding::Span _s, slice<syntax::ast::Attribute>* _n, syntax::ast::NodeId) Line 60
	at C:\Users\Peter\Code\rust\src\libsyntax\visit.rs(60)
rustc_resolve-0b651c3d6100cc9d.dll!syntax::visit::walk_item<rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor>(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * visitor, syntax::ast::Item * item) Line 245
	at C:\Users\Peter\Code\rust\src\libsyntax\visit.rs(245)
rustc_resolve-0b651c3d6100cc9d.dll!rustc_resolve::build_reduced_graph::{{impl}}::visit_item(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * self, syntax::ast::Item * item) Line 833
	at C:\Users\Peter\Code\rust\src\librustc_resolve\build_reduced_graph.rs(833)
rustc_resolve-0b651c3d6100cc9d.dll!syntax::ext::expand::AstFragment::visit_with<rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor>(rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor * self) Line 119
	at C:\Users\Peter\Code\rust\src\libsyntax\ext\expand.rs(119)
rustc_resolve-0b651c3d6100cc9d.dll!rustc_resolve::macros::{{impl}}::visit_ast_fragment_with_placeholders(rustc_resolve::Resolver * self, syntax_pos::hygiene::Mark mark, syntax::ext::expand::AstFragment * fragment, slice<syntax_pos::hygiene::Mark>* derives) Line 211
	at C:\Users\Peter\Code\rust\src\librustc_resolve\macros.rs(211)
syntax-65954e2afb111716.dll!syntax::ext::expand::MacroExpander::collect_invocations(syntax::ext::expand::AstFragment self, slice<syntax_pos::hygiene::Mark>* fragment) Line 497
	at C:\Users\Peter\Code\rust\src\libsyntax\ext\expand.rs(497)
syntax-65954e2afb111716.dll!syntax::ext::expand::MacroExpander::expand_fragment(syntax::ext::expand::AstFragment self) Line 324
	at C:\Users\Peter\Code\rust\src\libsyntax\ext\expand.rs(324)
syntax-65954e2afb111716.dll!syntax::ext::expand::MacroExpander::expand_crate(syntax::ast::Crate self) Line 299
	at C:\Users\Peter\Code\rust\src\libsyntax\ext\expand.rs(299)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}::{{closure}}(closure) Line 1006
	at C:\Users\Peter\Code\rust\src\librustc_driver\driver.rs(1006)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc::util::common::time_ext(bool do_it, core::option::Option<rustc::session::Session*> sess, str* what, closure) Line 165
	at C:\Users\Peter\Code\rust\src\librustc\util\common.rs(165)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc::util::common::time(rustc::session::Session * sess, str* what, closure) Line 159
	at C:\Users\Peter\Code\rust\src\librustc\util\common.rs(159)
rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}<closure>(closure) Line 1005
	at C:\Users\Peter\Code\rust\src\librustc_driver\driver.rs(1005)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc::util::common::time_ext(bool do_it, core::option::Option<rustc::session::Session*> sess, str* what, closure) Line 165
	at C:\Users\Peter\Code\rust\src\librustc\util\common.rs(165)
rustc_driver-295c99f0d90ee7b8.dll!rustc::util::common::time<syntax::ast::Crate,closure>(rustc::session::Session * sess, str* what, closure f) Line 159
	at C:\Users\Peter\Code\rust\src\librustc\util\common.rs(159)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::driver::phase_2_configure_and_expand_inner(rustc::session::Session * sess, rustc_metadata::cstore::CStore * cstore, syntax::ast::Crate make_glob_map, core::option::Option<rustc_plugin::registry::Registry> resolver_arenas, str* crate_loader, core::option::Option<alloc::vec::Vec<alloc::string::String>> after_expand, rustc_resolve::MakeGlobMap) Line 958
	at C:\Users\Peter\Code\rust\src\librustc_driver\driver.rs(958)
rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::driver::phase_2_configure_and_expand<closure>(rustc::session::Session * sess, rustc_metadata::cstore::CStore * cstore, syntax::ast::Crate krate, core::option::Option<rustc_plugin::registry::Registry> registry, str* crate_name, core::option::Option<alloc::vec::Vec<alloc::string::String>> addl_plugins, rustc_resolve::MakeGlobMap make_glob_map, closure after_expand) Line 763
	at C:\Users\Peter\Code\rust\src\librustc_driver\driver.rs(763)
rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::driver::compile_input(rustc_codegen_utils::codegen_backend::Box<CodegenBackend> codegen_backend, rustc::session::Session * sess, rustc_metadata::cstore::CStore * cstore, core::option::Option<std::path::PathBuf> * input_path, rustc::session::config::Input * input, core::option::Option<std::path::PathBuf> * outdir, core::option::Option<std::path::PathBuf> * output, core::option::Option<alloc::vec::Vec<alloc::string::String>> addl_plugins, rustc_driver::driver::CompileController * control) Line 184
	at C:\Users\Peter\Code\rust\src\librustc_driver\driver.rs(184)
rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::run_compiler_with_pool(getopts::Matches matches, rustc::session::config::Options sopts, std::collections::hash::set::HashSet<(syntax_pos::symbol::Symbol, core::option::Option<syntax_pos::symbol::Symbol>), std::collections::hash::map::RandomState> cfg, rustc_driver::Box<CompilerCalls> callbacks, core::option::Option<alloc::boxed::Box<FileLoader>> file_loader, core::option::Option<alloc::boxed::Box<Write>> emitter_dest) Line 571
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(571)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::run_compiler::{{closure}}::{{closure}}(closure) Line 483
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(483)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::driver::spawn_thread_pool::{{closure}}(closure) Line 74
	at C:\Users\Peter\Code\rust\src\librustc_driver\driver.rs(74)
rustc_driver-295c99f0d90ee7b8.dll!scoped_tls::ScopedKey<rustc_data_structures::sync::Lock<usize>>::set<rustc_data_structures::sync::Lock<usize>,closure,(core::result::Result<(), rustc::session::CompileIncomplete>, core::option::Option<rustc::session::Session>)>(rustc_data_structures::sync::Lock<usize> * self, closure t) Line 156
	at C:\Users\Peter\AppData\Roaming\Rust\registry\src\github.com-1ecc6299db9ec823\scoped-tls-0.1.2\src\lib.rs(156)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::driver::spawn_thread_pool(rustc::session::config::Options) Line 73
	at C:\Users\Peter\Code\rust\src\librustc_driver\driver.rs(73)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::run_compiler::{{closure}}(closure) Line 482
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(482)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!scoped_tls::ScopedKey<syntax_pos::Globals>::set(syntax_pos::Globals * self, closure t) Line 155
	at C:\Users\Peter\AppData\Roaming\Rust\registry\src\github.com-1ecc6299db9ec823\scoped-tls-0.1.2\src\lib.rs(155)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!syntax::with_globals::{{closure}}(closure) Line 101
	at C:\Users\Peter\Code\rust\src\libsyntax\lib.rs(101)
rustc_driver-295c99f0d90ee7b8.dll!scoped_tls::ScopedKey<syntax::Globals>::set<syntax::Globals,closure,(core::result::Result<(), rustc::session::CompileIncomplete>, core::option::Option<rustc::session::Session>)>(syntax::Globals * self, closure t) Line 155
	at C:\Users\Peter\AppData\Roaming\Rust\registry\src\github.com-1ecc6299db9ec823\scoped-tls-0.1.2\src\lib.rs(155)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!syntax::with_globals(closure f) Line 100
	at C:\Users\Peter\Code\rust\src\libsyntax\lib.rs(100)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::run_compiler(slice<alloc::string::String>* callbacks, rustc_driver::Box<CompilerCalls> file_loader, core::option::Option<alloc::boxed::Box<FileLoader>> emitter_dest, core::option::Option<alloc::boxed::Box<Write>>) Line 473
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(473)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::main::{{closure}}(closure) Line 1742
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(1742)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::run::{{closure}}(closure) Line 187
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(187)
rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::monitor::{{closure}}<closure>(closure) Line 1657
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(1657)
std-d64fb11a1f4dd817.dll!panic_unwind::__rust_maybe_catch_panic(void(*)(unsigned char *) f, unsigned char * data, unsigned __int64 * data_ptr, unsigned __int64 * vtable_ptr) Line 104
	at C:\Users\Peter\Code\rust\src\libpanic_unwind\lib.rs(104)
rustc_driver-295c99f0d90ee7b8.dll!std::panicking::try<(),std::panic::AssertUnwindSafe<closure>>(std::panic::AssertUnwindSafe<closure>) Line 294
	at C:\Users\Peter\Code\rust\src\libstd\panicking.rs(294)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!std::panic::catch_unwind(std::panic::AssertUnwindSafe<closure>) Line 392
	at C:\Users\Peter\Code\rust\src\libstd\panic.rs(392)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::in_named_rustc_thread(alloc::string::String name, closure) Line 1571
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(1571)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::in_rustc_thread(closure) Line 1582
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(1582)
[Inline Frame] rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::monitor(closure) Line 1656
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(1656)
rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::run<closure>(closure) Line 186
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(186)
rustc_driver-295c99f0d90ee7b8.dll!rustc_driver::main() Line 1747
	at C:\Users\Peter\Code\rust\src\librustc_driver\lib.rs(1747)
rustc.exe!std::rt::lang_start::{{closure}}<()>(closure *) Line 74
	at C:\Users\Peter\Code\rust\src\libstd\rt.rs(74)
[Inline Frame] std-d64fb11a1f4dd817.dll!std::rt::lang_start_internal::{{closure}}(closure) Line 59
	at C:\Users\Peter\Code\rust\src\libstd\rt.rs(59)
std-d64fb11a1f4dd817.dll!std::panicking::try::do_call<closure,i32>(unsigned char * data) Line 310
	at C:\Users\Peter\Code\rust\src\libstd\panicking.rs(310)
std-d64fb11a1f4dd817.dll!panic_unwind::__rust_maybe_catch_panic(void(*)(unsigned char *) f, unsigned char * data, unsigned __int64 * data_ptr, unsigned __int64 * vtable_ptr) Line 104
	at C:\Users\Peter\Code\rust\src\libpanic_unwind\lib.rs(104)
std-d64fb11a1f4dd817.dll!std::panicking::try<i32,closure>(closure f) Line 295
	at C:\Users\Peter\Code\rust\src\libstd\panicking.rs(295)
[Inline Frame] std-d64fb11a1f4dd817.dll!std::panic::catch_unwind(closure f) Line 392
	at C:\Users\Peter\Code\rust\src\libstd\panic.rs(392)
std-d64fb11a1f4dd817.dll!std::rt::lang_start_internal(core::ops::function::Fn<()>* argc, __int64 argv, unsigned char * *) Line 64
	at C:\Users\Peter\Code\rust\src\libstd\rt.rs(64)
rustc.exe!main()
[Inline Frame] rustc.exe!invoke_main() Line 78
	at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl(78)
rustc.exe!__scrt_common_main_seh() Line 283
	at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl(283)
kernel32.dll!BaseThreadInitThunk()
ntdll.dll!RtlUserThreadStart()
