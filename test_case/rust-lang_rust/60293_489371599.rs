plain
travis_time:end:22b04b48:start=1557007263198269933,finish=1557007264278482835,duration=1080212902
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:54:24] 
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:24]    --> src/librustdoc/clean/mod.rs:261:20
[00:54:24]     |
[00:54:24] 261 |             if let Res::Def(DefKind::Mod, def_id) = res {
[00:54:24] 
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:24]    --> src/librustdoc/clean/mod.rs:261:29
[00:54:24]     |
[00:54:24]     |
[00:54:24] 261 |             if let Res::Def(DefKind::Mod, def_id) = res {
[00:54:24]     |                             ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:24]    --> src/librustdoc/clean/mod.rs:284:38
[00:54:24]     |
[00:54:24] 284 |                         as_primitive(Res::Def(
[00:54:24] 284 |                         as_primitive(Res::Def(
[00:54:24]     |                                      ^^^ use of undeclared type or module `Res`
[00:54:24] 
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:24]    --> src/librustdoc/clean/mod.rs:285:29
[00:54:24]     |
[00:54:24] 285 |                             DefKind::Mod,
[00:54:24]     |                             ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:24]    --> src/librustdoc/clean/mod.rs:305:20
[00:54:24]     |
[00:54:24]     |
[00:54:24] 305 |             if let Res::Def(DefKind::Mod, def_id) = res {
[00:54:24] 
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:24]    --> src/librustdoc/clean/mod.rs:305:29
[00:54:24]     |
[00:54:24]     |
[00:54:24] 305 |             if let Res::Def(DefKind::Mod, def_id) = res {
[00:54:24]     |                             ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:24]    --> src/librustdoc/clean/mod.rs:329:36
[00:54:24]     |
[00:54:24]     |
[00:54:24] 329 |                         as_keyword(Res::Def(
[00:54:24] 
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:24]    --> src/librustdoc/clean/mod.rs:330:29
[00:54:24]     |
[00:54:24]     |
[00:54:24] 330 |                             DefKind::Mod,
[00:54:24]     |                             ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:24] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:24]     --> src/librustdoc/clean/mod.rs:1166:14
[00:54:24]      |
[00:54:24] 1166 |         res: Res::Err,
[00:54:24] 1166 |         res: Res::Err,
[00:54:24]      |              ^^^ use of undeclared type or module `Res`
[00:54:24] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:2790:24
[00:54:25]      |
[00:54:25] 2790 |                 if let Res::Def(DefKind::TyParam, did) = path.res {
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:2790:33
[00:54:25]      |
[00:54:25]      |
[00:54:25] 2790 |                 if let Res::Def(DefKind::TyParam, did) = path.res {
[00:54:25]      |                                 ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:2800:24
[00:54:25]      |
[00:54:25]      |
[00:54:25] 2800 |                 if let Res::Def(DefKind::TyAlias, def_id) = path.res {
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:2800:33
[00:54:25]      |
[00:54:25]      |
[00:54:25] 2800 |                 if let Res::Def(DefKind::TyAlias, def_id) = path.res {
[00:54:25]      |                                 ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:2899:26
[00:54:25]      |
[00:54:25] 2899 |                     res: Res::Def(
---
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:2915:27
[00:54:25]      |
[00:54:25] 2915 |                     res = Res::Def(DefKind::Trait, proj.trait_ref(cx.tcx).def_id);
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:2915:36
[00:54:25]      |
[00:54:25]      |
[00:54:25] 2915 |                     res = Res::Def(DefKind::Trait, proj.trait_ref(cx.tcx).def_id);
[00:54:25]      |                                    ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:3907:23
[00:54:25]      |
[00:54:25] 3907 |             let res = Res::Def(
[00:54:25] 3907 |             let res = Res::Def(
[00:54:25]      |                       ^^^ use of undeclared type or module `Res`
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:3908:17
[00:54:25]      |
[00:54:25] 3908 |                 DefKind::Mod,
[00:54:25]      |                 ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:3963:21
[00:54:25]      |
[00:54:25]      |
[00:54:25] 3963 |                     Res::Def(DefKind::Mod, did) => {
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:3963:30
[00:54:25]      |
[00:54:25]      |
[00:54:25] 3963 |                     Res::Def(DefKind::Mod, did) => {
[00:54:25]      |                              ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4160:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4160 |         Res::PrimTy(p) => match p {
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4168:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4168 |         Res::SelfTy(..) if path.segments.len() == 1 => {
[00:54:25]      |         ^^^ use of undeclared type or module `Res`
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4171:9
[00:54:25]      |
[00:54:25] 4171 |         Res::Def(DefKind::TyParam, _) if path.segments.len() == 1 => {
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4171:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4171 |         Res::Def(DefKind::TyParam, _) if path.segments.len() == 1 => {
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4174:9
[00:54:25]      |
[00:54:25] 4174 |         Res::SelfTy(..)
[00:54:25] 4174 |         Res::SelfTy(..)
[00:54:25]      |         ^^^ use of undeclared type or module `Res`
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4175:11
[00:54:25]      |
[00:54:25] 4175 |         | Res::Def(DefKind::TyParam, _)
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4175:20
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4175 |         | Res::Def(DefKind::TyParam, _)
[00:54:25]      |                    ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4176:11
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4176 |         | Res::Def(DefKind::AssociatedTy, _) => true,
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4176:20
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4176 |         | Res::Def(DefKind::AssociatedTy, _) => true,
[00:54:25]      |                    ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4187:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4187 |         Res::Def(DefKind::Fn, i) => (i, TypeKind::Function),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4187:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4187 |         Res::Def(DefKind::Fn, i) => (i, TypeKind::Function),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4188:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4188 |         Res::Def(DefKind::TyAlias, i) => (i, TypeKind::Typedef),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4188:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4188 |         Res::Def(DefKind::TyAlias, i) => (i, TypeKind::Typedef),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4189:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4189 |         Res::Def(DefKind::Enum, i) => (i, TypeKind::Enum),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4189:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4189 |         Res::Def(DefKind::Enum, i) => (i, TypeKind::Enum),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4190:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4190 |         Res::Def(DefKind::Trait, i) => (i, TypeKind::Trait),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4190:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4190 |         Res::Def(DefKind::Trait, i) => (i, TypeKind::Trait),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4191:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4191 |         Res::Def(DefKind::Struct, i) => (i, TypeKind::Struct),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4191:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4191 |         Res::Def(DefKind::Struct, i) => (i, TypeKind::Struct),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4192:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4192 |         Res::Def(DefKind::Union, i) => (i, TypeKind::Union),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4192:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4192 |         Res::Def(DefKind::Union, i) => (i, TypeKind::Union),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4193:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4193 |         Res::Def(DefKind::Mod, i) => (i, TypeKind::Module),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4193:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4193 |         Res::Def(DefKind::Mod, i) => (i, TypeKind::Module),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4194:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4194 |         Res::Def(DefKind::ForeignTy, i) => (i, TypeKind::Foreign),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4194:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4194 |         Res::Def(DefKind::ForeignTy, i) => (i, TypeKind::Foreign),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4195:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4195 |         Res::Def(DefKind::Const, i) => (i, TypeKind::Const),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4195:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4195 |         Res::Def(DefKind::Const, i) => (i, TypeKind::Const),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4196:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4196 |         Res::Def(DefKind::Static, i) => (i, TypeKind::Static),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4196:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4196 |         Res::Def(DefKind::Static, i) => (i, TypeKind::Static),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4197:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4197 |         Res::Def(DefKind::Variant, i) => (cx.tcx.parent(i).expect("cannot get parent def id"),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4197:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4197 |         Res::Def(DefKind::Variant, i) => (cx.tcx.parent(i).expect("cannot get parent def id"),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4199:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4199 |         Res::Def(DefKind::Macro(mac_kind), i) => match mac_kind {
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4199:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4199 |         Res::Def(DefKind::Macro(mac_kind), i) => match mac_kind {
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4205:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4205 |         Res::Def(DefKind::TraitAlias, i) => (i, TypeKind::TraitAlias),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `DefKind`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4205:18
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4205 |         Res::Def(DefKind::TraitAlias, i) => (i, TypeKind::TraitAlias),
[00:54:25]      |                  ^^^^^^^ use of undeclared type or module `DefKind`
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4206:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4206 |         Res::SelfTy(Some(def_id), _) => (def_id, TypeKind::Trait),
[00:54:25] 
[00:54:25] error[E0433]: failed to resolve: use of undeclared type or module `Res`
[00:54:25]     --> src/librustdoc/clean/mod.rs:4207:9
[00:54:25]      |
[00:54:25]      |
[00:54:25] 4207 |         Res::SelfTy(_, Some(impl_def_id)) => return impl_def_id,
[00:54:25] 
[00:54:25] error[E0412]: cannot find type `Res` in this scope
[00:54:25]    --> src/librustdoc/clean/mod.rs:260:34
[00:54:25]     |
[00:54:25]     |
[00:54:25] 260 |         let as_primitive = |res: Res| {
[00:54:25] help: possible candidate is found in another module, you can import it into scope
[00:54:25]     |
[00:54:25] 12  | use rustc::hir::def::Res;
[00:54:25]     |
[00:54:25]     |
[00:54:25] 
[00:54:25] error[E0412]: cannot find type `Res` in this scope
[00:54:25]    --> src/librustdoc/clean/mod.rs:304:32
[00:54:25]     |
[00:54:25] 304 |         let as_keyword = |res: Res| {
[00:54:25] help: possible candidate is found in another module, you can import it into scope
[00:54:25]     |
[00:54:25] 12  | use rustc::hir::def::Res;
[00:54:25]     |
---
[00:54:25] 
[00:54:25] error[E0412]: cannot find type `Res` in this scope
[00:54:25]     --> src/librustdoc/clean/mod.rs:4183:47
[00:54:25]      |
[00:54:25] 4183 | pub fn register_res(cx: &DocContext<'_>, res: Res) -> DefId {
[00:54:25] help: possible candidate is found in another module, you can import it into scope
[00:54:25]      |
[00:54:25] 12   | use rustc::hir::def::Res;
[00:54:25]      |
---
22636 ./src/llvm-project/llgo/third_party/gofrontend/libgo
21336 ./src/stdsimd/crates/stdsimd-veriafter_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ec218d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d11f76b
$ dmesg | grep -i kill
