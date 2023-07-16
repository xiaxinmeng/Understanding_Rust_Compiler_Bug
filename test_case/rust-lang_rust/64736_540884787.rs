plain
2019-10-11T03:17:29.0603327Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T03:17:29.0702930Z ##[command]git config gc.auto 0
2019-10-11T03:17:29.0780553Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T03:17:29.0853503Z ##[command]git config --get-all http.proxy
2019-10-11T03:17:29.0999223Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-11T03:26:04.7405806Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-10-11T03:26:06.5135486Z error[E0308]: mismatched types
2019-10-11T03:26:06.5135960Z    --> src/librustc_mir/borrow_check/mod.rs:249:9
2019-10-11T03:26:06.5136181Z     |
2019-10-11T03:26:06.5136436Z 249 |         body_cache,
2019-10-11T03:26:06.5136804Z     |         ^^^^^^^^^^ expected struct `rustc::mir::BodyCache`, found reference
2019-10-11T03:26:06.5137027Z     |
2019-10-11T03:26:06.5137355Z     = note: expected type `rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:06.5137627Z                found type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:06.5137666Z 
2019-10-11T03:26:06.5219394Z error[E0615]: attempted to take value of method `body` on type `borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:06.5220150Z    --> src/librustc_mir/borrow_check/mod.rs:288:69
2019-10-11T03:26:06.5220407Z     |
2019-10-11T03:26:06.5220754Z 288 |         let lint_root = if let ClearCrossCrate::Set(ref vsi) = mbcx.body.source_scope_local_data {
2019-10-11T03:26:06.5221304Z     |                                                                     ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.5221394Z 
2019-10-11T03:26:06.5266281Z error[E0615]: attempted to take value of method `body` on type `borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:06.5266633Z    --> src/librustc_mir/borrow_check/mod.rs:289:30
2019-10-11T03:26:06.5266887Z     |
2019-10-11T03:26:06.5267185Z 289 |             let scope = mbcx.body.source_info(location).scope;
2019-10-11T03:26:06.5267557Z     |                              ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.5267604Z 
2019-10-11T03:26:06.5332424Z error[E0615]: attempted to take value of method `body` on type `borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:06.5332796Z    --> src/librustc_mir/borrow_check/mod.rs:312:31
2019-10-11T03:26:06.5333020Z     |
2019-10-11T03:26:06.5333346Z 312 |         .filter(|&local| mbcx.body.local_decls[*local].is_user_variable.is_none())
2019-10-11T03:26:06.5333735Z     |                               ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.5333793Z 
2019-10-11T03:26:06.5387356Z error[E0615]: attempted to take value of method `body` on type `borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:06.5387700Z    --> src/librustc_mir/borrow_check/mod.rs:318:34
2019-10-11T03:26:06.5387921Z     |
2019-10-11T03:26:06.5388228Z 318 |     let unused_mut_locals = mbcx.body.mut_vars_iter()
2019-10-11T03:26:06.5388585Z     |                                  ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.5388630Z 
2019-10-11T03:26:06.5443673Z error[E0615]: attempted to take value of method `body` on type `borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:06.5444022Z    --> src/librustc_mir/borrow_check/mod.rs:325:23
2019-10-11T03:26:06.5444241Z     |
2019-10-11T03:26:06.5444596Z 325 |     for local in mbcx.body.mut_vars_and_args_iter().filter(|local| !used_mut.contains(local)) {
2019-10-11T03:26:06.5444969Z     |                       ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.5445015Z 
2019-10-11T03:26:06.5493318Z error[E0615]: attempted to take value of method `body` on type `borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:06.5493645Z    --> src/librustc_mir/borrow_check/mod.rs:326:53
2019-10-11T03:26:06.5493866Z     |
2019-10-11T03:26:06.5494460Z 326 |         if let ClearCrossCrate::Set(ref vsi) = mbcx.body.source_scope_local_data {
2019-10-11T03:26:06.5494845Z     |                                                     ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.5494891Z 
2019-10-11T03:26:06.5538755Z error[E0615]: attempted to take value of method `body` on type `borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:06.5539120Z    --> src/librustc_mir/borrow_check/mod.rs:327:36
2019-10-11T03:26:06.5539335Z     |
2019-10-11T03:26:06.5539658Z 327 |             let local_decl = &mbcx.body.local_decls[local];
2019-10-11T03:26:06.5540011Z     |                                    ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.5685865Z error[E0308]: mismatched types
2019-10-11T03:26:06.5686161Z    --> src/librustc_mir/borrow_check/mod.rs:478:9
2019-10-11T03:26:06.5686550Z     |
2019-10-11T03:26:06.5686550Z     |
2019-10-11T03:26:06.5686871Z 477 |     fn body(&self) -> &'cx Body<'tcx> {
2019-10-11T03:26:06.5687237Z     |                       --------------- expected `&'cx rustc::mir::Body<'tcx>` because of return type
2019-10-11T03:26:06.5687518Z 478 |         self.body_cache
2019-10-11T03:26:06.5688030Z     |         |
2019-10-11T03:26:06.5688343Z     |         expected reference, found struct `rustc::mir::BodyCache`
2019-10-11T03:26:06.5688343Z     |         expected reference, found struct `rustc::mir::BodyCache`
2019-10-11T03:26:06.5688636Z     |         help: consider borrowing here: `&self.body_cache`
2019-10-11T03:26:06.5689156Z     = note: expected type `&'cx rustc::mir::Body<'tcx>`
2019-10-11T03:26:06.5689156Z     = note: expected type `&'cx rustc::mir::Body<'tcx>`
2019-10-11T03:26:06.5689429Z                found type `rustc::mir::BodyCache<&'cx rustc::mir::Body<'tcx>>`
2019-10-11T03:26:06.5896527Z error[E0308]: mismatched types
2019-10-11T03:26:06.5896827Z    --> src/librustc_mir/borrow_check/mod.rs:628:51
2019-10-11T03:26:06.5897049Z     |
2019-10-11T03:26:06.5897049Z     |
2019-10-11T03:26:06.5897366Z 628 |                 let drop_place_ty = drop_place.ty(self.body_cache, self.infcx.tcx);
2019-10-11T03:26:06.5897929Z     |                                                   |
2019-10-11T03:26:06.5898265Z     |                                                   expected reference, found struct `rustc::mir::BodyCache`
2019-10-11T03:26:06.5898265Z     |                                                   expected reference, found struct `rustc::mir::BodyCache`
2019-10-11T03:26:06.5898578Z     |                                                   help: consider borrowing here: `&self.body_cache`
2019-10-11T03:26:06.5899262Z     = note: expected type `&_`
2019-10-11T03:26:06.5899262Z     = note: expected type `&_`
2019-10-11T03:26:06.5899515Z                found type `rustc::mir::BodyCache<&'cx rustc::mir::Body<'tcx>>`
2019-10-11T03:26:06.6251066Z error[E0308]: mismatched types
2019-10-11T03:26:06.6251548Z    --> src/librustc_mir/borrow_check/mod.rs:980:13
2019-10-11T03:26:06.6251809Z     |
2019-10-11T03:26:06.6252533Z 980 |             body,
2019-10-11T03:26:06.6252533Z 980 |             body,
2019-10-11T03:26:06.6252807Z     |             ^^^^
2019-10-11T03:26:06.6253065Z     |             |
2019-10-11T03:26:06.6253404Z     |             expected reference, found struct `rustc::mir::BodyCache`
2019-10-11T03:26:06.6253718Z     |             help: consider borrowing here: `&body`
2019-10-11T03:26:06.6253939Z     |
2019-10-11T03:26:06.6254258Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:06.6254551Z                found type `rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:06.6254592Z 
2019-10-11T03:26:06.6399144Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6400050Z     --> src/librustc_mir/borrow_check/mod.rs:1141:43
2019-10-11T03:26:06.6400497Z      |
2019-10-11T03:26:06.6400948Z 1141 |             if let Mutability::Not = self.body.local_decls[*local].mutability {
2019-10-11T03:26:06.6401502Z      |                                           ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6401677Z 
2019-10-11T03:26:06.6512752Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6513476Z     --> src/librustc_mir/borrow_check/mod.rs:1299:24
2019-10-11T03:26:06.6513936Z      |
2019-10-11T03:26:06.6514479Z 1299 |             }) if self.body.local_decls[local].is_user_variable.is_none() => {
2019-10-11T03:26:06.6515024Z      |                        ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6515398Z 
2019-10-11T03:26:06.6556080Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6556650Z     --> src/librustc_mir/borrow_check/mod.rs:1300:25
2019-10-11T03:26:06.6557097Z      |
2019-10-11T03:26:06.6557601Z 1300 |                 if self.body.local_decls[local].ty.is_mutable_ptr() {
2019-10-11T03:26:06.6558153Z      |                         ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6558331Z 
2019-10-11T03:26:06.6607873Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6608471Z     --> src/librustc_mir/borrow_check/mod.rs:1331:33
2019-10-11T03:26:06.6609236Z      |
2019-10-11T03:26:06.6609738Z 1331 |                 let bbd = &self.body[loc.block];
2019-10-11T03:26:06.6610847Z      |                                 ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6611053Z 
2019-10-11T03:26:06.6701110Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6701637Z     --> src/librustc_mir/borrow_check/mod.rs:1452:18
2019-10-11T03:26:06.6703202Z 1452 |             self.body,
2019-10-11T03:26:06.6703780Z      |                  ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6703969Z 
2019-10-11T03:26:06.6703969Z 
2019-10-11T03:26:06.6792875Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6793555Z     --> src/librustc_mir/borrow_check/mod.rs:1532:40
2019-10-11T03:26:06.6794001Z      |
2019-10-11T03:26:06.6794503Z 1532 |             let span = init.span(&self.body);
2019-10-11T03:26:06.6795059Z      |                                        ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6795238Z 
2019-10-11T03:26:06.6906799Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6907410Z     --> src/librustc_mir/borrow_check/mod.rs:1743:79
2019-10-11T03:26:06.6907804Z      |
2019-10-11T03:26:06.6908316Z 1743 |                     let base_ty = Place::ty_from(&place.base, proj_base, self.body, tcx).ty;
2019-10-11T03:26:06.6908865Z      |                                                                               ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6909067Z 
2019-10-11T03:26:06.6993332Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.6993961Z     --> src/librustc_mir/borrow_check/mod.rs:1850:69
2019-10-11T03:26:06.6994436Z      |
2019-10-11T03:26:06.6994951Z 1850 |                     Place::ty_from(base.base, base.projection, this.body, tcx).ty.kind
2019-10-11T03:26:06.6995718Z      |                                                                     ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.6995900Z 
2019-10-11T03:26:06.7043759Z error[E0615]: attempted to take value of method `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.7044344Z     --> src/librustc_mir/borrow_check/mod.rs:1855:48
2019-10-11T03:26:06.7045295Z 1855 | ...                   location, this.body,
2019-10-11T03:26:06.7046293Z      |                                      ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.7046649Z 
2019-10-11T03:26:06.7046649Z 
2019-10-11T03:26:06.7164756Z error[E0615]: attempted to take value of method `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.7165586Z     --> src/librustc_mir/borrow_check/mod.rs:2059:35
2019-10-11T03:26:06.7166259Z      |
2019-10-11T03:26:06.7166768Z 2059 |                 let local = &self.body.local_decls[*local];
2019-10-11T03:26:06.7167306Z      |                                   ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.7167498Z 
2019-10-11T03:26:06.7219909Z error[E0615]: attempted to take value of method `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.7220447Z     --> src/librustc_mir/borrow_check/mod.rs:2120:72
2019-10-11T03:26:06.7220873Z      |
2019-10-11T03:26:06.7221373Z 2120 | ...                   Place::ty_from(place.base, proj_base, self.body, self.infcx.tcx).ty;
2019-10-11T03:26:06.7222548Z      |                                                                  ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.7222792Z 
2019-10-11T03:26:06.7312411Z error[E0615]: attempted to take value of method `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.7313111Z     --> src/librustc_mir/borrow_check/mod.rs:2260:73
2019-10-11T03:26:06.7313562Z      |
2019-10-11T03:26:06.7314080Z 2260 |                 let base_ty = Place::ty_from(place_ref.base, base, self.body, tcx).ty;
2019-10-11T03:26:06.7314720Z      |                                                                         ^^^^ help: use parentheses to call the method: `body()`
2019-10-11T03:26:06.8233326Z error[E0308]: mismatched types
2019-10-11T03:26:06.8233681Z    --> src/librustc_mir/borrow_check/borrow_set.rs:110:41
2019-10-11T03:26:06.8233906Z     |
2019-10-11T03:26:06.8233906Z     |
2019-10-11T03:26:06.8234216Z 110 |             has_storage_dead.visit_body(body);
2019-10-11T03:26:06.8234645Z     |                                         ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:06.8234869Z     |
2019-10-11T03:26:06.8235212Z     = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:06.8235666Z 
2019-10-11T03:26:06.8235666Z 
2019-10-11T03:26:06.8475029Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8475792Z   --> src/librustc_mir/borrow_check/error_reporting.rs:42:27
2019-10-11T03:26:06.8476179Z    |
2019-10-11T03:26:06.8476835Z 42 |         for stmt in &self.body[location.block].statements[location.statement_index..] {
2019-10-11T03:26:06.8477752Z 
2019-10-11T03:26:06.8477752Z 
2019-10-11T03:26:06.8533722Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8534585Z   --> src/librustc_mir/borrow_check/error_reporting.rs:56:31
2019-10-11T03:26:06.8535141Z    |
2019-10-11T03:26:06.8535636Z 56 |         let terminator = self.body[location.block].terminator();
2019-10-11T03:26:06.8536299Z 
2019-10-11T03:26:06.8536299Z 
2019-10-11T03:26:06.8604373Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8605035Z   --> src/librustc_mir/borrow_check/error_reporting.rs:79:51
2019-10-11T03:26:06.8605599Z    |
2019-10-11T03:26:06.8606053Z 79 |                 if let ty::Closure(did, _) = self.body.local_decls[closure].ty.kind {
2019-10-11T03:26:06.8606689Z 
2019-10-11T03:26:06.8606689Z 
2019-10-11T03:26:06.8651769Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8652659Z    --> src/librustc_mir/borrow_check/error_reporting.rs:102:47
2019-10-11T03:26:06.8653090Z     |
2019-10-11T03:26:06.8653637Z 102 |             if let ty::Closure(did, _) = self.body.local_decls[target].ty.kind {
2019-10-11T03:26:06.8654325Z 
2019-10-11T03:26:06.8654325Z 
2019-10-11T03:26:06.8741759Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8742972Z    --> src/librustc_mir/borrow_check/error_reporting.rs:211:49
2019-10-11T03:26:06.8743447Z     |
2019-10-11T03:26:06.8743956Z 211 | ...                   if self.body.local_decls[*local].is_ref_for_guard() {
2019-10-11T03:26:06.8744621Z 
2019-10-11T03:26:06.8744621Z 
2019-10-11T03:26:06.8851173Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8855554Z    --> src/librustc_mir/borrow_check/error_reporting.rs:336:27
2019-10-11T03:26:06.8856823Z     |
2019-10-11T03:26:06.8857181Z 336 |         let local = &self.body.local_decls[local_index];
2019-10-11T03:26:06.8858170Z 
2019-10-11T03:26:06.8858170Z 
2019-10-11T03:26:06.8863381Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8876386Z    --> src/librustc_mir/borrow_check/error_reporting.rs:354:35
2019-10-11T03:26:06.8877456Z     |
2019-10-11T03:26:06.8877896Z 354 |                 let local = &self.body.local_decls[*local];
2019-10-11T03:26:06.8878655Z 
2019-10-11T03:26:06.8878655Z 
2019-10-11T03:26:06.8904570Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.8953071Z    --> src/librustc_mir/borrow_check/error_reporting.rs:374:75
2019-10-11T03:26:06.8953730Z     |
2019-10-11T03:26:06.8954361Z 374 |                         Place::ty_from(place.base, place.projection, self.body, self.infcx.tcx).ty;
2019-10-11T03:26:06.8955143Z 
2019-10-11T03:26:06.8955143Z 
2019-10-11T03:26:06.9090397Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9091016Z    --> src/librustc_mir/borrow_check/error_reporting.rs:509:37
2019-10-11T03:26:06.9091620Z     |
2019-10-11T03:26:06.9092765Z 509 |                     let bbd = &self.body[loc.block];
2019-10-11T03:26:06.9093499Z 
2019-10-11T03:26:06.9093499Z 
2019-10-11T03:26:06.9136087Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9136683Z    --> src/librustc_mir/borrow_check/error_reporting.rs:527:77
2019-10-11T03:26:06.9137078Z     |
2019-10-11T03:26:06.9137554Z 527 | ...                   = BorrowedContentSource::from_call(func.ty(self.body, tcx), tcx)
2019-10-11T03:26:06.9138266Z 
2019-10-11T03:26:06.9138266Z 
2019-10-11T03:26:06.9173227Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9173861Z    --> src/librustc_mir/borrow_check/error_reporting.rs:540:83
2019-10-11T03:26:06.9174327Z     |
2019-10-11T03:26:06.9174853Z 540 |         let base_ty = Place::ty_from(deref_base.base, deref_base.projection, self.body, tcx).ty;
2019-10-11T03:26:06.9175709Z 
2019-10-11T03:26:06.9175709Z 
2019-10-11T03:26:06.9353111Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9353453Z    --> src/librustc_mir/borrow_check/error_reporting.rs:788:31
2019-10-11T03:26:06.9353716Z     |
2019-10-11T03:26:06.9354070Z 788 |         let stmt = match self.body[location.block].statements.get(location.statement_index) {
2019-10-11T03:26:06.9354422Z 
2019-10-11T03:26:06.9354422Z 
2019-10-11T03:26:06.9388027Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9388612Z    --> src/librustc_mir/borrow_check/error_reporting.rs:790:42
2019-10-11T03:26:06.9389053Z     |
2019-10-11T03:26:06.9389865Z 790 |             None => return OtherUse(self.body.source_info(location).span),
2019-10-11T03:26:06.9390534Z 
2019-10-11T03:26:06.9390534Z 
2019-10-11T03:26:06.9459309Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9459960Z    --> src/librustc_mir/borrow_check/error_reporting.rs:827:33
2019-10-11T03:26:06.9460351Z     |
2019-10-11T03:26:06.9460821Z 827 |         let target = match self.body[location.block]
2019-10-11T03:26:06.9461414Z 
2019-10-11T03:26:06.9461414Z 
2019-10-11T03:26:06.9497626Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9498216Z    --> src/librustc_mir/borrow_check/error_reporting.rs:841:17
2019-10-11T03:26:06.9498613Z     |
2019-10-11T03:26:06.9499113Z 841 |         if self.body.local_kind(target) != LocalKind::Temp {
2019-10-11T03:26:06.9499930Z 
2019-10-11T03:26:06.9499930Z 
2019-10-11T03:26:06.9534886Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9535479Z    --> src/librustc_mir/borrow_check/error_reporting.rs:846:27
2019-10-11T03:26:06.9535948Z     |
2019-10-11T03:26:06.9536481Z 846 |         for stmt in &self.body[location.block].statements[location.statement_index + 1..] {
2019-10-11T03:26:06.9537294Z 
2019-10-11T03:26:06.9537294Z 
2019-10-11T03:26:06.9650338Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:06.9651484Z    --> src/librustc_mir/borrow_check/error_reporting.rs:916:25
2019-10-11T03:26:06.9653846Z     |
2019-10-11T03:26:06.9655290Z 916 |         let span = self.body.source_info(borrow.reserve_location).span;
2019-10-11T03:26:06.9657640Z 
2019-10-11T03:26:06.9657640Z 
2019-10-11T03:26:07.0051470Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0054496Z     |
2019-10-11T03:26:07.0054496Z     |
2019-10-11T03:26:07.0055930Z 209 |                 Place::ty_from(used_place.base, used_place.projection, self.body, self.infcx.tcx)
2019-10-11T03:26:07.0058305Z 
2019-10-11T03:26:07.0058305Z 
2019-10-11T03:26:07.0102777Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0105483Z     |
2019-10-11T03:26:07.0105483Z     |
2019-10-11T03:26:07.0106656Z 225 |                 let ty = place.ty(self.body, self.infcx.tcx).ty;
2019-10-11T03:26:07.0108898Z 
2019-10-11T03:26:07.0108898Z 
2019-10-11T03:26:07.0158597Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0162006Z     |
2019-10-11T03:26:07.0162006Z     |
2019-10-11T03:26:07.0163354Z 247 |                     let decl = &self.body.local_decls[*local];
2019-10-11T03:26:07.0165755Z 
2019-10-11T03:26:07.0165755Z 
2019-10-11T03:26:07.0293834Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0296476Z     |
2019-10-11T03:26:07.0296476Z     |
2019-10-11T03:26:07.0298013Z 315 |         ).add_explanation_to_diagnostic(self.infcx.tcx, self.body, &mut err, "", Some(borrow_span));
2019-10-11T03:26:07.0300581Z 
2019-10-11T03:26:07.0300581Z 
2019-10-11T03:26:07.0364530Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0368230Z     |
2019-10-11T03:26:07.0368230Z     |
2019-10-11T03:26:07.0370122Z 350 |             .add_explanation_to_diagnostic(self.infcx.tcx, self.body, &mut err, "", None);
2019-10-11T03:26:07.0373216Z 
2019-10-11T03:26:07.0373216Z 
2019-10-11T03:26:07.0482404Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0486205Z     |
2019-10-11T03:26:07.0487644Z 551 |             self.body,
2019-10-11T03:26:07.0489288Z     |                  ^^^^
2019-10-11T03:26:07.0490465Z 
2019-10-11T03:26:07.0490465Z 
2019-10-11T03:26:07.0534778Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0538108Z     |
2019-10-11T03:26:07.0538108Z     |
2019-10-11T03:26:07.0539681Z 591 |             let ty = Place::ty_from(place_base, place_projection, self.body, self.infcx.tcx).ty;
2019-10-11T03:26:07.0543478Z 
2019-10-11T03:26:07.0543478Z 
2019-10-11T03:26:07.0672998Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0676782Z     |
2019-10-11T03:26:07.0676782Z     |
2019-10-11T03:26:07.0681824Z 710 |             PlaceBase::Local(local) => self.body.local_decls[*local].source_info.span,
2019-10-11T03:26:07.0682413Z 
2019-10-11T03:26:07.0682413Z 
2019-10-11T03:26:07.0846393Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0847761Z     |
2019-10-11T03:26:07.0848086Z 927 |                     self.body,
2019-10-11T03:26:07.0848369Z     |                          ^^^^
2019-10-11T03:26:07.0848409Z 
2019-10-11T03:26:07.0848409Z 
2019-10-11T03:26:07.0926429Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.0928851Z     |
2019-10-11T03:26:07.0928851Z     |
2019-10-11T03:26:07.0930139Z 952 |                 self.infcx.tcx, self.body, &mut err, "", None);
2019-10-11T03:26:07.0930832Z 
2019-10-11T03:26:07.0930832Z 
2019-10-11T03:26:07.1079197Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1079876Z      |
2019-10-11T03:26:07.1079876Z      |
2019-10-11T03:26:07.1080259Z 1010 |         explanation.add_explanation_to_diagnostic(self.infcx.tcx, self.body, &mut err, "", None);
2019-10-11T03:26:07.1080668Z 
2019-10-11T03:26:07.1080668Z 
2019-10-11T03:26:07.1176129Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1176665Z      |
2019-10-11T03:26:07.1176665Z      |
2019-10-11T03:26:07.1177008Z 1090 |         explanation.add_explanation_to_diagnostic(self.infcx.tcx, self.body, &mut err, "", None);
2019-10-11T03:26:07.1177366Z 
2019-10-11T03:26:07.1177366Z 
2019-10-11T03:26:07.1227610Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1228167Z      |
2019-10-11T03:26:07.1228167Z      |
2019-10-11T03:26:07.1228706Z 1121 |         let reference_desc = if return_span == self.body.source_info(borrow.reserve_location).span {
2019-10-11T03:26:07.1229224Z 
2019-10-11T03:26:07.1229224Z 
2019-10-11T03:26:07.1269222Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1269994Z      |
2019-10-11T03:26:07.1269994Z      |
2019-10-11T03:26:07.1270312Z 1133 |                     match self.body.local_kind(local) {
2019-10-11T03:26:07.1270627Z 
2019-10-11T03:26:07.1270627Z 
2019-10-11T03:26:07.1327896Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1328487Z      |
2019-10-11T03:26:07.1328487Z      |
2019-10-11T03:26:07.1328946Z 1166 |             match self.body.local_kind(*local) {
2019-10-11T03:26:07.1329230Z 
2019-10-11T03:26:07.1401556Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-10-11T03:26:07.1402320Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1127:14
2019-10-11T03:26:07.1402551Z      |
2019-10-11T03:26:07.1402551Z      |
2019-10-11T03:26:07.1402920Z 1127 |         let (place_desc, note) = if let Some(place_desc) = opt_place_desc {
2019-10-11T03:26:07.1403273Z      |              ^^^^^^^^^^ doesn't have a size known at compile-time
2019-10-11T03:26:07.1403820Z      = help: the trait `std::marker::Sized` is not implemented for `str`
2019-10-11T03:26:07.1403820Z      = help: the trait `std::marker::Sized` is not implemented for `str`
2019-10-11T03:26:07.1404190Z      = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-10-11T03:26:07.1404519Z      = note: all local variables must have a statically known size
2019-10-11T03:26:07.1404849Z 
2019-10-11T03:26:07.1404849Z 
2019-10-11T03:26:07.1527209Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1527777Z      |
2019-10-11T03:26:07.1528035Z 1316 |         let body = self.body;
2019-10-11T03:26:07.1528291Z      |                         ^^^^
2019-10-11T03:26:07.1528337Z 
2019-10-11T03:26:07.1528337Z 
2019-10-11T03:26:07.1617677Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1618217Z      |
2019-10-11T03:26:07.1618523Z 1393 |                 self.body,
2019-10-11T03:26:07.1618789Z      |                      ^^^^
2019-10-11T03:26:07.1618827Z 
2019-10-11T03:26:07.1618827Z 
2019-10-11T03:26:07.1694470Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1695030Z      |
2019-10-11T03:26:07.1695030Z      |
2019-10-11T03:26:07.1695827Z 1453 |             .add_explanation_to_diagnostic(self.infcx.tcx, self.body, &mut err, "", None);
2019-10-11T03:26:07.1696178Z 
2019-10-11T03:26:07.1696178Z 
2019-10-11T03:26:07.1736297Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1736891Z      |
2019-10-11T03:26:07.1736891Z      |
2019-10-11T03:26:07.1737174Z 1475 |             if let LocalKind::Arg = self.body.local_kind(local) {
2019-10-11T03:26:07.1737484Z 
2019-10-11T03:26:07.1737484Z 
2019-10-11T03:26:07.1773271Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1773856Z      |
2019-10-11T03:26:07.1773856Z      |
2019-10-11T03:26:07.1774221Z 1476 |                 (true, Some(&self.body.local_decls[local]))
2019-10-11T03:26:07.1774565Z 
2019-10-11T03:26:07.1774565Z 
2019-10-11T03:26:07.1810316Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1810851Z      |
2019-10-11T03:26:07.1810851Z      |
2019-10-11T03:26:07.1811320Z 1478 |                 (false, Some(&self.body.local_decls[local]))
2019-10-11T03:26:07.1812207Z 
2019-10-11T03:26:07.1812207Z 
2019-10-11T03:26:07.1885561Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1886114Z      |
2019-10-11T03:26:07.1886114Z      |
2019-10-11T03:26:07.1886430Z 1560 | ...                   Place::ty_from(&place.base, proj_base, self.body, tcx).ty.is_box(),
2019-10-11T03:26:07.1886795Z 
2019-10-11T03:26:07.1886795Z 
2019-10-11T03:26:07.1924528Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1925121Z      |
2019-10-11T03:26:07.1925121Z      |
2019-10-11T03:26:07.1925607Z 1568 |                         let base_ty = Place::ty_from(&place.base, proj_base, self.body, tcx).ty;
2019-10-11T03:26:07.1925967Z 
2019-10-11T03:26:07.1925967Z 
2019-10-11T03:26:07.1981152Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.1982551Z      |
2019-10-11T03:26:07.1982551Z      |
2019-10-11T03:26:07.1982886Z 1627 |              = &self.body[location.block].statements.get(location.statement_index)
2019-10-11T03:26:07.1983218Z 
2019-10-11T03:26:07.1983218Z 
2019-10-11T03:26:07.2025170Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.2026007Z      |
2019-10-11T03:26:07.2026007Z      |
2019-10-11T03:26:07.2026304Z 1638 |                 } if self.body.local_kind(*local) == LocalKind::Temp => local,
2019-10-11T03:26:07.2026617Z 
2019-10-11T03:26:07.2026617Z 
2019-10-11T03:26:07.2065587Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.2066114Z      |
2019-10-11T03:26:07.2066114Z      |
2019-10-11T03:26:07.2066648Z 1645 |             for stmt in &self.body[location.block].statements[location.statement_index + 1..] {
2019-10-11T03:26:07.2066971Z 
2019-10-11T03:26:07.2137418Z error[E0614]: type `rustc::hir::def_id::DefId` cannot be dereferenced
2019-10-11T03:26:07.2137736Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1696:33
2019-10-11T03:26:07.2137998Z      |
2019-10-11T03:26:07.2137998Z      |
2019-10-11T03:26:07.2138264Z 1696 | ...                   *def_id,
2019-10-11T03:26:07.2138523Z      |                       ^^^^^^^
2019-10-11T03:26:07.2138576Z 
2019-10-11T03:26:07.2173473Z error[E0614]: type `rustc::hir::def_id::DefId` cannot be dereferenced
2019-10-11T03:26:07.2173806Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1697:56
2019-10-11T03:26:07.2174058Z      |
2019-10-11T03:26:07.2174376Z 1697 | ...                   self.infcx.closure_sig(*def_id, *substs),
2019-10-11T03:26:07.2174718Z 
2019-10-11T03:26:07.2233312Z error[E0308]: mismatched types
2019-10-11T03:26:07.2233644Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1697:65
2019-10-11T03:26:07.2233899Z      |
2019-10-11T03:26:07.2233899Z      |
2019-10-11T03:26:07.2234211Z 1697 | ...                   self.infcx.closure_sig(*def_id, *substs),
2019-10-11T03:26:07.2234868Z      |                                                       |
2019-10-11T03:26:07.2235628Z      |                                                       expected reference, found struct `rustc::ty::List`
2019-10-11T03:26:07.2235628Z      |                                                       expected reference, found struct `rustc::ty::List`
2019-10-11T03:26:07.2235951Z      |                                                       help: consider borrowing here: `&*substs`
2019-10-11T03:26:07.2236413Z      |
2019-10-11T03:26:07.2236715Z      = note: expected type `&rustc::ty::List<rustc::ty::subst::GenericArg<'_>>`
2019-10-11T03:26:07.2237094Z                 found type `rustc::ty::List<rustc::ty::subst::GenericArg<'_>>`
2019-10-11T03:26:07.2278567Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.2278931Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1706:32
2019-10-11T03:26:07.2279167Z      |
2019-10-11T03:26:07.2279167Z      |
2019-10-11T03:26:07.2279428Z 1706 | ...                   if *assigned_to == mir::RETURN_PLACE {
2019-10-11T03:26:07.2279746Z 
2019-10-11T03:26:07.2309292Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.2309601Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1712:42
2019-10-11T03:26:07.2309856Z      |
2019-10-11T03:26:07.2309856Z      |
2019-10-11T03:26:07.2310131Z 1712 | ...                   target = *assigned_to;
2019-10-11T03:26:07.2310455Z 
2019-10-11T03:26:07.2363462Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.2363806Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1762:24
2019-10-11T03:26:07.2364090Z      |
2019-10-11T03:26:07.2364090Z      |
2019-10-11T03:26:07.2364393Z 1762 |                     if *assigned_to == mir::RETURN_PLACE {
2019-10-11T03:26:07.2364731Z 
2019-10-11T03:26:07.2394760Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.2395098Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1770:30
2019-10-11T03:26:07.2395513Z      |
2019-10-11T03:26:07.2395513Z      |
2019-10-11T03:26:07.2396102Z 1770 |                     target = *assigned_to;
2019-10-11T03:26:07.2396405Z 
2019-10-11T03:26:07.2396405Z 
2019-10-11T03:26:07.2431804Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.2432621Z      |
2019-10-11T03:26:07.2432621Z      |
2019-10-11T03:26:07.2432955Z 1775 |             let terminator = &self.body[location.block].terminator();
2019-10-11T03:26:07.2433306Z 
2019-10-11T03:26:07.2503361Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.2503708Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1811:28
2019-10-11T03:26:07.2503968Z      |
2019-10-11T03:26:07.2503968Z      |
2019-10-11T03:26:07.2504310Z 1811 |                         if *assigned_to == mir::RETURN_PLACE && assigned_from_local == target {
2019-10-11T03:26:07.2504896Z 
2019-10-11T03:26:07.2504896Z 
---
2019-10-11T03:26:07.3013835Z     |
2019-10-11T03:26:07.3014143Z 125 | ...                   *local,
2019-10-11T03:26:07.3014449Z     |                       ^^^^^^
2019-10-11T03:26:07.3014516Z 
2019-10-11T03:26:07.3225917Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3226314Z    --> src/librustc_mir/borrow_check/move_errors.rs:306:45
2019-10-11T03:26:07.3226645Z     |
2019-10-11T03:26:07.3227009Z 306 |         let ty = deref_target_place.ty(self.body, self.infcx.tcx).ty;
2019-10-11T03:26:07.3227857Z 
2019-10-11T03:26:07.3227857Z 
2019-10-11T03:26:07.3279380Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3279729Z    --> src/librustc_mir/borrow_check/move_errors.rs:324:30
2019-10-11T03:26:07.3279998Z     |
2019-10-11T03:26:07.3280311Z 324 |             let decl = &self.body.local_decls[*local];
2019-10-11T03:26:07.3280680Z 
2019-10-11T03:26:07.3280680Z 
2019-10-11T03:26:07.3406489Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3406870Z    --> src/librustc_mir/borrow_check/move_errors.rs:415:32
2019-10-11T03:26:07.3407164Z     |
2019-10-11T03:26:07.3407521Z 415 |             move_place.ty(self.body, self.infcx.tcx).ty,
2019-10-11T03:26:07.3407911Z 
2019-10-11T03:26:07.3407911Z 
2019-10-11T03:26:07.3486466Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3486873Z    --> src/librustc_mir/borrow_check/move_errors.rs:458:54
2019-10-11T03:26:07.3487208Z     |
2019-10-11T03:26:07.3487594Z 458 |                     let place_ty = move_from.ty(self.body, self.infcx.tcx).ty;
2019-10-11T03:26:07.3488013Z 
2019-10-11T03:26:07.3488013Z 
2019-10-11T03:26:07.3548637Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3549021Z    --> src/librustc_mir/borrow_check/move_errors.rs:486:54
2019-10-11T03:26:07.3549515Z     |
2019-10-11T03:26:07.3550273Z 486 |                 let place_ty = original_path.ty(self.body, self.infcx.tcx).ty;
2019-10-11T03:26:07.3551085Z 
2019-10-11T03:26:07.3551085Z 
2019-10-11T03:26:07.3622310Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3622757Z    --> src/librustc_mir/borrow_check/move_errors.rs:514:33
2019-10-11T03:26:07.3623078Z     |
2019-10-11T03:26:07.3623669Z 514 |             let bind_to = &self.body.local_decls[*local];
2019-10-11T03:26:07.3624177Z 
2019-10-11T03:26:07.3624177Z 
2019-10-11T03:26:07.3715418Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3715788Z    --> src/librustc_mir/borrow_check/move_errors.rs:564:33
2019-10-11T03:26:07.3716100Z     |
2019-10-11T03:26:07.3716442Z 564 |             let bind_to = &self.body.local_decls[*local];
2019-10-11T03:26:07.3716801Z 
2019-10-11T03:26:07.3716801Z 
2019-10-11T03:26:07.3826423Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3826831Z   --> src/librustc_mir/borrow_check/mutability_errors.rs:58:37
2019-10-11T03:26:07.3827146Z    |
2019-10-11T03:26:07.3827505Z 58 |                     let name = self.body.local_decls[*local]
2019-10-11T03:26:07.3827883Z 
2019-10-11T03:26:07.3827883Z 
2019-10-11T03:26:07.3876123Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3876475Z   --> src/librustc_mir/borrow_check/mutability_errors.rs:70:73
2019-10-11T03:26:07.3876750Z    |
2019-10-11T03:26:07.3877121Z 70 |                     Place::ty_from(&the_place_err.base, proj_base, self.body, self.infcx.tcx).ty
2019-10-11T03:26:07.3877532Z 
2019-10-11T03:26:07.3877532Z 
2019-10-11T03:26:07.3944813Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3945189Z   --> src/librustc_mir/borrow_check/mutability_errors.rs:90:40
2019-10-11T03:26:07.3945503Z    |
2019-10-11T03:26:07.3945878Z 90 |                     debug_assert!(self.body.local_decls[Local::new(1)].ty.is_region_ptr());
2019-10-11T03:26:07.3946268Z 
2019-10-11T03:26:07.3946268Z 
2019-10-11T03:26:07.3995041Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.3995563Z   --> src/librustc_mir/borrow_check/mutability_errors.rs:95:34
2019-10-11T03:26:07.3996120Z 95 | ...                   self.body,
2019-10-11T03:26:07.3996404Z    |                            ^^^^
2019-10-11T03:26:07.3996445Z 
2019-10-11T03:26:07.3996445Z 
2019-10-11T03:26:07.4043297Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4043675Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:109:30
2019-10-11T03:26:07.4043964Z     |
2019-10-11T03:26:07.4044538Z 109 |                         self.body.local_decls[*local].is_ref_for_guard()
2019-10-11T03:26:07.4044984Z 
2019-10-11T03:26:07.4044984Z 
2019-10-11T03:26:07.4185573Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4185927Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:245:58
2019-10-11T03:26:07.4186195Z     |
2019-10-11T03:26:07.4186560Z 245 |                     Place::ty_from(base, proj_base, self.body, self.infcx.tcx).ty,
2019-10-11T03:26:07.4186918Z 
2019-10-11T03:26:07.4186918Z 
2019-10-11T03:26:07.4228904Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4229273Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:262:22
2019-10-11T03:26:07.4229552Z     |
2019-10-11T03:26:07.4229891Z 262 |                 self.body.local_decls.get(*local).map(|local_decl| {
2019-10-11T03:26:07.4230395Z 
2019-10-11T03:26:07.4230395Z 
2019-10-11T03:26:07.4263003Z error[E0614]: type `rustc::mir::ImplicitSelfKind` cannot be dereferenced
2019-10-11T03:26:07.4263374Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:271:25
2019-10-11T03:26:07.4263660Z     |
2019-10-11T03:26:07.4264021Z 271 |                         *kind == mir::ImplicitSelfKind::MutRef
2019-10-11T03:26:07.4264378Z 
2019-10-11T03:26:07.4264378Z 
2019-10-11T03:26:07.4315882Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4316233Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:297:23
2019-10-11T03:26:07.4316470Z     |
2019-10-11T03:26:07.4316830Z 297 |             } if self.body.local_decls[*local].can_be_made_mutable() => {
2019-10-11T03:26:07.4317181Z 
2019-10-11T03:26:07.4317181Z 
2019-10-11T03:26:07.4354762Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4355195Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:302:40
2019-10-11T03:26:07.4355491Z     |
2019-10-11T03:26:07.4356247Z 302 |                 let local_decl = &self.body.local_decls[*local];
2019-10-11T03:26:07.4356788Z 
2019-10-11T03:26:07.4356788Z 
2019-10-11T03:26:07.4419387Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4420021Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:320:58
2019-10-11T03:26:07.4420267Z     |
2019-10-11T03:26:07.4420637Z 320 |                     Place::ty_from(base, proj_base, self.body, self.infcx.tcx).ty
2019-10-11T03:26:07.4421166Z 
2019-10-11T03:26:07.4421166Z 
2019-10-11T03:26:07.4496135Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4496505Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:368:26
2019-10-11T03:26:07.4496761Z     |
2019-10-11T03:26:07.4497125Z 368 |                     self.body.local_decls[*local].is_user_variable
2019-10-11T03:26:07.4497497Z 
2019-10-11T03:26:07.4497497Z 
2019-10-11T03:26:07.4546115Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4546493Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:390:23
2019-10-11T03:26:07.4546751Z     |
2019-10-11T03:26:07.4547128Z 390 |             } if self.body.local_decls[*local].is_user_variable.is_some() =>
2019-10-11T03:26:07.4547467Z 
2019-10-11T03:26:07.4547467Z 
2019-10-11T03:26:07.4587086Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4587495Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:392:40
2019-10-11T03:26:07.4587766Z     |
2019-10-11T03:26:07.4588155Z 392 |                 let local_decl = &self.body.local_decls[*local];
2019-10-11T03:26:07.4588536Z 
2019-10-11T03:26:07.4588536Z 
2019-10-11T03:26:07.4638919Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4639246Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:404:30
2019-10-11T03:26:07.4639769Z 404 |                         self.body,
2019-10-11T03:26:07.4640059Z     |                              ^^^^
2019-10-11T03:26:07.4640098Z 
2019-10-11T03:26:07.4640098Z 
2019-10-11T03:26:07.4675487Z error[E0614]: type `std::option::Option<syntax_pos::span_encoding::Span>` cannot be dereferenced
2019-10-11T03:26:07.4675840Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:407:25
2019-10-11T03:26:07.4676080Z     |
2019-10-11T03:26:07.4676399Z 407 |                         *opt_ty_info,
2019-10-11T03:26:07.4676719Z 
2019-10-11T03:26:07.4676996Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-10-11T03:26:07.4676996Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-10-11T03:26:07.4808856Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'a, 'tcx>`
2019-10-11T03:26:07.4809507Z    --> src/librustc_mir/borrow_check/mutability_errors.rs:476:26
2019-10-11T03:26:07.4809789Z     |
2019-10-11T03:26:07.4810071Z 476 |                     self.body.span,
2019-10-11T03:26:07.4810589Z 
2019-10-11T03:26:07.4810589Z 
2019-10-11T03:26:07.5672184Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.5672637Z   --> src/librustc_mir/borrow_check/prefixes.rs:59:24
2019-10-11T03:26:07.5673230Z 59 |             body: self.body,
2019-10-11T03:26:07.5673551Z    |                        ^^^^
2019-10-11T03:26:07.5673611Z 
2019-10-11T03:26:07.5673611Z 
2019-10-11T03:26:07.5774854Z error[E0609]: no field `body` on type `&mut borrow_check::MirBorrowckCtxt<'_, '_>`
2019-10-11T03:26:07.5775228Z   --> src/librustc_mir/borrow_check/used_muts.rs:37:45
2019-10-11T03:26:07.5775669Z    |
2019-10-11T03:26:07.5775961Z 37 |             visitor.visit_body(visitor.mbcx.body);
2019-10-11T03:26:07.5776303Z 
2019-10-11T03:26:07.5776303Z 
2019-10-11T03:26:07.6553080Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6553485Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:237:25
2019-10-11T03:26:07.6554049Z 237 |         let body = self.body;
2019-10-11T03:26:07.6554352Z     |                         ^^^^
2019-10-11T03:26:07.6554392Z 
2019-10-11T03:26:07.6554392Z 
2019-10-11T03:26:07.6621206Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6623017Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:300:34
2019-10-11T03:26:07.6623585Z 300 | ...                   self.body,
2019-10-11T03:26:07.6623856Z     |                            ^^^^
2019-10-11T03:26:07.6623914Z 
2019-10-11T03:26:07.6623914Z 
2019-10-11T03:26:07.6712864Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6715104Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:367:31
2019-10-11T03:26:07.6715416Z     |
2019-10-11T03:26:07.6715755Z 367 |             let block = &self.body.basic_blocks()[location.block];
2019-10-11T03:26:07.6716111Z 
2019-10-11T03:26:07.6716111Z 
2019-10-11T03:26:07.6774123Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6774469Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:429:31
2019-10-11T03:26:07.6774732Z     |
2019-10-11T03:26:07.6775288Z 429 |             let block = &self.body.basic_blocks()[from.block];
2019-10-11T03:26:07.6775934Z 
2019-10-11T03:26:07.6775934Z 
2019-10-11T03:26:07.6822854Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6823400Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:461:40
2019-10-11T03:26:07.6823721Z     |
2019-10-11T03:26:07.6824024Z 461 |         target.dominates(source, &self.body.dominators())
2019-10-11T03:26:07.6824375Z 
2019-10-11T03:26:07.6824375Z 
2019-10-11T03:26:07.6861906Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6862469Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:477:35
2019-10-11T03:26:07.6862730Z     |
2019-10-11T03:26:07.6863047Z 477 |                 let block = &self.body.basic_blocks()[location.block];
2019-10-11T03:26:07.6863404Z 
2019-10-11T03:26:07.6863404Z 
2019-10-11T03:26:07.6904586Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6904926Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:505:56
2019-10-11T03:26:07.6905183Z     |
2019-10-11T03:26:07.6905706Z 505 | ...                   let local_decl = &self.body.local_decls[*l];
2019-10-11T03:26:07.6906070Z 
2019-10-11T03:26:07.6906070Z 
2019-10-11T03:26:07.6946469Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.6946807Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:533:27
2019-10-11T03:26:07.6947058Z     |
2019-10-11T03:26:07.6947513Z 533 |         let block = &self.body[location.block];
2019-10-11T03:26:07.6947838Z 
2019-10-11T03:26:07.6947838Z 
2019-10-11T03:26:07.7057299Z error[E0609]: no field `body` on type `&borrow_check::MirBorrowckCtxt<'cx, 'tcx>`
2019-10-11T03:26:07.7057645Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:563:31
2019-10-11T03:26:07.7057892Z     |
2019-10-11T03:26:07.7058184Z 563 |             let block = &self.body[current_location.block];
2019-10-11T03:26:07.7058523Z 
2019-10-11T03:26:07.7096934Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.7097241Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:593:36
2019-10-11T03:26:07.7097473Z     |
2019-10-11T03:26:07.7097473Z     |
2019-10-11T03:26:07.7097735Z 593 | ...                   if *from == target =>
2019-10-11T03:26:07.7098259Z 
2019-10-11T03:26:07.7127816Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.7128112Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:612:36
2019-10-11T03:26:07.7128498Z     |
2019-10-11T03:26:07.7128498Z     |
2019-10-11T03:26:07.7128805Z 612 | ...                   if *from == target =>
2019-10-11T03:26:07.7129081Z 
2019-10-11T03:26:07.7192359Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-10-11T03:26:07.7192700Z    --> src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:667:34
2019-10-11T03:26:07.7192979Z     |
2019-10-11T03:26:07.7192979Z     |
2019-10-11T03:26:07.7193268Z 667 |                         target = *dest;
2019-10-11T03:26:07.7193588Z 
2019-10-11T03:26:07.7643366Z error[E0599]: no method named `dominators` found for type `&rustc::mir::Body<'tcx>` in the current scope
2019-10-11T03:26:07.7643717Z   --> src/librustc_mir/borrow_check/nll/invalidation.rs:34:31
2019-10-11T03:26:07.7643987Z    |
2019-10-11T03:26:07.7643987Z    |
2019-10-11T03:26:07.7644324Z 34 |         let dominators = body.dominators();
2019-10-11T03:26:07.7644711Z    |                               ^^^^^^^^^^ method not found in `&rustc::mir::Body<'tcx>`
2019-10-11T03:26:07.8507391Z error[E0308]: mismatched types
2019-10-11T03:26:07.8507771Z   --> src/librustc_mir/borrow_check/nll/invalidation.rs:44:23
2019-10-11T03:26:07.8508044Z    |
2019-10-11T03:26:07.8508044Z    |
2019-10-11T03:26:07.8508510Z 44 |         ig.visit_body(body);
2019-10-11T03:26:07.8508843Z    |                       ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:07.8509060Z    |
2019-10-11T03:26:07.8509340Z    = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:07.8509625Z 
2019-10-11T03:26:08.0220469Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-11T03:26:08.2055958Z error[E0599]: no method named `predecessors_for` found for type `&rustc::mir::Body<'_>` in the current scope
2019-10-11T03:26:08.2056479Z    --> src/librustc_mir/borrow_check/nll/region_infer/values.rs:107:22
2019-10-11T03:26:08.2056479Z    --> src/librustc_mir/borrow_check/nll/region_infer/values.rs:107:22
2019-10-11T03:26:08.2056723Z     |
2019-10-11T03:26:08.2056995Z 107 |                 body.predecessors_for(block)
2019-10-11T03:26:08.2057342Z     |                      ^^^^^^^^^^^^^^^^ method not found in `&rustc::mir::Body<'_>`
2019-10-11T03:26:08.3440876Z error[E0308]: mismatched types
2019-10-11T03:26:08.3442305Z   --> src/librustc_mir/borrow_check/nll/renumber.rs:21:28
2019-10-11T03:26:08.3442662Z    |
2019-10-11T03:26:08.3442662Z    |
2019-10-11T03:26:08.3442978Z 21 |         visitor.visit_body(body);
2019-10-11T03:26:08.3443617Z    |                            ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:08.3443952Z    |
2019-10-11T03:26:08.3444341Z    = note: expected type `&mut rustc::mir::BodyCache<&mut rustc::mir::Body<'_>>`
2019-10-11T03:26:08.3444699Z 
2019-10-11T03:26:08.4387282Z error[E0308]: mismatched types
2019-10-11T03:26:08.4387726Z   --> src/librustc_mir/borrow_check/nll/renumber.rs:24:24
2019-10-11T03:26:08.4387964Z    |
2019-10-11T03:26:08.4387964Z    |
2019-10-11T03:26:08.4388255Z 24 |     visitor.visit_body(body);
2019-10-11T03:26:08.4388642Z    |                        ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:08.4389110Z    |
2019-10-11T03:26:08.4389493Z    = note: expected type `&mut rustc::mir::BodyCache<&mut rustc::mir::Body<'_>>`
2019-10-11T03:26:08.4389832Z 
2019-10-11T03:26:08.5506030Z error[E0308]: mismatched types
2019-10-11T03:26:08.5506476Z    --> src/librustc_mir/borrow_check/nll/type_check/mod.rs:204:29
2019-10-11T03:26:08.5506761Z     |
2019-10-11T03:26:08.5506761Z     |
2019-10-11T03:26:08.5507035Z 204 |         verifier.visit_body(body);
2019-10-11T03:26:08.5507413Z     |                             ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:08.5507825Z     |
2019-10-11T03:26:08.5508147Z     = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:08.5508495Z 
2019-10-11T03:26:08.6904818Z error[E0308]: mismatched types
2019-10-11T03:26:08.6905250Z    --> src/librustc_mir/borrow_check/nll/type_check/mod.rs:569:25
2019-10-11T03:26:08.6905722Z     |
2019-10-11T03:26:08.6905722Z     |
2019-10-11T03:26:08.6906021Z 569 |         self.visit_body(promoted_body);
2019-10-11T03:26:08.6906420Z     |                         ^^^^^^^^^^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:08.6906702Z     |
2019-10-11T03:26:08.6907032Z     = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:08.6907368Z 
2019-10-11T03:26:09.0896507Z error[E0308]: mismatched types
2019-10-11T03:26:09.0897333Z   --> src/librustc_mir/borrow_check/nll/type_check/liveness/local_use_map.rs:82:25
2019-10-11T03:26:09.0897568Z    |
2019-10-11T03:26:09.0897568Z    |
2019-10-11T03:26:09.0897841Z 82 |             .visit_body(body);
2019-10-11T03:26:09.0898359Z    |                         ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:09.0898615Z    |
2019-10-11T03:26:09.0898923Z    = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:09.0899201Z               found type `&rustc::mir::Body<'_>`
2019-10-11T03:26:09.1846140Z error[E0308]: mismatched types
2019-10-11T03:26:09.1846483Z    --> src/librustc_mir/borrow_check/nll/type_check/liveness/polonius.rs:116:21
2019-10-11T03:26:09.1846683Z     |
2019-10-11T03:26:09.1846683Z     |
2019-10-11T03:26:09.1846937Z 116 |         .visit_body(body);
2019-10-11T03:26:09.1847269Z     |                     ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:09.1847471Z     |
2019-10-11T03:26:09.1847770Z     = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:09.1848028Z 
2019-10-11T03:26:09.1848028Z 
2019-10-11T03:26:09.2123446Z error[E0599]: no method named `predecessors_for` found for type `&'me rustc::mir::Body<'tcx>` in the current scope
2019-10-11T03:26:09.2123827Z    --> src/librustc_mir/borrow_check/nll/type_check/liveness/trace.rs:305:41
2019-10-11T03:26:09.2124055Z     |
2019-10-11T03:26:09.2124417Z 305 |         for &pred_block in self.cx.body.predecessors_for(block).iter() {
2019-10-11T03:26:09.2124801Z     |                                         ^^^^^^^^^^^^^^^^ method not found in `&'me rustc::mir::Body<'tcx>`
2019-10-11T03:26:09.5068205Z error[E0308]: mismatched types
2019-10-11T03:26:09.5068554Z    --> src/librustc_codegen_ssa/mir/block.rs:166:70
2019-10-11T03:26:09.5068771Z     |
2019-10-11T03:26:09.5068771Z     |
2019-10-11T03:26:09.5069100Z 166 |                         .is_predecessor_of(self.bb.start_location(), mir)
2019-10-11T03:26:09.5069492Z     |                                                                      ^^^ expected struct `rustc::mir::BodyCache`, found reference
2019-10-11T03:26:09.5069712Z     |
2019-10-11T03:26:09.5070023Z     = note: expected type `rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:09.5070302Z 
2019-10-11T03:26:09.6659825Z error[E0308]: mismatched types
2019-10-11T03:26:09.6660155Z    --> src/librustc_codegen_ssa/mir/block.rs:220:41
2019-10-11T03:26:09.6660607Z     |
2019-10-11T03:26:09.6660607Z     |
2019-10-11T03:26:09.6660909Z 220 |                 helper.maybe_sideeffect(self.mir, &mut bx, targets.as_slice());
2019-10-11T03:26:09.6661611Z     |
2019-10-11T03:26:09.6661935Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:09.6661935Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:09.6662973Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:09.8129759Z error[E0308]: mismatched types
2019-10-11T03:26:09.8130094Z    --> src/librustc_codegen_ssa/mir/block.rs:234:41
2019-10-11T03:26:09.8130348Z     |
2019-10-11T03:26:09.8130348Z     |
2019-10-11T03:26:09.8130709Z 234 |                 helper.maybe_sideeffect(self.mir, &mut bx, targets.as_slice());
2019-10-11T03:26:09.8131332Z     |
2019-10-11T03:26:09.8131615Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:09.8131615Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:09.8132488Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:09.9618559Z error[E0308]: mismatched types
2019-10-11T03:26:09.9619146Z    --> src/librustc_codegen_ssa/mir/block.rs:238:37
2019-10-11T03:26:09.9619445Z     |
2019-10-11T03:26:09.9619445Z     |
2019-10-11T03:26:09.9619780Z 238 |             helper.maybe_sideeffect(self.mir, &mut bx, targets.as_slice());
2019-10-11T03:26:09.9620742Z     |
2019-10-11T03:26:09.9621069Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:09.9621069Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:09.9621434Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:10.1366520Z error[E0308]: mismatched types
2019-10-11T03:26:10.1366926Z    --> src/librustc_codegen_ssa/mir/block.rs:334:37
2019-10-11T03:26:10.1367160Z     |
2019-10-11T03:26:10.1367160Z     |
2019-10-11T03:26:10.1367491Z 334 |             helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:10.1368118Z     |
2019-10-11T03:26:10.1368833Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.1368833Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.1369135Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:10.1869683Z error[E0308]: mismatched types
2019-10-11T03:26:10.1869978Z    --> src/librustc_mir/dataflow/impls/storage_liveness.rs:188:32
2019-10-11T03:26:10.1870458Z     |
2019-10-11T03:26:10.1870458Z     |
2019-10-11T03:26:10.1870719Z 188 |         visitor.visit_location(self.body, loc);
2019-10-11T03:26:10.1872426Z     |                                ^^^^^^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:10.1872779Z     |
2019-10-11T03:26:10.1873112Z     = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:10.1873413Z                found type `&'mir rustc::mir::Body<'tcx>`
2019-10-11T03:26:10.2902311Z error[E0308]: mismatched types
2019-10-11T03:26:10.2902691Z    --> src/librustc_codegen_ssa/mir/block.rs:365:33
2019-10-11T03:26:10.2902932Z     |
2019-10-11T03:26:10.2902932Z     |
2019-10-11T03:26:10.2903276Z 365 |         helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:10.2903925Z     |
2019-10-11T03:26:10.2904225Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.2904225Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.2904571Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:10.4403079Z error[E0308]: mismatched types
2019-10-11T03:26:10.4403754Z    --> src/librustc_codegen_ssa/mir/block.rs:401:37
2019-10-11T03:26:10.4404160Z     |
2019-10-11T03:26:10.4404160Z     |
2019-10-11T03:26:10.4404723Z 401 |             helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:10.4405848Z     |
2019-10-11T03:26:10.4406295Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.4406295Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.4406813Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:10.5760448Z error[E0308]: mismatched types
2019-10-11T03:26:10.5761476Z    --> src/librustc_codegen_ssa/mir/block.rs:412:33
2019-10-11T03:26:10.5761881Z     |
2019-10-11T03:26:10.5761881Z     |
2019-10-11T03:26:10.5762930Z 412 |         helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:10.5763959Z     |
2019-10-11T03:26:10.5764418Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.5764418Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.5764911Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:10.7328372Z error[E0308]: mismatched types
2019-10-11T03:26:10.7329415Z    --> src/librustc_codegen_ssa/mir/block.rs:516:41
2019-10-11T03:26:10.7329856Z     |
2019-10-11T03:26:10.7329856Z     |
2019-10-11T03:26:10.7330545Z 516 |                 helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:10.7332354Z     |
2019-10-11T03:26:10.7332912Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.7332912Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.7333434Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:10.8888223Z error[E0308]: mismatched types
2019-10-11T03:26:10.8888587Z    --> src/librustc_codegen_ssa/mir/block.rs:544:41
2019-10-11T03:26:10.8888900Z     |
2019-10-11T03:26:10.8888900Z     |
2019-10-11T03:26:10.8889238Z 544 |                 helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:10.8889913Z     |
2019-10-11T03:26:10.8890244Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.8890244Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:10.8890597Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:10.8890648Z 
2019-10-11T03:26:10.9352931Z error[E0599]: no method named `basic_blocks_mut` found for type `rustc::mir::Body<'_>` in the current scope
2019-10-11T03:26:10.9353439Z    --> src/librustc_mir/shim.rs:218:18
2019-10-11T03:26:10.9353703Z     |
2019-10-11T03:26:10.9354100Z 218 |             body.basic_blocks_mut()[START_BLOCK].statements.insert(0, Statement {
2019-10-11T03:26:10.9354509Z     |                  ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `basic_blocks`
2019-10-11T03:26:10.9354923Z 
2019-10-11T03:26:11.0210039Z error[E0599]: no method named `basic_blocks_and_local_decls_mut` found for type `&mut rustc::mir::Body<'tcx>` in the current scope
2019-10-11T03:26:11.0210383Z   --> src/librustc_mir/transform/add_retag.rs:67:48
2019-10-11T03:26:11.0210661Z    |
2019-10-11T03:26:11.0210996Z 67 |         let (basic_blocks, local_decls) = body.basic_blocks_and_local_decls_mut();
2019-10-11T03:26:11.0211832Z    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc::mir::Body<'tcx>`
2019-10-11T03:26:11.0361376Z error[E0308]: mismatched types
2019-10-11T03:26:11.0361862Z    --> src/librustc_codegen_ssa/mir/block.rs:582:45
2019-10-11T03:26:11.0363017Z     |
2019-10-11T03:26:11.0363017Z     |
2019-10-11T03:26:11.0363373Z 582 |                     helper.maybe_sideeffect(self.mir, &mut bx, &[*target]);
2019-10-11T03:26:11.0364328Z     |
2019-10-11T03:26:11.0364682Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:11.0364682Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:11.0365027Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:11.1317445Z error[E0308]: mismatched types
2019-10-11T03:26:11.1317765Z   --> src/librustc_mir/transform/cleanup_post_borrowck.rs:32:27
2019-10-11T03:26:11.1318035Z    |
2019-10-11T03:26:11.1318035Z    |
2019-10-11T03:26:11.1318268Z 32 |         delete.visit_body(body);
2019-10-11T03:26:11.1318597Z    |                           ^^^^ expected struct `rustc::mir::BodyCache`, found struct `rustc::mir::Body`
2019-10-11T03:26:11.1318818Z    |
2019-10-11T03:26:11.1319091Z    = note: expected type `&mut rustc::mir::BodyCache<&mut rustc::mir::Body<'_>>`
2019-10-11T03:26:11.1320286Z 
2019-10-11T03:26:11.2074873Z error[E0308]: mismatched types
2019-10-11T03:26:11.2075266Z    --> src/librustc_codegen_ssa/mir/block.rs:597:41
2019-10-11T03:26:11.2075531Z     |
2019-10-11T03:26:11.2075531Z     |
2019-10-11T03:26:11.2075995Z 597 |                 helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:11.2076622Z     |
2019-10-11T03:26:11.2076906Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:11.2076906Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:11.2077232Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:11.3533515Z error[E0308]: mismatched types
2019-10-11T03:26:11.3533896Z    --> src/librustc_mir/transform/check_unsafety.rs:541:24
2019-10-11T03:26:11.3534146Z     |
2019-10-11T03:26:11.3534146Z     |
2019-10-11T03:26:11.3534421Z 541 |     checker.visit_body(body);
2019-10-11T03:26:11.3534834Z     |                        ^^^^ expected struct `rustc::mir::BodyCache`, found struct `lock_api::rwlock::MappedRwLockReadGuard`
2019-10-11T03:26:11.3535097Z     |
2019-10-11T03:26:11.3535422Z     = note: expected type `&rustc::mir::BodyCache<&rustc::mir::Body<'_>>`
2019-10-11T03:26:11.3535755Z                found type `&lock_api::rwlock::MappedRwLockReadGuard<'_, parking_lot::raw_rwlock::RawRwLock, rustc::mir::Body<'_>>`
2019-10-11T03:26:11.3605723Z error[E0308]: mismatched types
2019-10-11T03:26:11.3606004Z    --> src/librustc_codegen_ssa/mir/block.rs:707:41
2019-10-11T03:26:11.3606234Z     |
2019-10-11T03:26:11.3606234Z     |
2019-10-11T03:26:11.3606508Z 707 |                 helper.maybe_sideeffect(self.mir, &mut bx, &[target]);
2019-10-11T03:26:11.3607334Z     |
2019-10-11T03:26:11.3607587Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:11.3607587Z     = note: expected type `&rustc::mir::Body<'_>`
2019-10-11T03:26:11.3607877Z                found type `std::option::Option<&'a mut rustc::mir::BodyCache<&'a rustc::mir::Body<'tcx>>>`
2019-10-11T03:26:11.3778384Z error[E0599]: no method named `basic_blocks_mut` found for type `&mut rustc::mir::Body<'tcx>` in the current scope
2019-10-11T03:26:11.3778720Z   --> src/librustc_mir/transform/simplify_branches.rs:24:27
2019-10-11T03:26:11.3778920Z    |
2019-10-11T03:26:11.3778920Z    |
2019-10-11T03:26:11.3779190Z 24 |         for block in body.basic_blocks_mut() {
2019-10-11T03:26:11.3779900Z    |                           ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `basic_blocks`
2019-10-11T03:26:11.3779948Z 
2019-10-11T03:26:11.3866555Z error[E0599]: no method named `basic_blocks_mut` found for type `&mut rustc::mir::Body<'_>` in the current scope
2019-10-11T03:26:11.3867241Z   --> src/librustc_mir/transform/simplify.rs:52:10
2019-10-11T03:26:11.3867461Z    |
2019-10-11T03:26:11.3867718Z 52 |     body.basic_blocks_mut().raw.shrink_to_fit();
2019-10-11T03:26:11.3868301Z    |          ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `basic_blocks`
2019-10-11T03:26:11.3958343Z error[E0599]: no method named `basic_blocks_mut` found for type `&'a mut rustc::mir::Body<'tcx>` in the current scope
2019-10-11T03:26:11.3958702Z   --> src/librustc_mir/transform/simplify.rs:87:33
2019-10-11T03:26:11.3958928Z    |
2019-10-11T03:26:11.3958928Z    |
2019-10-11T03:26:11.3959220Z 87 |         let basic_blocks = body.basic_blocks_mut();
2019-10-11T03:26:11.3959650Z    |                                 ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `basic_blocks`
2019-10-11T03:26:11.3959699Z 
2019-10-11T03:26:11.4169650Z error[E0599]: no method named `basic_blocks_mut` found for type `&mut rustc::mir::Body<'_>` in the current scope
2019-10-11T03:26:11.4170014Z    --> src/librustc_mir/transform/simplify.rs:270:29
2019-10-11T03:26:11.4170225Z     |
2019-10-11T03:26:11.4170495Z 270 |     let basic_blocks = body.basic_blocks_mut();
2019-10-11T03:26:11.4170874Z     |                             ^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `basic_blocks`
2019-10-11T03:26:11.5177376Z error[E0308]: mismatched types
2019-10-11T03:26:11.5178359Z    --> src/librustc_codegen_ssa/mir/block.rs:801:37
2019-10-11T03:26:11.5178722Z     |
2019-10-11T03:26:11.5178722Z     |
2019-10-11T03:26:11.5178995Z 801 |             helper.maybe_sideeffect(self.mir, &mut bx, &[*target]);
2019-10-11T03:26:11.5179830Z     |
---
2019-10-11T03:26:17.0348362Z == clock drift check ==
2019-10-11T03:26:17.0368847Z   local time: Fri Oct 11 03:26:17 UTC 2019
2019-10-11T03:26:17.1348918Z   network time: Fri, 11 Oct 2019 03:26:17 GMT
2019-10-11T03:26:17.1354485Z == end clock drift check ==
2019-10-11T03:26:17.7922882Z ##[error]Bash exited with code '1'.
2019-10-11T03:26:17.7977064Z ##[section]Starting: Checkout
2019-10-11T03:26:17.7979005Z ==============================================================================
2019-10-11T03:26:17.7979074Z Task         : Get sources
2019-10-11T03:26:17.7979118Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
