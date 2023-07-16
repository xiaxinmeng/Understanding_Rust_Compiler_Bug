plain
2020-02-09T09:01:18.2564665Z ========================== Starting Command Output ===========================
2020-02-09T09:01:18.2581761Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bbbe8dff-0e89-43e5-aaaf-dc339e876705.sh
2020-02-09T09:01:18.2736554Z 
2020-02-09T09:01:18.2791302Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T09:01:18.2795538Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68988/merge to s
2020-02-09T09:01:18.2796813Z Task         : Get sources
2020-02-09T09:01:18.2796839Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T09:01:18.2796863Z Version      : 1.0.0
2020-02-09T09:01:18.2796887Z Author       : Microsoft
---
2020-02-09T09:01:19.7547216Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T09:01:19.7627664Z ##[command]git config gc.auto 0
2020-02-09T09:01:19.7692275Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T09:01:19.7739639Z ##[command]git config --get-all http.proxy
2020-02-09T09:01:19.7849249Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68988/merge:refs/remotes/pull/68988/merge
---
2020-02-09T09:07:39.5334139Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-09T09:07:41.5555225Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-02-09T09:07:42.1928646Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-09T09:07:43.5231661Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-02-09T09:07:51.7382336Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::result::Result<(), rustc_errors::ErrorReported>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7384475Z      |
2020-02-09T09:07:51.7384475Z      |
2020-02-09T09:07:51.7385152Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7386009Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7388712Z      |              |
2020-02-09T09:07:51.7389689Z      |              `std::cell::RefCell<std::option::Option<(std::result::Result<(), rustc_errors::ErrorReported>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7390281Z      |
2020-02-09T09:07:51.7391390Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::result::Result<(), rustc_errors::ErrorReported>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7392194Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::result::Result<(), rustc_errors::ErrorReported>>`
2020-02-09T09:07:51.7392927Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::analysis<'tcx>>`
2020-02-09T09:07:51.7393436Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7393953Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7394451Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7395148Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7395658Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7396224Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7396453Z 
2020-02-09T09:07:51.7417465Z error[E0277]: `std::cell::RefCell<std::option::Option<(&rustc_hir::hir::Crate<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7420785Z      |
2020-02-09T09:07:51.7420785Z      |
2020-02-09T09:07:51.7421346Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7421997Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7422510Z      |              |
2020-02-09T09:07:51.7423060Z      |              `std::cell::RefCell<std::option::Option<(&rustc_hir::hir::Crate<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7423522Z      |
2020-02-09T09:07:51.7424164Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&rustc_hir::hir::Crate<'_>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7424723Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&rustc_hir::hir::Crate<'_>>`
2020-02-09T09:07:51.7425245Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::hir_crate<'tcx>>`
2020-02-09T09:07:51.7425765Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7426375Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7426959Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7427451Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7428099Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7428631Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7428864Z 
2020-02-09T09:07:51.7453787Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7454258Z      |
2020-02-09T09:07:51.7454258Z      |
2020-02-09T09:07:51.7454510Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7454926Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7455151Z      |              |
2020-02-09T09:07:51.7455495Z      |              `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7455708Z      |
2020-02-09T09:07:51.7456116Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7456459Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>>`
2020-02-09T09:07:51.7456750Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::native_libraries<'tcx>>`
2020-02-09T09:07:51.7457031Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7457284Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7457564Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7457810Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7458279Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7458757Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7458802Z 
2020-02-09T09:07:51.7487627Z error[E0277]: `std::cell::RefCell<std::option::Option<(&lint::LintLevelMap, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7488263Z      |
2020-02-09T09:07:51.7488263Z      |
2020-02-09T09:07:51.7488514Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7488888Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7489133Z      |              |
2020-02-09T09:07:51.7489438Z      |              `std::cell::RefCell<std::option::Option<(&lint::LintLevelMap, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7489641Z      |
2020-02-09T09:07:51.7490030Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&lint::LintLevelMap, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7490329Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&lint::LintLevelMap>`
2020-02-09T09:07:51.7490800Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::lint_levels<'tcx>>`
2020-02-09T09:07:51.7491088Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7491351Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7491649Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7491910Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7492182Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7492503Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7496441Z 
2020-02-09T09:07:51.7523466Z error[E0277]: `std::cell::RefCell<std::option::Option<(bool, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7523946Z      |
2020-02-09T09:07:51.7523946Z      |
2020-02-09T09:07:51.7524198Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7524569Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7524934Z      |              |
2020-02-09T09:07:51.7525270Z      |              `std::cell::RefCell<std::option::Option<(bool, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7525568Z      |
2020-02-09T09:07:51.7525936Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(bool, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7526219Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<bool>`
2020-02-09T09:07:51.7526513Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::is_panic_runtime<'tcx>>`
2020-02-09T09:07:51.7526784Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7527036Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7527295Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7527608Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7527860Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7528174Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7531808Z 
2020-02-09T09:07:51.7559551Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7560507Z      |
2020-02-09T09:07:51.7560507Z      |
2020-02-09T09:07:51.7560865Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7561431Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7561708Z      |              |
2020-02-09T09:07:51.7562085Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7562304Z      |
2020-02-09T09:07:51.7562880Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7563495Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:07:51.7563898Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::mir_keys<'tcx>>`
2020-02-09T09:07:51.7564186Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7564446Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7564733Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7564992Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7565245Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7565583Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7569432Z 
2020-02-09T09:07:51.7594076Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7594564Z      |
2020-02-09T09:07:51.7594564Z      |
2020-02-09T09:07:51.7594841Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7595216Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7595471Z      |              |
2020-02-09T09:07:51.7595867Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7596078Z      |
2020-02-09T09:07:51.7596538Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7596940Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:07:51.7597363Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::wasm_import_module_map<'tcx>>`
2020-02-09T09:07:51.7597685Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7597932Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7598302Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7598544Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7598785Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7599100Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7602737Z 
2020-02-09T09:07:51.7627810Z error[E0277]: `std::cell::RefCell<std::option::Option<(&ty::CrateVariancesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7628263Z      |
2020-02-09T09:07:51.7628263Z      |
2020-02-09T09:07:51.7628530Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7628909Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7629150Z      |              |
2020-02-09T09:07:51.7629463Z      |              `std::cell::RefCell<std::option::Option<(&ty::CrateVariancesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7629728Z      |
2020-02-09T09:07:51.7630113Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&ty::CrateVariancesMap<'_>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7630424Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&ty::CrateVariancesMap<'_>>`
2020-02-09T09:07:51.7630720Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_variances<'tcx>>`
2020-02-09T09:07:51.7630989Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7631247Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7631500Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7631772Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7632139Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7632500Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7636078Z 
2020-02-09T09:07:51.7662603Z error[E0277]: `std::cell::RefCell<std::option::Option<(&ty::CratePredicatesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7663236Z      |
2020-02-09T09:07:51.7663236Z      |
2020-02-09T09:07:51.7663503Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7663883Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7664117Z      |              |
2020-02-09T09:07:51.7664425Z      |              `std::cell::RefCell<std::option::Option<(&ty::CratePredicatesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7664618Z      |
2020-02-09T09:07:51.7665020Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&ty::CratePredicatesMap<'_>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7665310Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&ty::CratePredicatesMap<'_>>`
2020-02-09T09:07:51.7665626Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::inferred_outlives_crate<'tcx>>`
2020-02-09T09:07:51.7665884Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7666148Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7666404Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7666673Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7666921Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7667230Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7670826Z 
2020-02-09T09:07:51.7696675Z error[E0277]: `std::cell::RefCell<std::option::Option<((), dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7697310Z      |
2020-02-09T09:07:51.7697310Z      |
2020-02-09T09:07:51.7697670Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7699043Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7699694Z      |              |
2020-02-09T09:07:51.7700306Z      |              `std::cell::RefCell<std::option::Option<((), dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7700668Z      |
2020-02-09T09:07:51.7701054Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<((), dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7701325Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<()>`
2020-02-09T09:07:51.7701628Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::typeck_item_bodies<'tcx>>`
2020-02-09T09:07:51.7701887Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7702159Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7702408Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7702649Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7702915Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7703206Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7706847Z 
2020-02-09T09:07:51.7732848Z error[E0277]: `std::cell::RefCell<std::option::Option<(&ty::CrateInherentImpls, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7733311Z      |
2020-02-09T09:07:51.7733311Z      |
2020-02-09T09:07:51.7733556Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7733938Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7734182Z      |              |
2020-02-09T09:07:51.7734500Z      |              `std::cell::RefCell<std::option::Option<(&ty::CrateInherentImpls, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7734713Z      |
2020-02-09T09:07:51.7735211Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&ty::CrateInherentImpls, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7735558Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&ty::CrateInherentImpls>`
2020-02-09T09:07:51.7735849Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_inherent_impls<'tcx>>`
2020-02-09T09:07:51.7736218Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7736467Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7736741Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7736996Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7737259Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7737551Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7741196Z 
2020-02-09T09:07:51.7766421Z error[E0277]: `std::cell::RefCell<std::option::Option<(&middle::privacy::AccessLevels, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7766887Z      |
2020-02-09T09:07:51.7766887Z      |
2020-02-09T09:07:51.7767155Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7767531Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7767782Z      |              |
2020-02-09T09:07:51.7768094Z      |              `std::cell::RefCell<std::option::Option<(&middle::privacy::AccessLevels, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7768301Z      |
2020-02-09T09:07:51.7768689Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&middle::privacy::AccessLevels, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7768998Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&middle::privacy::AccessLevels>`
2020-02-09T09:07:51.7769302Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::privacy_access_levels<'tcx>>`
2020-02-09T09:07:51.7769576Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7769822Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7770222Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7770519Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7770782Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7771179Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7774749Z 
2020-02-09T09:07:51.7804012Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7804505Z      |
2020-02-09T09:07:51.7804505Z      |
2020-02-09T09:07:51.7804754Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7805122Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7805367Z      |              |
2020-02-09T09:07:51.7805758Z      |              `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7805976Z      |
2020-02-09T09:07:51.7806435Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7806836Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>>`
2020-02-09T09:07:51.7807152Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::reachable_set<'tcx>>`
2020-02-09T09:07:51.7807410Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7807673Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7807936Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7808176Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7808437Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7808851Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7812429Z 
2020-02-09T09:07:51.7841023Z error[E0277]: `std::cell::RefCell<std::option::Option<(&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7841665Z      |
2020-02-09T09:07:51.7841665Z      |
2020-02-09T09:07:51.7841917Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7842297Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7842545Z      |              |
2020-02-09T09:07:51.7842882Z      |              `std::cell::RefCell<std::option::Option<(&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7843100Z      |
2020-02-09T09:07:51.7843508Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)], dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7843856Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)]>`
2020-02-09T09:07:51.7844154Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::dylib_dependency_formats<'tcx>>`
2020-02-09T09:07:51.7844429Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7844673Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7844936Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7845185Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7845430Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7845741Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7849836Z 
2020-02-09T09:07:51.7876658Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7877107Z      |
2020-02-09T09:07:51.7877107Z      |
2020-02-09T09:07:51.7877515Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7877935Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7878326Z      |              |
2020-02-09T09:07:51.7878702Z      |              `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7878914Z      |
2020-02-09T09:07:51.7879362Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7879743Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>>`
2020-02-09T09:07:51.7880050Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::dependency_formats<'tcx>>`
2020-02-09T09:07:51.7880303Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7880555Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7880826Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7881077Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7881333Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7881628Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7885202Z 
2020-02-09T09:07:51.7909780Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_target::spec::PanicStrategy, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7910212Z      |
2020-02-09T09:07:51.7910212Z      |
2020-02-09T09:07:51.7910480Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7910862Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7911100Z      |              |
2020-02-09T09:07:51.7911540Z      |              `std::cell::RefCell<std::option::Option<(rustc_target::spec::PanicStrategy, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7911796Z      |
2020-02-09T09:07:51.7912187Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_target::spec::PanicStrategy, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7912603Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_target::spec::PanicStrategy>`
2020-02-09T09:07:51.7912891Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::panic_strategy<'tcx>>`
2020-02-09T09:07:51.7913165Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7913418Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7913671Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7913947Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7914188Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7914498Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7918130Z 
2020-02-09T09:07:51.7943324Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_session::config::SymbolManglingVersion, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7943774Z      |
2020-02-09T09:07:51.7943774Z      |
2020-02-09T09:07:51.7944230Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7944849Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7945108Z      |              |
2020-02-09T09:07:51.7945459Z      |              `std::cell::RefCell<std::option::Option<(rustc_session::config::SymbolManglingVersion, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7945677Z      |
2020-02-09T09:07:51.7946119Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_session::config::SymbolManglingVersion, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7946464Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_session::config::SymbolManglingVersion>`
2020-02-09T09:07:51.7946913Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::symbol_mangling_version<'tcx>>`
2020-02-09T09:07:51.7947429Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7947862Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7948236Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7948486Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7948729Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7949048Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7953239Z 
2020-02-09T09:07:51.7980434Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7980901Z      |
2020-02-09T09:07:51.7980901Z      |
2020-02-09T09:07:51.7981160Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.7981544Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7981788Z      |              |
2020-02-09T09:07:51.7982208Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.7982426Z      |
2020-02-09T09:07:51.7982911Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.7983327Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:07:51.7983655Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::reachable_non_generics<'tcx>>`
2020-02-09T09:07:51.7983910Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.7984290Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7984589Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.7984830Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7985204Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.7985496Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.7989072Z 
2020-02-09T09:07:51.8016431Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8016907Z      |
2020-02-09T09:07:51.8016907Z      |
2020-02-09T09:07:51.8017162Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8017559Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8017787Z      |              |
2020-02-09T09:07:51.8018517Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8018738Z      |
2020-02-09T09:07:51.8019334Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8019845Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:07:51.8020170Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::upstream_monomorphizations<'tcx>>`
2020-02-09T09:07:51.8020565Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8020864Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8021138Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8021484Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8021725Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8022038Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8025595Z 
2020-02-09T09:07:51.8049816Z error[E0277]: `std::cell::RefCell<std::option::Option<(&[middle::cstore::ForeignModule], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8050234Z      |
2020-02-09T09:07:51.8050234Z      |
2020-02-09T09:07:51.8050509Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8050879Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8051100Z      |              |
2020-02-09T09:07:51.8051444Z      |              `std::cell::RefCell<std::option::Option<(&[middle::cstore::ForeignModule], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8051634Z      |
2020-02-09T09:07:51.8052040Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&[middle::cstore::ForeignModule], dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8052337Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&[middle::cstore::ForeignModule]>`
2020-02-09T09:07:51.8052655Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::foreign_modules<'tcx>>`
2020-02-09T09:07:51.8052911Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8053179Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8053444Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8053708Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8053950Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8054376Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8058099Z 
2020-02-09T09:07:51.8083404Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8084022Z      |
2020-02-09T09:07:51.8084022Z      |
2020-02-09T09:07:51.8084287Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8084671Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8084904Z      |              |
2020-02-09T09:07:51.8085430Z      |              `std::cell::RefCell<std::option::Option<(std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8085661Z      |
2020-02-09T09:07:51.8086314Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8086709Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>>`
2020-02-09T09:07:51.8087036Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::entry_fn<'tcx>>`
2020-02-09T09:07:51.8087346Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8087623Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8087900Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8088196Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8088619Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8088938Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8092883Z 
2020-02-09T09:07:51.8120766Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_hir::def_id::DefId>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8121212Z      |
2020-02-09T09:07:51.8121212Z      |
2020-02-09T09:07:51.8121457Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8121973Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8122288Z      |              |
2020-02-09T09:07:51.8122630Z      |              `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_hir::def_id::DefId>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8122822Z      |
2020-02-09T09:07:51.8123245Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_hir::def_id::DefId>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8123547Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::option::Option<rustc_hir::def_id::DefId>>`
2020-02-09T09:07:51.8123860Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::plugin_registrar_fn<'tcx>>`
2020-02-09T09:07:51.8124113Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8124374Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8124639Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8124910Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8125155Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8125634Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8129186Z 
2020-02-09T09:07:51.8154822Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_session::session::CrateDisambiguator, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8155278Z      |
2020-02-09T09:07:51.8155278Z      |
2020-02-09T09:07:51.8155539Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8155936Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8156159Z      |              |
2020-02-09T09:07:51.8156497Z      |              `std::cell::RefCell<std::option::Option<(rustc_session::session::CrateDisambiguator, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8156682Z      |
2020-02-09T09:07:51.8157217Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_session::session::CrateDisambiguator, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8157559Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_session::session::CrateDisambiguator>`
2020-02-09T09:07:51.8157968Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_disambiguator<'tcx>>`
2020-02-09T09:07:51.8158223Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8158495Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8158757Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8159025Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8159281Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8159571Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8163100Z 
2020-02-09T09:07:51.8199518Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_data_structures::svh::Svh, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8200384Z      |
2020-02-09T09:07:51.8200384Z      |
2020-02-09T09:07:51.8200895Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8201702Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8201954Z      |              |
2020-02-09T09:07:51.8202580Z      |              `std::cell::RefCell<std::option::Option<(rustc_data_structures::svh::Svh, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8202966Z      |
2020-02-09T09:07:51.8203432Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_data_structures::svh::Svh, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8204012Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_data_structures::svh::Svh>`
2020-02-09T09:07:51.8204367Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_hash<'tcx>>`
2020-02-09T09:07:51.8204668Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8205284Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8205629Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8206431Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8207128Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8207870Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8213115Z 
2020-02-09T09:07:51.8246318Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_data_structures::svh::Svh>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8247588Z      |
2020-02-09T09:07:51.8247588Z      |
2020-02-09T09:07:51.8248222Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8248703Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8248959Z      |              |
2020-02-09T09:07:51.8249549Z      |              `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_data_structures::svh::Svh>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8250141Z      |
2020-02-09T09:07:51.8251744Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_data_structures::svh::Svh>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8252359Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::option::Option<rustc_data_structures::svh::Svh>>`
2020-02-09T09:07:51.8252912Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_host_hash<'tcx>>`
2020-02-09T09:07:51.8253228Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8253768Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8254252Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8254539Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8255187Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8256022Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8261573Z 
2020-02-09T09:07:51.8295263Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_span::symbol::Symbol, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8295923Z      |
2020-02-09T09:07:51.8295923Z      |
2020-02-09T09:07:51.8296193Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8296596Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8296858Z      |              |
2020-02-09T09:07:51.8297195Z      |              `std::cell::RefCell<std::option::Option<(rustc_span::symbol::Symbol, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8297433Z      |
2020-02-09T09:07:51.8297850Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_span::symbol::Symbol, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8298404Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_span::symbol::Symbol>`
2020-02-09T09:07:51.8298737Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::original_crate_name<'tcx>>`
2020-02-09T09:07:51.8299038Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8299304Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8299609Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8299879Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8300161Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8300482Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8306152Z 
2020-02-09T09:07:51.8337610Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::string::String, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8338327Z      |
2020-02-09T09:07:51.8338327Z      |
2020-02-09T09:07:51.8338605Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8339126Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8339428Z      |              |
2020-02-09T09:07:51.8339751Z      |              `std::cell::RefCell<std::option::Option<(std::string::String, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8340075Z      |
2020-02-09T09:07:51.8340475Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::string::String, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8340993Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::string::String>`
2020-02-09T09:07:51.8341308Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::extra_filename<'tcx>>`
2020-02-09T09:07:51.8341602Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:07:51.8341880Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8342359Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:07:51.8342639Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8342920Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:07:51.8343667Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8348195Z 
2020-02-09T09:07:51.8372276Z error[E0277]: `std::cell::RefCell<std::option::Option<(&[rustc_hir::def_id::DefId], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8372750Z      |
2020-02-09T09:07:51.8372750Z      |
2020-02-09T09:07:51.8373007Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:07:51.8373396Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:07:51.8373643Z      |              |
2020-02-09T09:07:51.8373960Z      |              `std::cell::RefCell<std::option::Option<(&[rustc_hir::def_id::DefId], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:07:51.8374180Z      |
2020-02-09T09:07:51.8374573Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&[rustc_hir::def_id::DefId], dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:07:51.8375015Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&[rustc_hir::def_id::DefId]>`
