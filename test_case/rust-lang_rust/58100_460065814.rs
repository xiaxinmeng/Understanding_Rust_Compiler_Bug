plain
travis_time:end:2cf40e95:start=1549208012365143660,finish=1549208093583390720,duration=81218247060
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:48:59]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/inline.rs:38:24
[00:49:00]    |
[00:49:00] 38 | pub fn try_inline(cx: &DocContext, def: Def, name: ast::Name, visited: &mut FxHashSet<DefId>)
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:124:29
[00:49:00]     |
[00:49:00]     |
[00:49:00] 124 | pub fn try_inline_glob(cx: &DocContext, def: Def, visited: &mut FxHashSet<DefId>)
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:141:24
[00:49:00]     |
[00:49:00]     |
[00:49:00] 141 | pub fn load_attrs(cx: &DocContext, did: DefId) -> clean::Attributes {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:149:31
[00:49:00]     |
[00:49:00]     |
[00:49:00] 149 | pub fn record_extern_fqn(cx: &DocContext, did: DefId, kind: clean::TypeKind) {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:177:34
[00:49:00]     |
[00:49:00]     |
[00:49:00] 177 | pub fn build_external_trait(cx: &DocContext, did: DefId) -> clean::Trait {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:197:33
[00:49:00]     |
[00:49:00]     |
[00:49:00] 197 | fn build_external_function(cx: &DocContext, did: DefId) -> clean::Function {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:219:20
[00:49:00]     |
[00:49:00]     |
[00:49:00] 219 | fn build_enum(cx: &DocContext, did: DefId) -> clean::Enum {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:229:22
[00:49:00]     |
[00:49:00]     |
[00:49:00] 229 | fn build_struct(cx: &DocContext, did: DefId) -> clean::Struct {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:245:21
[00:49:00]     |
[00:49:00]     |
[00:49:00] 245 | fn build_union(cx: &DocContext, did: DefId) -> clean::Union {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:257:26
[00:49:00]     |
[00:49:00]     |
[00:49:00] 257 | fn build_type_alias(cx: &DocContext, did: DefId) -> clean::Typedef {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:266:25
[00:49:00]     |
[00:49:00]     |
[00:49:00] 266 | pub fn build_impls(cx: &DocContext, did: DefId) -> Vec<clean::Item> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:277:24
[00:49:00]     |
[00:49:00]     |
[00:49:00] 277 | pub fn build_impl(cx: &DocContext, did: DefId, ret: &mut Vec<clean::Item>) {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:387:22
[00:49:00]     |
[00:49:00]     |
[00:49:00] 387 | fn build_module(cx: &DocContext, did: DefId, visited: &mut FxHashSet<DefId>) -> clean::Module {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:395:21
[00:49:00]     |
[00:49:00]     |
[00:49:00] 395 |     fn fill_in(cx: &DocContext, did: DefId, items: &mut Vec<clean::Item>,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:412:33
[00:49:00]     |
[00:49:00]     |
[00:49:00] 412 | pub fn print_inlined_const(cx: &DocContext, did: DefId) -> String {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:420:21
[00:49:00]     |
[00:49:00]     |
[00:49:00] 420 | fn build_const(cx: &DocContext, did: DefId) -> clean::Constant {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:427:22
[00:49:00]     |
[00:49:00]     |
[00:49:00] 427 | fn build_static(cx: &DocContext, did: DefId, mutable: bool) -> clean::Static {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:435:21
[00:49:00]     |
[00:49:00]     |
[00:49:00] 435 | fn build_macro(cx: &DocContext, did: DefId, name: ast::Name) -> clean::ItemEnum {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/inline.rs:537:33
[00:49:00]     |
[00:49:00]     |
[00:49:00] 537 | pub fn record_extern_trait(cx: &DocContext, did: DefId) {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/cfg.rs:264:15
[00:49:00]     |
---
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/simplify.rs:25:27
[00:49:00]    |
[00:49:00] 25 | pub fn where_clauses(cx: &DocContext, clauses: Vec<WP>) -> Vec<WP> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/simplify.rs:144:37
[00:49:00]     |
[00:49:00]     |
[00:49:00] 144 | fn trait_is_same_or_supertrait(cx: &DocContext, child: DefId,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/auto_trait.rs:223:36
[00:49:00]     |
[00:49:00]     |
[00:49:00] 223 |     fn get_lifetime(&self, region: Region, names_map: &FxHashMap<String, Lifetime>) -> Lifetime {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/auto_trait.rs:234:35
[00:49:00]     |
[00:49:00]     |
[00:49:00] 234 |     fn region_name(&self, region: Region) -> Option<String> {
[00:49:00]     |                                   ^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/auto_trait.rs:262:36
[00:49:00]     |
[00:49:00] 262 |         let mut vid_map: FxHashMap<RegionTarget, RegionDeps> = Default::default();
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/auto_trait.rs:262:50
[00:49:00]     |
[00:49:00]     |
[00:49:00] 262 |         let mut vid_map: FxHashMap<RegionTarget, RegionDeps> = Default::default();
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/auto_trait.rs:840:30
[00:49:00]     |
[00:49:00]     |
[00:49:00] 840 |     fn is_fn_ty(&self, tcx: &TyCtxt, ty: &Type) -> bool {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]  --> src/librustdoc/clean/def_ctor.rs:5:36
[00:49:00]   |
[00:49:00] 5 | pub fn get_def_from_def_id<F>(cx: &DocContext,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/def_ctor.rs:41:37
[00:49:00]    |
[00:49:00] 41 | pub fn get_def_from_node_id<F>(cx: &DocContext,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/mod.rs:73:23
[00:49:00]    |
[00:49:00]    |
[00:49:00] 73 | fn get_stability(cx: &DocContext, def_id: DefId) -> Option<Stability> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/mod.rs:77:25
[00:49:00]    |
[00:49:00]    |
[00:49:00] 77 | fn get_deprecation(cx: &DocContext, def_id: DefId) -> Option<Deprecation> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/mod.rs:82:26
[00:49:00]    |
[00:49:00]    |
[00:49:00] 82 |     fn clean(&self, cx: &DocContext) -> T;
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/mod.rs:86:26
[00:49:00]    |
[00:49:00]    |
[00:49:00] 86 |     fn clean(&self, cx: &DocContext) -> Vec<U> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/mod.rs:92:26
[00:49:00]    |
[00:49:00]    |
[00:49:00] 92 |     fn clean(&self, cx: &DocContext) -> IndexVec<V, U> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/clean/mod.rs:98:26
[00:49:00]    |
[00:49:00]    |
[00:49:00] 98 |     fn clean(&self, cx: &DocContext) -> U {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:104:26
[00:49:00]     |
[00:49:00]     |
[00:49:00] 104 |     fn clean(&self, cx: &DocContext) -> U {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:110:26
[00:49:00]     |
[00:49:00]     |
[00:49:00] 110 |     fn clean(&self, cx: &DocContext) -> Option<U> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:116:26
[00:49:00]     |
[00:49:00]     |
[00:49:00] 116 |     fn clean(&self, cx: &DocContext) -> U {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:122:26
[00:49:00]     |
[00:49:00]     |
[00:49:00] 122 |     fn clean(&self, cx: &DocContext) -> Vec<U> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:142:26
[00:49:00]     |
[00:49:00]     |
[00:49:00] 142 |     fn clean(&self, cx: &DocContext) -> Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:231:26
[00:49:00]     |
[00:49:00]     |
[00:49:00] 231 |     fn clean(&self, cx: &DocContext) -> ExternalCrate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:363:29
[00:49:00]     |
[00:49:00]     |
[00:49:00] 363 |     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]     |                             ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/clean/mod.rs:577:26
[00:49:00]     |
[00:49:00] 577 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1014:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1014 |     fn clean(&self, cx: &DocContext) -> Attributes {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1026:25
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1026 |     fn maybe_sized(cx: &DocContext) -> GenericBound {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1043:35
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1043 |     fn is_sized_bound(&self, cx: &DocContext) -> bool {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1069:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1069 |     fn clean(&self, cx: &DocContext) -> GenericBound {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1079:31
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1079 | fn external_generic_args(cx: &DocContext, trait_did: Option<DefId>, has_self: bool,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1080:62
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1080 |                         bindings: Vec<TypeBinding>, substs: &Substs) -> GenericArgs {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1121:23
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1121 | fn external_path(cx: &DocContext, name: &str, trait_did: Option<DefId>, has_self: bool,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1122:55
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1122 |                  bindings: Vec<TypeBinding>, substs: &Substs) -> Path {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1134:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1134 |     fn clean(&self, cx: &DocContext) -> GenericBound {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1178:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1178 |     fn clean(&self, cx: &DocContext) -> GenericBound {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1184:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1184 |     fn clean(&self, cx: &DocContext) -> Option<Vec<GenericBound>> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1211:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1211 |     fn clean(&self, cx: &DocContext) -> Lifetime {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1231:25
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1231 |     fn clean(&self, _: &DocContext) -> Lifetime {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1255:27
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1255 |     fn clean(&self, _cx: &DocContext) -> Lifetime {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1261:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1261 |     fn clean(&self, cx: &DocContext) -> Option<Lifetime> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1287:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1287 |     fn clean(&self, cx: &DocContext) -> WherePredicate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1314:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1314 |     fn clean(&self, cx: &DocContext) -> WherePredicate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1333:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1333 |     fn clean(&self, cx: &DocContext) -> WherePredicate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1342:27
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1342 |     fn clean(&self, _cx: &DocContext) -> WherePredicate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1349:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1349 |     fn clean(&self, cx: &DocContext) -> WherePredicate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1359:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1359 |     fn clean(&self, cx: &DocContext) -> WherePredicate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1370:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1370 |     fn clean(&self, cx: &DocContext) -> WherePredicate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1379:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1379 |     fn clean(&self, cx: &DocContext) -> Type {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1420:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1420 |     fn clean(&self, cx: &DocContext) -> GenericParamDef {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1450:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1450 |     fn clean(&self, cx: &DocContext) -> GenericParamDef {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1494:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1494 |     fn clean(&self, cx: &DocContext) -> Generics {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1562:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1562 |     fn clean(&self, cx: &DocContext) -> Generics {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1643:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1643 |     fn clean(&self, cx: &DocContext) -> Method {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1670:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1670 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1718:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1718 |     fn clean(&self, cx: &DocContext) -> Arguments {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1736:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1736 |     fn clean(&self, cx: &DocContext) -> Arguments {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1753:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1753 |     fn clean(&self, cx: &DocContext) -> FnDecl {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1764:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1764 |     fn clean(&self, cx: &DocContext) -> FnDecl {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1825:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1825 |     fn clean(&self, cx: &DocContext) -> FunctionRetTy {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1854:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1854 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1879:25
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1879 |     fn clean(&self, _: &DocContext) -> bool {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1888:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1888 |     fn clean(&self, cx: &DocContext) -> Type {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1894:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1894 |     fn clean(&self, cx: &DocContext) -> PolyTrait {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1903:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1903 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1940:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1940 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:1972:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 1972 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2398:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2398 |     fn clean(&self, cx: &DocContext) -> Type {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2574:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2574 |     fn clean(&self, cx: &DocContext) -> Type {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2769:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2769 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2784:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2784 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2807:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2807 |     fn clean(&self, cx: &DocContext) -> Option<Visibility> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2822:25
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2822 |     fn clean(&self, _: &DocContext) -> Option<Visibility> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2844:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2844 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2864:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2864 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2894:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2894 |     fn clean(&self, cx: &DocContext) -> VariantStruct {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2911:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2911 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2935:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2935 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:2952:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2952 |     fn clean(&self, cx: &DocContext) -> Item {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3000:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3000 |     fn clean(&self, cx: &DocContext) -> VariantKind {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3031:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3031 |     fn clean(&self, cx: &DocContext) -> Span {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3064:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3064 |     fn clean(&self, cx: &DocContext) -> Path {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3087:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3087 |     fn clean(&self, cx: &DocContext) -> GenericArgs {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3126:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3126 |     fn clean(&self, cx: &DocContext) -> PathSegment {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3198:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3198 |     fn clean(&self, cx: &DocContext) -> String {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3205:25
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3205 |     fn clean(&self, _: &DocContext) -> String {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/clean/mod.rs:3212:25
---
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/format.rs:432:26
[00:49:00]     |
[00:49:00] 432 | fn resolved_path(w: &mut fmt::Formatter, did: DefId, path: &clean::Path,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/format.rs:461:27
[00:49:00]     |
[00:49:00]     |
[00:49:00] 461 | fn primitive_link(f: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/format.rs:506:21
[00:49:00]     |
[00:49:00]     |
[00:49:00] 506 | fn tybounds(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/format.rs:527:27
[00:49:00]     |
[00:49:00]     |
[00:49:00] 527 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/format.rs:540:38
[00:49:00]     |
[00:49:00] 540 | fn fmt_type(t: &clean::Type, f: &mut fmt::Formatter, use_absolute: bool) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/format.rs:728:27
[00:49:00]     |
---
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/html/layout.rs:30:12
[00:49:00]    |
[00:49:00] 30 |     page: &Page,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/markdown.rs:104:25
[00:49:00]     |
[00:49:00]     |
[00:49:00] 104 | fn map_line(s: &str) -> Line {
[00:49:00]     |                         ^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/markdown.rs:188:40
[00:49:00]     |
[00:49:00] 188 |         let text = lines.collect::<Vec<Cow<str>>>().join("\n");
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/markdown.rs:199:36
[00:49:00]     |
[00:49:00]     |
[00:49:00] 199 |                     .collect::<Vec<Cow<str>>>().join("\n");
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/markdown.rs:389:29
[00:49:00]     |
[00:49:00]     |
[00:49:00] 389 | fn check_if_allowed_tag(t: &Tag) -> bool {
[00:49:00]     |                             ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/markdown.rs:526:27
[00:49:00]     |
[00:49:00] 526 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:49:00] 526 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/markdown.rs:572:52
[00:49:00]     |
[00:49:00] 572 |                     let text = lines.collect::<Vec<Cow<str>>>().join("\n");
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/markdown.rs:684:29
[00:49:00]     |
---
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:1874:26
[00:49:00]      |
[00:49:00] 1874 | fn print_entries(f: &mut fmt::Formatter, e: &FxHashSet<ItemEntry>, title: &str,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:1889:27
[00:49:00]      |
---
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2346:18
[00:49:00]      |
[00:49:00] 2346 | where F: Fn(&mut fmt::Formatter) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2344:34
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2344 | fn wrap_into_docblock<F>(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2353:29
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2353 |     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]      |                             ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2494:21
[00:49:00]      |
[00:49:00] 2494 | fn document(w: &mut fmt::Formatter, cx: &Context, item: &clean::Item) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2504:28
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2504 | fn render_markdown(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2519:27
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2519 | fn document_short(w: &mut fmt::Formatter, cx: &Context, item: &clean::Item, link: AssocItemLink,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2519:83
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2519 | fn document_short(w: &mut fmt::Formatter, cx: &Context, item: &clean::Item, link: AssocItemLink,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2537:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2537 | fn document_full(w: &mut fmt::Formatter, item: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2550:31
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2550 | fn document_stability(w: &mut fmt::Formatter, cx: &Context, item: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2567:36
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2567 | fn document_non_exhaustive(w: &mut fmt::Formatter, item: &clean::Item) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2615:24
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2615 | fn item_module(w: &mut fmt::Formatter, cx: &Context,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2918:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2918 | fn item_constant(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2930:24
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2930 | fn item_static(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2943:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2943 | fn item_function(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2974:65
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2974 | fn render_implementor(cx: &Context, implementor: &Impl, w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:2991:39
[00:49:00]      |
[00:49:00]      |
[00:49:00] 2991 | fn render_impls(cx: &Context, w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3034:13
[00:49:00]      |
---
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3131:38
[00:49:00]      |
[00:49:00] 3131 |     fn write_loading_content(w: &mut fmt::Formatter, extra_content: &str) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3135:27
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3135 |     fn trait_item(w: &mut fmt::Formatter, cx: &Context, m: &clean::Item, t: &clean::Item)
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3292:45
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3292 | fn naive_assoc_href(it: &clean::Item, link: AssocItemLink) -> String {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3311:24
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3311 | fn assoc_const(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3315:22
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3315 |                link: AssocItemLink) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3327:36
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3327 |                              link: AssocItemLink) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3353:35
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3353 | fn render_stability_since(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3359:30
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3359 | fn render_assoc_item(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3361:28
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3361 |                      link: AssocItemLink,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3363:23
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3363 |     fn method(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3368:21
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3368 |               link: AssocItemLink,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3442:24
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3442 | fn item_struct(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3497:23
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3497 | fn item_union(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3542:22
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3542 | fn item_enum(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3706:30
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3706 | fn render_attributes(w: &mut fmt::Formatter, it: &clean::Item) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3724:26
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3724 | fn render_struct(w: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3800:25
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3800 | fn render_union(w: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3858:31
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3858 | fn render_assoc_items(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3862:29
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3862 |                       what: AssocItemRender) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3920:37
[00:49:00]      |
[00:49:00]      |
[00:49:00] 3920 |             fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]      |                                     ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:3961:33
[00:49:00]      |
[00:49:00] 3961 | fn render_deref_methods(w: &mut fmt::Formatter, cx: &Context, impl_: &Impl,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4072:24
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4072 | fn render_impl(w: &mut fmt::Formatter, cx: &Context, i: &Impl, link: AssocItemLink,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4072:70
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4072 | fn render_impl(w: &mut fmt::Formatter, cx: &Context, i: &Impl, link: AssocItemLink,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4114:30
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4114 |     fn doc_impl_item(w: &mut fmt::Formatter, cx: &Context, item: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4115:28
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4115 |                      link: AssocItemLink, render_mode: RenderMode,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4229:37
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4229 |     fn render_default_items(w: &mut fmt::Formatter,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4262:13
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4262 |     w: &mut fmt::Formatter,
[00:49:00]      |             ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4284:25
[00:49:00]      |
[00:49:00] 4284 | fn item_typedef(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4303:30
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4303 | fn item_foreign_type(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4319:29
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4319 |     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]      |                             ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4575:29
[00:49:00]      |
[00:49:00] 4575 | fn sidebar_struct(fmt: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4612:28
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4612 | fn sidebar_trait(fmt: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4721:32
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4721 | fn sidebar_primitive(fmt: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4731:30
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4731 | fn sidebar_typedef(fmt: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4756:28
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4756 | fn sidebar_union(fmt: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4774:27
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4774 | fn sidebar_enum(fmt: &mut fmt::Formatter, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4828:29
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4828 | fn sidebar_module(fmt: &mut fmt::Formatter, _it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4860:35
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4860 | fn sidebar_foreign_type(fmt: &mut fmt::Formatter, it: &clean::Item) -> fmt::Result {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4869:29
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4869 |     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]      |                             ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4889:23
[00:49:00]      |
[00:49:00] 4889 | fn item_macro(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4900:28
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4900 | fn item_proc_macro(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item, m: &clean::ProcMacro)
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4933:27
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4933 | fn item_primitive(w: &mut fmt::Formatter, cx: &Context,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]     --> src/librustdoc/html/render.rs:4940:25
[00:49:00]      |
[00:49:00]      |
[00:49:00] 4940 | fn item_keyword(w: &mut fmt::Formatter, cx: &Context,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/html/toc.rs:169:27
[00:49:00]     |
---
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/strip_hidden.rs:17:46
[00:49:00]    |
[00:49:00] 17 | pub fn strip_hidden(krate: clean::Crate, _: &DocContext) -> clean::Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/strip_private.rs:15:52
[00:49:00]    |
[00:49:00]    |
[00:49:00] 15 | pub fn strip_private(mut krate: clean::Crate, cx: &DocContext) -> clean::Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]  --> src/librustdoc/passes/strip_priv_imports.rs:9:52
[00:49:00]   |
[00:49:00]   |
[00:49:00] 9 | pub fn strip_priv_imports(krate: clean::Crate, _: &DocContext)  -> clean::Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/collect_intra_doc_links.rs:25:51
[00:49:00]    |
[00:49:00]    |
[00:49:00] 25 | pub fn collect_intra_doc_links(krate: Crate, cx: &DocContext) -> Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/passes/collect_intra_doc_links.rs:426:23
[00:49:00]     |
[00:49:00]     |
[00:49:00] 426 | fn macro_resolve(cx: &DocContext, path_str: &str) -> Option<Def> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/passes/collect_intra_doc_links.rs:454:10
[00:49:00]     |
[00:49:00]     |
[00:49:00] 454 |     cx: &DocContext,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/passes/collect_intra_doc_links.rs:510:25
[00:49:00]     |
[00:49:00]     |
[00:49:00] 510 | fn ambiguity_error(cx: &DocContext, attrs: &Attributes,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/passes/collect_intra_doc_links.rs:566:24
[00:49:00]     |
[00:49:00]     |
[00:49:00] 566 | fn handle_variant(cx: &DocContext, def: Def) -> Result<(Def, Option<String>), ()> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/private_items_doc_tests.rs:24:57
[00:49:00]    |
[00:49:00]    |
[00:49:00] 24 | pub fn check_private_items_doc_tests(krate: Crate, cx: &DocContext) -> Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/collect_trait_impls.rs:14:47
[00:49:00]    |
[00:49:00]    |
[00:49:00] 14 | pub fn collect_trait_impls(krate: Crate, cx: &DocContext) -> Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/check_code_block_syntax.rs:17:58
[00:49:00]    |
[00:49:00]    |
[00:49:00] 17 | pub fn check_code_block_syntax(krate: clean::Crate, cx: &DocContext) -> clean::Crate {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/mod.rs:58:33
[00:49:00]    |
[00:49:00]    |
[00:49:00] 58 |         pass: fn(clean::Crate, &DocContext) -> clean::Crate,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/mod.rs:70:27
[00:49:00]    |
[00:49:00]    |
[00:49:00] 70 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:49:00]    |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]   --> src/librustdoc/passes/mod.rs:86:48
[00:49:00]    |
[00:49:00] 86 |                        pass: fn(clean::Crate, &DocContext) -> clean::Crate,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/passes/mod.rs:115:55
[00:49:00]     |
[00:49:00]     |
[00:49:00] 115 |     pub fn early_fn(self) -> Option<fn(clean::Crate, &DocContext) -> clean::Crate> {
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/passes/mod.rs:422:10
[00:49:00]     |
[00:49:00]     |
[00:49:00] 422 |     cx: &DocContext,
[00:49:00] 
[00:49:00] error: hidden lifetime parameters in types are deprecated
[00:49:00]    --> src/librustdoc/visit_ast.rs:271:37
[00:49:00]     |
[00:49:00]     |
[00:49:00] 271 |         fn inherits_doc_hidden(cx: &core::DocContext, mut node: ast::NodeId) -> bool {
[00:49:00] 
[00:49:04] error: unused extern crate
[00:49:04]   --> src/librustdoc/lib.rs:25:1
[00:49:04]    |
[00:49:04]    |
[00:49:04] 25 | extern crate arena;
[00:49:04]    | ^^^^^^^^^^^^^^^^^^^ help: remove it
[00:49:04]    |
[00:49:04] note: lint level defined here
[00:49:04]   --> src/librustdoc/lib.rs:1:9
[00:49:04]    |
[00:49:04] 1  | #![deny(rust_2018_idioms)]
[00:49:04]    |         ^^^^^^^^^^^^^^^^
[00:49:04]    = note: #[deny(unused_extern_crates)] implied by #[deny(rust_2018_idioms)]
[00:49:04] error: aborting due to 277 previous errors
[00:49:04] 
[00:49:05] error: Could not compile `rustdoc`.
[00:49:05] 
---
[00:49:05] 
[00:49:05] 
[00:49:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:49:05] Build completed unsuccessfully in 0:45:38
[00:49:05] Makefile:18: recipe for target 'all' failed
[00:49:05] make: *** [all] Error 1
110888 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu
108528 ./src/llvm-project/lldb
103572 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
97552 ./src/llvm-project/clang/test
---
travis_time:end:0341b0e0:start=1549211048486078401,finish=1549211048491249564,duration=5171163
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07b6181e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25e3c388
