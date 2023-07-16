plain
2020-01-05T16:19:29.1402092Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T16:19:29.1414127Z ##[command]git config gc.auto 0
2020-01-05T16:19:29.1416413Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T16:19:29.1420187Z ##[command]git config --get-all http.proxy
2020-01-05T16:19:29.1428623Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67870/merge:refs/remotes/pull/67870/merge
---
2020-01-05T16:31:51.9186378Z configure: build.locked-deps    := True
2020-01-05T16:31:51.9186510Z configure: llvm.ccache          := sccache
2020-01-05T16:31:51.9186788Z configure: build.cargo-native-static := True
2020-01-05T16:31:51.9187284Z configure: dist.missing-tools   := True
2020-01-05T16:31:51.9187566Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-05T16:31:51.9194881Z configure: writing `config.toml` in current directory
2020-01-05T16:31:51.9195217Z configure: 
2020-01-05T16:31:51.9195841Z configure: run `python /checkout/x.py --help`
2020-01-05T16:31:51.9196149Z configure: 
---
2020-01-05T16:36:56.6154679Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-01-05T16:36:58.4680463Z error[E0277]: `std::rc::Rc<rustc::lint::LintStore>` cannot be shared between threads safely
2020-01-05T16:36:58.4680877Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.4681168Z     |
2020-01-05T16:36:58.4682388Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.4682887Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<rustc::lint::LintStore>` cannot be shared between threads safely
2020-01-05T16:36:58.4683142Z     |
2020-01-05T16:36:58.4683553Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<rustc::lint::LintStore>`
2020-01-05T16:36:58.4684279Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.4684651Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.4684651Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.4685007Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.4685053Z 
2020-01-05T16:36:58.4875138Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc::ty::CReaderCacheKey, &rustc::ty::TyS<'_>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.4875841Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.4876216Z     |
2020-01-05T16:36:58.4876569Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.4877124Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc::ty::CReaderCacheKey, &rustc::ty::TyS<'_>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.4877387Z     |
2020-01-05T16:36:58.4877901Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc::ty::CReaderCacheKey, &rustc::ty::TyS<'_>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.4878418Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<rustc::ty::CReaderCacheKey, &rustc::ty::TyS<'_>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.4879166Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.4879526Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.4879526Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.4879914Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.4879962Z 
2020-01-05T16:36:58.4899327Z error[E0277]: `std::cell::RefCell<rustc::mir::interpret::AllocMap<'_>>` cannot be shared between threads safely
2020-01-05T16:36:58.4899662Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.4899894Z     |
2020-01-05T16:36:58.4900274Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.4900726Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<rustc::mir::interpret::AllocMap<'_>>` cannot be shared between threads safely
2020-01-05T16:36:58.4900995Z     |
2020-01-05T16:36:58.4901418Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<rustc::mir::interpret::AllocMap<'_>>`
2020-01-05T16:36:58.4901828Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<rustc::mir::interpret::AllocMap<'_>>`
2020-01-05T16:36:58.4902533Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.4903052Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.4903052Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.4903468Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.4903629Z 
2020-01-05T16:36:58.5033896Z error[E0277]: `(dyn rustc::middle::cstore::CrateStore + rustc_data_structures::sync::Sync + 'static)` cannot be shared between threads safely
2020-01-05T16:36:58.5034274Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5034531Z     |
2020-01-05T16:36:58.5034882Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5035376Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn rustc::middle::cstore::CrateStore + rustc_data_structures::sync::Sync + 'static)` cannot be shared between threads safely
2020-01-05T16:36:58.5035671Z     |
2020-01-05T16:36:58.5036068Z     = help: the trait `std::marker::Sync` is not implemented for `(dyn rustc::middle::cstore::CrateStore + rustc_data_structures::sync::Sync + 'static)`
2020-01-05T16:36:58.5036543Z     = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn rustc::middle::cstore::CrateStore + rustc_data_structures::sync::Sync + 'static)>`
2020-01-05T16:36:58.5036959Z     = note: required because it appears within the type `std::boxed::Box<(dyn rustc::middle::cstore::CrateStore + rustc_data_structures::sync::Sync + 'static)>`
2020-01-05T16:36:58.5037709Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5038091Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5038091Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5038455Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5038502Z 
2020-01-05T16:36:58.5066972Z error[E0277]: `std::rc::Rc<rustc::dep_graph::graph::DepGraphData>` cannot be shared between threads safely
2020-01-05T16:36:58.5067315Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5067545Z     |
2020-01-05T16:36:58.5067930Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5068372Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<rustc::dep_graph::graph::DepGraphData>` cannot be shared between threads safely
2020-01-05T16:36:58.5068652Z     |
2020-01-05T16:36:58.5069058Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<rustc::dep_graph::graph::DepGraphData>`
2020-01-05T16:36:58.5069448Z     = note: required because it appears within the type `std::option::Option<std::rc::Rc<rustc::dep_graph::graph::DepGraphData>>`
2020-01-05T16:36:58.5069992Z     = note: required because it appears within the type `rustc::dep_graph::DepGraph`
2020-01-05T16:36:58.5070891Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5071247Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5071247Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5071602Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5071667Z 
2020-01-05T16:36:58.5234935Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::TraitRef<'_>>, rustc::traits::select::WithDepNode<std::result::Result<std::option::Option<rustc::traits::select::SelectionCandidate<'_>>, rustc::traits::SelectionError<'_>>>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5235400Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5235645Z     |
2020-01-05T16:36:58.5236007Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5236727Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::TraitRef<'_>>, rustc::traits::select::WithDepNode<std::result::Result<std::option::Option<rustc::traits::select::SelectionCandidate<'_>>, rustc::traits::SelectionError<'_>>>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5237022Z     |
2020-01-05T16:36:58.5237696Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::TraitRef<'_>>, rustc::traits::select::WithDepNode<std::result::Result<std::option::Option<rustc::traits::select::SelectionCandidate<'_>>, rustc::traits::SelectionError<'_>>>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5238395Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::TraitRef<'_>>, rustc::traits::select::WithDepNode<std::result::Result<std::option::Option<rustc::traits::select::SelectionCandidate<'_>>, rustc::traits::SelectionError<'_>>>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5239146Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5239478Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5239820Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5239820Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5240159Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5240204Z 
2020-01-05T16:36:58.5247096Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::Binder<rustc::ty::TraitRef<'_>>>, rustc::traits::select::WithDepNode<rustc::traits::EvaluationResult>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5247563Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5247824Z     |
2020-01-05T16:36:58.5248174Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5248837Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::Binder<rustc::ty::TraitRef<'_>>>, rustc::traits::select::WithDepNode<rustc::traits::EvaluationResult>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5249154Z     |
2020-01-05T16:36:58.5249783Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::Binder<rustc::ty::TraitRef<'_>>>, rustc::traits::select::WithDepNode<rustc::traits::EvaluationResult>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5250433Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<rustc::ty::ParamEnvAnd<'_, rustc::ty::Binder<rustc::ty::TraitRef<'_>>>, rustc::traits::select::WithDepNode<rustc::traits::EvaluationResult>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5251216Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5251596Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5251942Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5251942Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5252300Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5252366Z 
2020-01-05T16:36:58.5332988Z error[E0277]: `std::rc::Rc<rustc_span::source_map::SourceMap>` cannot be shared between threads safely
2020-01-05T16:36:58.5333330Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5333608Z     |
2020-01-05T16:36:58.5333976Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5334431Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<rustc_span::source_map::SourceMap>` cannot be shared between threads safely
2020-01-05T16:36:58.5334710Z     |
2020-01-05T16:36:58.5335109Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<rustc_span::source_map::SourceMap>`
2020-01-05T16:36:58.5335479Z     = note: required because it appears within the type `syntax::sess::ParseSess`
2020-01-05T16:36:58.5336628Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5337159Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5337530Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5337879Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5337879Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5338212Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5338257Z 
2020-01-05T16:36:58.5514436Z error[E0277]: `std::cell::RefCell<std::collections::HashSet<(rustc::rustc_session::DiagnosticMessageId, std::option::Option<rustc_span::span_encoding::Span>, std::string::String), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5514843Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5515112Z     |
2020-01-05T16:36:58.5515471Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5516095Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashSet<(rustc::rustc_session::DiagnosticMessageId, std::option::Option<rustc_span::span_encoding::Span>, std::string::String), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5516364Z     |
2020-01-05T16:36:58.5516960Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashSet<(rustc::rustc_session::DiagnosticMessageId, std::option::Option<rustc_span::span_encoding::Span>, std::string::String), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5517539Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashSet<(rustc::rustc_session::DiagnosticMessageId, std::option::Option<rustc_span::span_encoding::Span>, std::string::String), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5518268Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5518638Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5518969Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5519306Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5519306Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5519715Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5519760Z 
2020-01-05T16:36:58.5528198Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc_span::span_encoding::Span, (std::string::String, rustc_span::span_encoding::Span), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5528662Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5528921Z     |
2020-01-05T16:36:58.5529735Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5531009Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc_span::span_encoding::Span, (std::string::String, rustc_span::span_encoding::Span), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5531312Z     |
2020-01-05T16:36:58.5531868Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc_span::span_encoding::Span, (std::string::String, rustc_span::span_encoding::Span), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5532776Z     = note: required because it appears within the type `rustc_data_structures::sync::OneThread<std::cell::RefCell<std::collections::HashMap<rustc_span::span_encoding::Span, (std::string::String, rustc_span::span_encoding::Span), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>`
2020-01-05T16:36:58.5533551Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5533923Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5534268Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5534590Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5534590Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5534943Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5534988Z 
2020-01-05T16:36:58.5713873Z error[E0277]: `std::cell::RefCell<rustc::rustc_session::IncrCompSession>` cannot be shared between threads safely
2020-01-05T16:36:58.5714274Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5714511Z     |
2020-01-05T16:36:58.5714889Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5715374Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<rustc::rustc_session::IncrCompSession>` cannot be shared between threads safely
2020-01-05T16:36:58.5715621Z     |
2020-01-05T16:36:58.5716046Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<rustc::rustc_session::IncrCompSession>`
2020-01-05T16:36:58.5716461Z     = note: required because it appears within the type `rustc_data_structures::sync::OneThread<std::cell::RefCell<rustc::rustc_session::IncrCompSession>>`
2020-01-05T16:36:58.5717456Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5717791Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5718183Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5718534Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5718534Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5718907Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5718954Z 
2020-01-05T16:36:58.5725157Z error[E0277]: `std::cell::RefCell<rustc::rustc_session::OptimizationFuel>` cannot be shared between threads safely
2020-01-05T16:36:58.5725509Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5725740Z     |
2020-01-05T16:36:58.5726085Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5726535Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<rustc::rustc_session::OptimizationFuel>` cannot be shared between threads safely
2020-01-05T16:36:58.5726781Z     |
2020-01-05T16:36:58.5727219Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<rustc::rustc_session::OptimizationFuel>`
2020-01-05T16:36:58.5727623Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<rustc::rustc_session::OptimizationFuel>`
2020-01-05T16:36:58.5728324Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5728647Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5729032Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5729390Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5729390Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5729747Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5781248Z error[E0277]: `std::cell::Cell<u64>` cannot be shared between threads safely
2020-01-05T16:36:58.5781568Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5781831Z     |
2020-01-05T16:36:58.5781831Z     |
2020-01-05T16:36:58.5782175Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5782571Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::Cell<u64>` cannot be shared between threads safely
2020-01-05T16:36:58.5782981Z     |
2020-01-05T16:36:58.5783406Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<u64>`
2020-01-05T16:36:58.5783898Z     = note: required because it appears within the type `rustc_data_structures::sync::Atomic<u64>`
2020-01-05T16:36:58.5784563Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5784906Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5785408Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5785795Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5785795Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5786190Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5786236Z 
2020-01-05T16:36:58.5956365Z error[E0277]: `std::cell::RefCell<std::collections::HashSet<rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5956751Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5956985Z     |
2020-01-05T16:36:58.5957566Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5958093Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashSet<rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5958384Z     |
2020-01-05T16:36:58.5958868Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashSet<rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5959369Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashSet<rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5960087Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5960421Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5960792Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5961162Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5961162Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5961522Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5961740Z 
2020-01-05T16:36:58.5962290Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc_span::span_encoding::Span, rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5962739Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.5963007Z     |
2020-01-05T16:36:58.5963374Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.5963945Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc_span::span_encoding::Span, rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.5964252Z     |
2020-01-05T16:36:58.5964791Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc_span::span_encoding::Span, rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5965351Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<rustc_span::span_encoding::Span, rustc_span::span_encoding::Span, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.5966106Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.5966459Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5966837Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.5967183Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5967183Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.5967540Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.5967603Z 
2020-01-05T16:36:58.6008914Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc::dep_graph::DepNodeIndex, std::vec::Vec<syntax::rustc_errors::Diagnostic>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6009256Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6009523Z     |
2020-01-05T16:36:58.6009874Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6010462Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc::dep_graph::DepNodeIndex, std::vec::Vec<syntax::rustc_errors::Diagnostic>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6010722Z     |
2020-01-05T16:36:58.6011440Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc::dep_graph::DepNodeIndex, std::vec::Vec<syntax::rustc_errors::Diagnostic>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6012093Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<rustc::dep_graph::DepNodeIndex, std::vec::Vec<syntax::rustc_errors::Diagnostic>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6012478Z     = note: required because it appears within the type `rustc::ty::query::OnDiskCache<'_>`
2020-01-05T16:36:58.6012816Z     = note: required because it appears within the type `rustc::ty::query::Queries<'_>`
2020-01-05T16:36:58.6013502Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6013835Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6013835Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6014188Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6014233Z 
2020-01-05T16:36:58.6061080Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc::ty::query::on_disk_cache::SourceFileIndex, std::rc::Rc<rustc_span::SourceFile>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6061453Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6061685Z     |
2020-01-05T16:36:58.6062048Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6062651Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc::ty::query::on_disk_cache::SourceFileIndex, std::rc::Rc<rustc_span::SourceFile>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6062912Z     |
2020-01-05T16:36:58.6063480Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc::ty::query::on_disk_cache::SourceFileIndex, std::rc::Rc<rustc_span::SourceFile>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6064023Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<rustc::ty::query::on_disk_cache::SourceFileIndex, std::rc::Rc<rustc_span::SourceFile>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6064415Z     = note: required because it appears within the type `rustc::ty::query::OnDiskCache<'_>`
2020-01-05T16:36:58.6064770Z     = note: required because it appears within the type `rustc::ty::query::Queries<'_>`
2020-01-05T16:36:58.6065815Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6066315Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6066315Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6066768Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6066813Z 
2020-01-05T16:36:58.6113014Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<rustc::ty::query::on_disk_cache::AbsoluteBytePos, rustc_span::hygiene::SyntaxContext, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6113364Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6113595Z     |
2020-01-05T16:36:58.6113974Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6114610Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<rustc::ty::query::on_disk_cache::AbsoluteBytePos, rustc_span::hygiene::SyntaxContext, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6114929Z     |
2020-01-05T16:36:58.6115502Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<rustc::ty::query::on_disk_cache::AbsoluteBytePos, rustc_span::hygiene::SyntaxContext, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6116080Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<rustc::ty::query::on_disk_cache::AbsoluteBytePos, rustc_span::hygiene::SyntaxContext, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6116474Z     = note: required because it appears within the type `rustc::ty::query::OnDiskCache<'_>`
2020-01-05T16:36:58.6116860Z     = note: required because it appears within the type `rustc::ty::query::Queries<'_>`
2020-01-05T16:36:58.6117584Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6117930Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6117930Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6118358Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6118422Z 
2020-01-05T16:36:58.6169192Z error[E0277]: `std::cell::RefCell<std::vec::Vec<rustc_span::span_encoding::Span>>` cannot be shared between threads safely
2020-01-05T16:36:58.6169529Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6169783Z     |
2020-01-05T16:36:58.6170131Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6170596Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::vec::Vec<rustc_span::span_encoding::Span>>` cannot be shared between threads safely
2020-01-05T16:36:58.6170831Z     |
2020-01-05T16:36:58.6171429Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::vec::Vec<rustc_span::span_encoding::Span>>`
2020-01-05T16:36:58.6171996Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::vec::Vec<rustc_span::span_encoding::Span>>`
2020-01-05T16:36:58.6172332Z     = note: required because it appears within the type `syntax::sess::ParseSess`
2020-01-05T16:36:58.6173011Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6173346Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6173692Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6174026Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6174026Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6174358Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6174421Z 
2020-01-05T16:36:58.6222666Z error[E0277]: `std::cell::RefCell<std::vec::Vec<std::path::PathBuf>>` cannot be shared between threads safely
2020-01-05T16:36:58.6223006Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6223270Z     |
2020-01-05T16:36:58.6223638Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6224136Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::vec::Vec<std::path::PathBuf>>` cannot be shared between threads safely
2020-01-05T16:36:58.6224424Z     |
2020-01-05T16:36:58.6224849Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::vec::Vec<std::path::PathBuf>>`
2020-01-05T16:36:58.6225403Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::vec::Vec<std::path::PathBuf>>`
2020-01-05T16:36:58.6225811Z     = note: required because it appears within the type `syntax::sess::ParseSess`
2020-01-05T16:36:58.6226571Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6226921Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6227290Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6227640Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6227640Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6228159Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6228241Z 
2020-01-05T16:36:58.6277260Z error[E0277]: `std::cell::RefCell<std::vec::Vec<syntax::early_buffered_lints::BufferedEarlyLint>>` cannot be shared between threads safely
2020-01-05T16:36:58.6277790Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6278056Z     |
2020-01-05T16:36:58.6278407Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6278871Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::vec::Vec<syntax::early_buffered_lints::BufferedEarlyLint>>` cannot be shared between threads safely
2020-01-05T16:36:58.6279132Z     |
2020-01-05T16:36:58.6279568Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::vec::Vec<syntax::early_buffered_lints::BufferedEarlyLint>>`
2020-01-05T16:36:58.6280008Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::vec::Vec<syntax::early_buffered_lints::BufferedEarlyLint>>`
2020-01-05T16:36:58.6280340Z     = note: required because it appears within the type `syntax::sess::ParseSess`
2020-01-05T16:36:58.6281073Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6281420Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6281813Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6282178Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6282178Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6282536Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6282600Z 
2020-01-05T16:36:58.6332344Z error[E0277]: `std::cell::RefCell<bool>` cannot be shared between threads safely
2020-01-05T16:36:58.6332672Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6332930Z     |
2020-01-05T16:36:58.6333298Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6333703Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<bool>` cannot be shared between threads safely
2020-01-05T16:36:58.6333968Z     |
2020-01-05T16:36:58.6334339Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<bool>`
2020-01-05T16:36:58.6334702Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<bool>`
2020-01-05T16:36:58.6335031Z     = note: required because it appears within the type `syntax::sess::ParseSess`
2020-01-05T16:36:58.6335930Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6336381Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6336753Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6337123Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6337123Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6337481Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6337529Z 
2020-01-05T16:36:58.6385564Z error[E0277]: `std::cell::RefCell<std::option::Option<std::vec::Vec<rustc::rustc_session::config::CrateType>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6385932Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6386187Z     |
2020-01-05T16:36:58.6386540Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6387013Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::option::Option<std::vec::Vec<rustc::rustc_session::config::CrateType>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6387272Z     |
2020-01-05T16:36:58.6387727Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<std::vec::Vec<rustc::rustc_session::config::CrateType>>>`
2020-01-05T16:36:58.6388172Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::option::Option<std::vec::Vec<rustc::rustc_session::config::CrateType>>>`
2020-01-05T16:36:58.6388572Z     = note: required because it appears within the type `rustc_data_structures::sync::Once<std::vec::Vec<rustc::rustc_session::config::CrateType>>`
2020-01-05T16:36:58.6389261Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6389597Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6389990Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6390351Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6390351Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6390711Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6390776Z 
2020-01-05T16:36:58.6441162Z error[E0277]: `std::cell::RefCell<std::option::Option<rustc::rustc_session::CrateDisambiguator>>` cannot be shared between threads safely
2020-01-05T16:36:58.6441500Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6441763Z     |
2020-01-05T16:36:58.6442291Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6442933Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::option::Option<rustc::rustc_session::CrateDisambiguator>>` cannot be shared between threads safely
2020-01-05T16:36:58.6443199Z     |
2020-01-05T16:36:58.6443629Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<rustc::rustc_session::CrateDisambiguator>>`
2020-01-05T16:36:58.6444059Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::option::Option<rustc::rustc_session::CrateDisambiguator>>`
2020-01-05T16:36:58.6444453Z     = note: required because it appears within the type `rustc_data_structures::sync::Once<rustc::rustc_session::CrateDisambiguator>`
2020-01-05T16:36:58.6445147Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6445477Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6445820Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6446145Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6446145Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6446486Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6446556Z 
2020-01-05T16:36:58.6495831Z error[E0277]: `std::cell::RefCell<std::option::Option<rustc_feature::active::Features>>` cannot be shared between threads safely
2020-01-05T16:36:58.6496167Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6496431Z     |
2020-01-05T16:36:58.6496779Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6497247Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::option::Option<rustc_feature::active::Features>>` cannot be shared between threads safely
2020-01-05T16:36:58.6497514Z     |
2020-01-05T16:36:58.6497933Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<rustc_feature::active::Features>>`
2020-01-05T16:36:58.6498365Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::option::Option<rustc_feature::active::Features>>`
2020-01-05T16:36:58.6498743Z     = note: required because it appears within the type `rustc_data_structures::sync::Once<rustc_feature::active::Features>`
2020-01-05T16:36:58.6499577Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6499957Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6500424Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6500749Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6500749Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6501083Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6501146Z 
2020-01-05T16:36:58.6551119Z error[E0277]: `std::cell::RefCell<std::option::Option<usize>>` cannot be shared between threads safely
2020-01-05T16:36:58.6551459Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6551720Z     |
2020-01-05T16:36:58.6552066Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6552501Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::option::Option<usize>>` cannot be shared between threads safely
2020-01-05T16:36:58.6552759Z     |
2020-01-05T16:36:58.6553145Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<usize>>`
2020-01-05T16:36:58.6554042Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::option::Option<usize>>`
2020-01-05T16:36:58.6554560Z     = note: required because it appears within the type `rustc_data_structures::sync::Once<usize>`
2020-01-05T16:36:58.6555203Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6555505Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6555824Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6556122Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6556122Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6556436Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6604983Z error[E0277]: `std::cell::RefCell<std::time::Duration>` cannot be shared between threads safely
2020-01-05T16:36:58.6605456Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6605693Z     |
2020-01-05T16:36:58.6605693Z     |
2020-01-05T16:36:58.6606024Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6606412Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::time::Duration>` cannot be shared between threads safely
2020-01-05T16:36:58.6606652Z     |
2020-01-05T16:36:58.6607533Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::time::Duration>`
2020-01-05T16:36:58.6608040Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::time::Duration>`
2020-01-05T16:36:58.6608384Z     = note: required because it appears within the type `rustc::rustc_session::PerfStats`
2020-01-05T16:36:58.6609065Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6609402Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6609731Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6610081Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6610081Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6610413Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6664686Z error[E0277]: `std::cell::Cell<usize>` cannot be shared between threads safely
2020-01-05T16:36:58.6665010Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6665238Z     |
2020-01-05T16:36:58.6665238Z     |
2020-01-05T16:36:58.6665817Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6666221Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::Cell<usize>` cannot be shared between threads safely
2020-01-05T16:36:58.6666483Z     |
2020-01-05T16:36:58.6666853Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<usize>`
2020-01-05T16:36:58.6667200Z     = note: required because it appears within the type `rustc_data_structures::sync::Atomic<usize>`
2020-01-05T16:36:58.6667555Z     = note: required because it appears within the type `rustc::rustc_session::PerfStats`
2020-01-05T16:36:58.6668240Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6668578Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6668902Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6669237Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6669237Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6669570Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6669615Z 
2020-01-05T16:36:58.6721839Z error[E0277]: `std::cell::RefCell<std::collections::HashSet<rustc::rustc_session::code_stats::TypeSizeInfo, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6722290Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6722521Z     |
2020-01-05T16:36:58.6722892Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6723424Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashSet<rustc::rustc_session::code_stats::TypeSizeInfo, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6723698Z     |
2020-01-05T16:36:58.6724208Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashSet<rustc::rustc_session::code_stats::TypeSizeInfo, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6724724Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashSet<rustc::rustc_session::code_stats::TypeSizeInfo, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6725088Z     = note: required because it appears within the type `rustc::rustc_session::code_stats::CodeStats`
2020-01-05T16:36:58.6725774Z     = note: required because it appears within the type `&rustc::rustc_session::Session`
2020-01-05T16:36:58.6726101Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6726457Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6726776Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6726776Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6727120Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6779278Z error[E0277]: `rustc_span::span_encoding::Span` cannot be shared between threads safely
2020-01-05T16:36:58.6780372Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6780635Z     |
2020-01-05T16:36:58.6780635Z     |
2020-01-05T16:36:58.6780986Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6781666Z     |
2020-01-05T16:36:58.6781666Z     |
2020-01-05T16:36:58.6782041Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `rustc_span::span_encoding::Span`
2020-01-05T16:36:58.6782389Z     = note: required because it appears within the type `rustc::hir::Crate<'_>`
2020-01-05T16:36:58.6782871Z     = note: required because it appears within the type `rustc::hir::map::Forest<'_>`
2020-01-05T16:36:58.6783274Z     = note: required because it appears within the type `&rustc::hir::map::Forest<'_>`
2020-01-05T16:36:58.6784050Z     = note: required because it appears within the type `rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6784394Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6784716Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6784716Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6785054Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6785120Z 
2020-01-05T16:36:58.6834677Z error[E0277]: `std::cell::RefCell<std::option::Option<rustc_index::vec::IndexVec<rustc::hir::def_id::CrateNum, std::option::Option<rustc::hir::def_id::CrateNum>>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6835059Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6835317Z     |
2020-01-05T16:36:58.6835661Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6836218Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::option::Option<rustc_index::vec::IndexVec<rustc::hir::def_id::CrateNum, std::option::Option<rustc::hir::def_id::CrateNum>>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6836632Z     |
2020-01-05T16:36:58.6837311Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<rustc_index::vec::IndexVec<rustc::hir::def_id::CrateNum, std::option::Option<rustc::hir::def_id::CrateNum>>>>`
2020-01-05T16:36:58.6837808Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::option::Option<rustc_index::vec::IndexVec<rustc::hir::def_id::CrateNum, std::option::Option<rustc::hir::def_id::CrateNum>>>>`
2020-01-05T16:36:58.6838290Z     = note: required because it appears within the type `rustc_data_structures::sync::Once<rustc_index::vec::IndexVec<rustc::hir::def_id::CrateNum, std::option::Option<rustc::hir::def_id::CrateNum>>>`
2020-01-05T16:36:58.6838645Z     = note: required because it appears within the type `rustc::ty::query::OnDiskCache<'_>`
2020-01-05T16:36:58.6839004Z     = note: required because it appears within the type `rustc::ty::query::Queries<'_>`
2020-01-05T16:36:58.6839657Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6839998Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6839998Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6840333Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6840378Z 
2020-01-05T16:36:58.6892546Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<&syntax::attr::Stability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6893031Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6893264Z     |
2020-01-05T16:36:58.6893633Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6894145Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<&syntax::attr::Stability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6894414Z     |
2020-01-05T16:36:58.6894902Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<&syntax::attr::Stability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6895550Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<&syntax::attr::Stability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6896229Z     = note: required because it appears within the type `rustc_data_structures::sharded::CacheAligned<rustc_data_structures::sync::Lock<std::collections::HashMap<&syntax::attr::Stability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>`
2020-01-05T16:36:58.6896776Z     = note: required because it appears within the type `[rustc_data_structures::sharded::CacheAligned<rustc_data_structures::sync::Lock<std::collections::HashMap<&syntax::attr::Stability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>; 1]`
2020-01-05T16:36:58.6897256Z     = note: required because it appears within the type `rustc_data_structures::sharded::Sharded<std::collections::HashMap<&syntax::attr::Stability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6898267Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6898612Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6898612Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6898954Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6899005Z 
2020-01-05T16:36:58.6951637Z error[E0277]: `std::cell::RefCell<std::collections::HashMap<&syntax::attr::ConstStability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6952000Z    --> src/librustc_typeck/check/mod.rs:732:15
2020-01-05T16:36:58.6952229Z     |
2020-01-05T16:36:58.6952602Z 732 |     tcx.hir().par_visit_item_likes_in_module(module_def_id, &CheckItemTypesVisitor { tcx });
2020-01-05T16:36:58.6953281Z     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::RefCell<std::collections::HashMap<&syntax::attr::ConstStability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>` cannot be shared between threads safely
2020-01-05T16:36:58.6953694Z     |
2020-01-05T16:36:58.6954191Z     = help: within `check::CheckItemTypesVisitor<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::collections::HashMap<&syntax::attr::ConstStability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6954693Z     = note: required because it appears within the type `rustc_data_structures::sync::Lock<std::collections::HashMap<&syntax::attr::ConstStability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6955212Z     = note: required because it appears within the type `rustc_data_structures::sharded::CacheAligned<rustc_data_structures::sync::Lock<std::collections::HashMap<&syntax::attr::ConstStability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>`
2020-01-05T16:36:58.6955765Z     = note: required because it appears within the type `[rustc_data_structures::sharded::CacheAligned<rustc_data_structures::sync::Lock<std::collections::HashMap<&syntax::attr::ConstStability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>; 1]`
2020-01-05T16:36:58.6956273Z     = note: required because it appears within the type `rustc_data_structures::sharded::Sharded<std::collections::HashMap<&syntax::attr::ConstStability, (), std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>`
2020-01-05T16:36:58.6956997Z     = note: required because it appears within the type `&rustc::ty::GlobalCtxt<'_>`
2020-01-05T16:36:58.6957372Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6957372Z     = note: required because it appears within the type `rustc::ty::TyCtxt<'_>`
2020-01-05T16:36:58.6957739Z     = note: required because it appears within the type `check::CheckItemTypesVisitor<'_>`
2020-01-05T16:36:58.6957787Z 
