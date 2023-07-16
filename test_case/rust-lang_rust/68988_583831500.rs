plain
2020-02-09T09:41:53.4162781Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-02-09T09:42:06.3318237Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-02-09T09:42:13.3145261Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-02-09T09:42:19.1772720Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-09T09:42:29.9947734Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::result::Result<(), rustc_errors::ErrorReported>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:29.9948363Z      |
2020-02-09T09:42:29.9948363Z      |
2020-02-09T09:42:29.9948708Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:29.9949199Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:29.9949568Z      |              |
2020-02-09T09:42:29.9950028Z      |              `std::cell::RefCell<std::option::Option<(std::result::Result<(), rustc_errors::ErrorReported>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:29.9950323Z      |
2020-02-09T09:42:29.9950845Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::result::Result<(), rustc_errors::ErrorReported>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:29.9951325Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::result::Result<(), rustc_errors::ErrorReported>>`
2020-02-09T09:42:29.9951737Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::analysis<'tcx>>`
2020-02-09T09:42:29.9952101Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:29.9952458Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:29.9952828Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:29.9953165Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:29.9953515Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:29.9954105Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:29.9954209Z 
2020-02-09T09:42:30.0006493Z error[E0277]: `std::cell::RefCell<std::option::Option<(&rustc_hir::hir::Crate<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0007526Z      |
2020-02-09T09:42:30.0007526Z      |
2020-02-09T09:42:30.0008267Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0008781Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0009145Z      |              |
2020-02-09T09:42:30.0009588Z      |              `std::cell::RefCell<std::option::Option<(&rustc_hir::hir::Crate<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0009889Z      |
2020-02-09T09:42:30.0010389Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&rustc_hir::hir::Crate<'_>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0010834Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&rustc_hir::hir::Crate<'_>>`
2020-02-09T09:42:30.0011264Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::hir_crate<'tcx>>`
2020-02-09T09:42:30.0011643Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0012003Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0012378Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0012741Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0013082Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0013676Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0013763Z 
2020-02-09T09:42:30.0065640Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0066255Z      |
2020-02-09T09:42:30.0066255Z      |
2020-02-09T09:42:30.0066584Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0067228Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0067616Z      |              |
2020-02-09T09:42:30.0068233Z      |              `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0068606Z      |
2020-02-09T09:42:30.0069136Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0069602Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::sync::Arc<std::vec::Vec<middle::cstore::NativeLibrary>>>`
2020-02-09T09:42:30.0070042Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::native_libraries<'tcx>>`
2020-02-09T09:42:30.0070416Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0070769Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0071128Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0071462Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0071818Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0072220Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0072327Z 
2020-02-09T09:42:30.0224353Z error[E0277]: `std::cell::RefCell<std::option::Option<(&lint::LintLevelMap, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0224953Z      |
2020-02-09T09:42:30.0224953Z      |
2020-02-09T09:42:30.0225399Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0226879Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0227488Z      |              |
2020-02-09T09:42:30.0247669Z      |              `std::cell::RefCell<std::option::Option<(&lint::LintLevelMap, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0247963Z      |
2020-02-09T09:42:30.0248633Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&lint::LintLevelMap, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0249121Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&lint::LintLevelMap>`
2020-02-09T09:42:30.0249527Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::lint_levels<'tcx>>`
2020-02-09T09:42:30.0250039Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0250367Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0250713Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0251050Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0251369Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0251762Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0251853Z 
2020-02-09T09:42:30.0252254Z error[E0277]: `std::cell::RefCell<std::option::Option<(bool, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0252756Z      |
2020-02-09T09:42:30.0252756Z      |
2020-02-09T09:42:30.0253066Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0253556Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0253871Z      |              |
2020-02-09T09:42:30.0254244Z      |              `std::cell::RefCell<std::option::Option<(bool, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0254496Z      |
2020-02-09T09:42:30.0254948Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(bool, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0255326Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<bool>`
2020-02-09T09:42:30.0255707Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::is_panic_runtime<'tcx>>`
2020-02-09T09:42:30.0256063Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0256384Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0256718Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0257131Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0257659Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0258176Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0258259Z 
2020-02-09T09:42:30.0258696Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0259225Z      |
2020-02-09T09:42:30.0259225Z      |
2020-02-09T09:42:30.0259546Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0260025Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0260360Z      |              |
2020-02-09T09:42:30.0260834Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0261116Z      |
2020-02-09T09:42:30.0261678Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0262204Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashSet<rustc_hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:42:30.0262606Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::mir_keys<'tcx>>`
2020-02-09T09:42:30.0262953Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0263277Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0263612Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0263947Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0264262Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0264645Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0264725Z 
2020-02-09T09:42:30.0411636Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0412365Z      |
2020-02-09T09:42:30.0412365Z      |
2020-02-09T09:42:30.0412694Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0413365Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0413720Z      |              |
2020-02-09T09:42:30.0414272Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0414595Z      |
2020-02-09T09:42:30.0415213Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0415808Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashMap<rustc_hir::def_id::DefId, std::string::String, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:42:30.0416271Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::wasm_import_module_map<'tcx>>`
2020-02-09T09:42:30.0416649Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0417004Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0417488Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0417873Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0418219Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0418805Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0418900Z 
2020-02-09T09:42:30.0543851Z error[E0277]: `std::cell::RefCell<std::option::Option<(&ty::CrateVariancesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0544626Z      |
2020-02-09T09:42:30.0544626Z      |
2020-02-09T09:42:30.0545775Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0546401Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0546869Z      |              |
2020-02-09T09:42:30.0547303Z      |              `std::cell::RefCell<std::option::Option<(&ty::CrateVariancesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0547590Z      |
2020-02-09T09:42:30.0548092Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&ty::CrateVariancesMap<'_>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0548526Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&ty::CrateVariancesMap<'_>>`
2020-02-09T09:42:30.0548944Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_variances<'tcx>>`
2020-02-09T09:42:30.0549308Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0549643Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0550010Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0550356Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0550870Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0551277Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0551364Z 
2020-02-09T09:42:30.0551779Z error[E0277]: `std::cell::RefCell<std::option::Option<(&ty::CratePredicatesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0552290Z      |
2020-02-09T09:42:30.0552290Z      |
2020-02-09T09:42:30.0552609Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0553097Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0553427Z      |              |
2020-02-09T09:42:30.0553834Z      |              `std::cell::RefCell<std::option::Option<(&ty::CratePredicatesMap<'_>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0554099Z      |
2020-02-09T09:42:30.0554664Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&ty::CratePredicatesMap<'_>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0555161Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&ty::CratePredicatesMap<'_>>`
2020-02-09T09:42:30.0555561Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::inferred_outlives_crate<'tcx>>`
2020-02-09T09:42:30.0555901Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0556257Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0556632Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0556988Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0557501Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0557886Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0557967Z 
2020-02-09T09:42:30.0698567Z error[E0277]: `std::cell::RefCell<std::option::Option<((), dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0699626Z      |
2020-02-09T09:42:30.0699626Z      |
2020-02-09T09:42:30.0700402Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0701128Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0701527Z      |              |
2020-02-09T09:42:30.0701973Z      |              `std::cell::RefCell<std::option::Option<((), dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0702432Z      |
2020-02-09T09:42:30.0702942Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<((), dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0703370Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<()>`
2020-02-09T09:42:30.0703786Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::typeck_item_bodies<'tcx>>`
2020-02-09T09:42:30.0704145Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0704635Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0705059Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0705422Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0706079Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0706503Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0706614Z 
2020-02-09T09:42:30.0707218Z error[E0277]: `std::cell::RefCell<std::option::Option<(&ty::CrateInherentImpls, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0707762Z      |
2020-02-09T09:42:30.0707762Z      |
2020-02-09T09:42:30.0708244Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0708922Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0709313Z      |              |
2020-02-09T09:42:30.0709730Z      |              `std::cell::RefCell<std::option::Option<(&ty::CrateInherentImpls, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0710019Z      |
2020-02-09T09:42:30.0710532Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&ty::CrateInherentImpls, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0710962Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&ty::CrateInherentImpls>`
2020-02-09T09:42:30.0711384Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_inherent_impls<'tcx>>`
2020-02-09T09:42:30.0711752Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0712112Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0712467Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0712836Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0713183Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0713587Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0713682Z 
2020-02-09T09:42:30.0865423Z error[E0277]: `std::cell::RefCell<std::option::Option<(&middle::privacy::AccessLevels, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0867700Z      |
2020-02-09T09:42:30.0867700Z      |
2020-02-09T09:42:30.0868006Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0868670Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0869033Z      |              |
2020-02-09T09:42:30.0869464Z      |              `std::cell::RefCell<std::option::Option<(&middle::privacy::AccessLevels, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0869768Z      |
2020-02-09T09:42:30.0870291Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&middle::privacy::AccessLevels, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0870746Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&middle::privacy::AccessLevels>`
2020-02-09T09:42:30.0871164Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::privacy_access_levels<'tcx>>`
2020-02-09T09:42:30.0871545Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0871903Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0872256Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0872627Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0872979Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0873387Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0873491Z 
2020-02-09T09:42:30.0873962Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0874560Z      |
2020-02-09T09:42:30.0874560Z      |
2020-02-09T09:42:30.0874882Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.0875413Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0875840Z      |              |
2020-02-09T09:42:30.0876392Z      |              `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.0876801Z      |
2020-02-09T09:42:30.0877432Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.0878019Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::sync::Arc<std::collections::HashSet<rustc_hir::hir_id::HirId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>>`
2020-02-09T09:42:30.0878465Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::reachable_set<'tcx>>`
2020-02-09T09:42:30.0879013Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.0879367Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0879718Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.0880055Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0880403Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.0880809Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.0880947Z 
2020-02-09T09:42:30.1062734Z error[E0277]: `std::cell::RefCell<std::option::Option<(&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1063343Z      |
2020-02-09T09:42:30.1063343Z      |
2020-02-09T09:42:30.1063685Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1064195Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1064582Z      |              |
2020-02-09T09:42:30.1065074Z      |              `std::cell::RefCell<std::option::Option<(&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1065379Z      |
2020-02-09T09:42:30.1066253Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)], dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1066951Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&[(rustc_hir::def_id::CrateNum, middle::cstore::LinkagePreference)]>`
2020-02-09T09:42:30.1067573Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::dylib_dependency_formats<'tcx>>`
2020-02-09T09:42:30.1068121Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1068496Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1068891Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1069398Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1069763Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1070357Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1071674Z 
2020-02-09T09:42:30.1076730Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1077435Z      |
2020-02-09T09:42:30.1077435Z      |
2020-02-09T09:42:30.1077898Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1078600Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1078943Z      |              |
2020-02-09T09:42:30.1079459Z      |              `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1079779Z      |
2020-02-09T09:42:30.1080551Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1081090Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::sync::Arc<std::vec::Vec<(rustc_session::config::CrateType, std::vec::Vec<middle::dependency_format::Linkage>)>>>`
2020-02-09T09:42:30.1081838Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::dependency_formats<'tcx>>`
2020-02-09T09:42:30.1082257Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1082746Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1083130Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1083492Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1083836Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1084268Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1084357Z 
2020-02-09T09:42:30.1225469Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_target::spec::PanicStrategy, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1226137Z      |
2020-02-09T09:42:30.1226137Z      |
2020-02-09T09:42:30.1226461Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1226963Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1227299Z      |              |
2020-02-09T09:42:30.1227725Z      |              `std::cell::RefCell<std::option::Option<(rustc_target::spec::PanicStrategy, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1228012Z      |
2020-02-09T09:42:30.1228506Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_target::spec::PanicStrategy, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1228952Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_target::spec::PanicStrategy>`
2020-02-09T09:42:30.1229360Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::panic_strategy<'tcx>>`
2020-02-09T09:42:30.1229710Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1230340Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1230732Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1231105Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1231613Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1232101Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1232285Z 
2020-02-09T09:42:30.1239280Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_session::config::SymbolManglingVersion, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1239884Z      |
2020-02-09T09:42:30.1239884Z      |
2020-02-09T09:42:30.1240381Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1240889Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1241271Z      |              |
2020-02-09T09:42:30.1241736Z      |              `std::cell::RefCell<std::option::Option<(rustc_session::config::SymbolManglingVersion, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1242032Z      |
2020-02-09T09:42:30.1242743Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_session::config::SymbolManglingVersion, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1243399Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_session::config::SymbolManglingVersion>`
2020-02-09T09:42:30.1243851Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::symbol_mangling_version<'tcx>>`
2020-02-09T09:42:30.1244230Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1244574Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1244941Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1245307Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1245646Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1246335Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1246430Z 
2020-02-09T09:42:30.1387623Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1388383Z      |
2020-02-09T09:42:30.1388383Z      |
2020-02-09T09:42:30.1388772Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1389439Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1389938Z      |              |
2020-02-09T09:42:30.1390499Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1390844Z      |
2020-02-09T09:42:30.1391488Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1392316Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashMap<rustc_hir::def_id::DefId, middle::exported_symbols::SymbolExportLevel, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:42:30.1392784Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::reachable_non_generics<'tcx>>`
2020-02-09T09:42:30.1393155Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1393518Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1393895Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1394259Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1394611Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1395057Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1395148Z 
2020-02-09T09:42:30.1395990Z error[E0277]: `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1396584Z      |
2020-02-09T09:42:30.1396584Z      |
2020-02-09T09:42:30.1397082Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1397693Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1398090Z      |              |
2020-02-09T09:42:30.1398777Z      |              `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1399124Z      |
2020-02-09T09:42:30.1399907Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1400827Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&std::collections::HashMap<rustc_hir::def_id::DefId, std::collections::HashMap<&ty::List<ty::subst::GenericArg<'_>>, rustc_hir::def_id::CrateNum, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
2020-02-09T09:42:30.1401308Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::upstream_monomorphizations<'tcx>>`
2020-02-09T09:42:30.1401649Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1401992Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1402334Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1402653Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1402994Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1403396Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1403486Z 
2020-02-09T09:42:30.1587939Z error[E0277]: `std::cell::RefCell<std::option::Option<(&[middle::cstore::ForeignModule], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1588548Z      |
2020-02-09T09:42:30.1588548Z      |
2020-02-09T09:42:30.1589047Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1589902Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1590312Z      |              |
2020-02-09T09:42:30.1591028Z      |              `std::cell::RefCell<std::option::Option<(&[middle::cstore::ForeignModule], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1591301Z      |
2020-02-09T09:42:30.1591842Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&[middle::cstore::ForeignModule], dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1592305Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&[middle::cstore::ForeignModule]>`
2020-02-09T09:42:30.1592869Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::foreign_modules<'tcx>>`
2020-02-09T09:42:30.1593240Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1593626Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1594009Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1594368Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1594737Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1595155Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1595272Z 
2020-02-09T09:42:30.1595726Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1596310Z      |
2020-02-09T09:42:30.1596310Z      |
2020-02-09T09:42:30.1596649Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1597195Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1597566Z      |              |
2020-02-09T09:42:30.1598246Z      |              `std::cell::RefCell<std::option::Option<(std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1598533Z      |
2020-02-09T09:42:30.1599196Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1599749Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::option::Option<(rustc_hir::def_id::DefId, rustc_session::config::EntryFnType)>>`
2020-02-09T09:42:30.1600272Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::entry_fn<'tcx>>`
2020-02-09T09:42:30.1601028Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1601494Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1601845Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1602202Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1602564Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1602973Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1603078Z 
2020-02-09T09:42:30.1651875Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_hir::def_id::DefId>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1652469Z      |
2020-02-09T09:42:30.1652469Z      |
2020-02-09T09:42:30.1652777Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1653298Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1653657Z      |              |
2020-02-09T09:42:30.1654103Z      |              `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_hir::def_id::DefId>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1654402Z      |
2020-02-09T09:42:30.1654935Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_hir::def_id::DefId>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1655394Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::option::Option<rustc_hir::def_id::DefId>>`
2020-02-09T09:42:30.1655830Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::plugin_registrar_fn<'tcx>>`
2020-02-09T09:42:30.1656333Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1656741Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1657099Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1657718Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1658057Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1658444Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1658547Z 
2020-02-09T09:42:30.1826314Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_session::session::CrateDisambiguator, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1826918Z      |
2020-02-09T09:42:30.1826918Z      |
2020-02-09T09:42:30.1827224Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1827734Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1828096Z      |              |
2020-02-09T09:42:30.1828534Z      |              `std::cell::RefCell<std::option::Option<(rustc_session::session::CrateDisambiguator, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1828830Z      |
2020-02-09T09:42:30.1829376Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_session::session::CrateDisambiguator, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1829838Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_session::session::CrateDisambiguator>`
2020-02-09T09:42:30.1830262Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_disambiguator<'tcx>>`
2020-02-09T09:42:30.1830647Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1831007Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1831366Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1831730Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1832088Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1832616Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1832741Z 
2020-02-09T09:42:30.1833147Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_data_structures::svh::Svh, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1833829Z      |
2020-02-09T09:42:30.1833829Z      |
2020-02-09T09:42:30.1834156Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.1834685Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1835157Z      |              |
2020-02-09T09:42:30.1835550Z      |              `std::cell::RefCell<std::option::Option<(rustc_data_structures::svh::Svh, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.1835830Z      |
2020-02-09T09:42:30.1836322Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_data_structures::svh::Svh, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.1836758Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_data_structures::svh::Svh>`
2020-02-09T09:42:30.1837157Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_hash<'tcx>>`
2020-02-09T09:42:30.1837523Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.1837883Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1838224Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.1838574Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1838918Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.1839318Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.1839420Z 
2020-02-09T09:42:30.2028607Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_data_structures::svh::Svh>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2029205Z      |
2020-02-09T09:42:30.2029205Z      |
2020-02-09T09:42:30.2029500Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.2030915Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.2033753Z      |              |
2020-02-09T09:42:30.2034331Z      |              `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_data_structures::svh::Svh>, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2034876Z      |
2020-02-09T09:42:30.2035575Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::option::Option<rustc_data_structures::svh::Svh>, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.2037073Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::option::Option<rustc_data_structures::svh::Svh>>`
2020-02-09T09:42:30.2037726Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::crate_host_hash<'tcx>>`
2020-02-09T09:42:30.2038292Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.2038798Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.2039122Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.2039464Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.2039800Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.2040184Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.2040295Z 
2020-02-09T09:42:30.2040820Z error[E0277]: `std::cell::RefCell<std::option::Option<(rustc_span::symbol::Symbol, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2041361Z      |
2020-02-09T09:42:30.2041361Z      |
2020-02-09T09:42:30.2042018Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.2042712Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.2043076Z      |              |
2020-02-09T09:42:30.2043506Z      |              `std::cell::RefCell<std::option::Option<(rustc_span::symbol::Symbol, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2043787Z      |
2020-02-09T09:42:30.2044589Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(rustc_span::symbol::Symbol, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.2045355Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<rustc_span::symbol::Symbol>`
2020-02-09T09:42:30.2046413Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::original_crate_name<'tcx>>`
2020-02-09T09:42:30.2046761Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.2047132Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.2047515Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.2048039Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.2048377Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.2048786Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.2048875Z 
2020-02-09T09:42:30.2049246Z error[E0277]: `std::cell::RefCell<std::option::Option<(std::string::String, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2050108Z      |
2020-02-09T09:42:30.2050108Z      |
2020-02-09T09:42:30.2050432Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.2051138Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.2051503Z      |              |
2020-02-09T09:42:30.2051940Z      |              `std::cell::RefCell<std::option::Option<(std::string::String, dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2052217Z      |
2020-02-09T09:42:30.2052745Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(std::string::String, dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.2053906Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<std::string::String>`
2020-02-09T09:42:30.2054831Z      = note: required because it appears within the type `ty::query::plumbing::QueryState<'tcx, ty::query::queries::extra_filename<'tcx>>`
2020-02-09T09:42:30.2055670Z      = note: required because it appears within the type `ty::query::Queries<'tcx>`
2020-02-09T09:42:30.2056018Z      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.2056470Z      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
2020-02-09T09:42:30.2056833Z      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.2057172Z      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
2020-02-09T09:42:30.2058016Z      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.2058105Z 
2020-02-09T09:42:30.2225396Z error[E0277]: `std::cell::RefCell<std::option::Option<(&[rustc_hir::def_id::DefId], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2225958Z      |
2020-02-09T09:42:30.2225958Z      |
2020-02-09T09:42:30.2226289Z 2705 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
2020-02-09T09:42:30.2226795Z      |              ^^^^^^^^ --------------------------------------------------- within this `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`
2020-02-09T09:42:30.2227137Z      |              |
2020-02-09T09:42:30.2227574Z      |              `std::cell::RefCell<std::option::Option<(&[rustc_hir::def_id::DefId], dep_graph::graph::DepNodeIndex)>>` cannot be shared between threads safely
2020-02-09T09:42:30.2227856Z      |
2020-02-09T09:42:30.2228357Z      = help: within `[closure@src/librustc/ty/mod.rs:2705:23: 2705:74 f:&F, self:&ty::context::TyCtxt<'tcx>]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<std::option::Option<(&[rustc_hir::def_id::DefId], dep_graph::graph::DepNodeIndex)>>`
2020-02-09T09:42:30.2228801Z      = note: required because it appears within the type `ty::query::caches::CrateNumCache<&[rustc_hir::def_id::DefId]>`
---
2020-02-09T09:42:58.1414014Z   local time: Sun Feb  9 09:42:58 UTC 2020
2020-02-09T09:42:58.8480798Z   network time: Sun, 09 Feb 2020 09:42:58 GMT
2020-02-09T09:42:58.8480908Z == end clock drift check ==
2020-02-09T09:43:00.0030368Z 
2020-02-09T09:43:00.0117436Z ##[error]Bash exited with code '1'.
2020-02-09T09:43:00.0170304Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-09T09:43:00.0172081Z ==============================================================================
2020-02-09T09:43:00.0172153Z Task         : Get sources
2020-02-09T09:43:00.0172236Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
