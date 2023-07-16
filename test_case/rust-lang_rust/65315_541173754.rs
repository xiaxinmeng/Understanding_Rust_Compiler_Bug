plain
2019-10-11T18:19:05.9111162Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T18:19:05.9210537Z ##[command]git config gc.auto 0
2019-10-11T18:19:05.9290681Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T18:19:05.9357409Z ##[command]git config --get-all http.proxy
2019-10-11T18:19:05.9516985Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65315/merge:refs/remotes/pull/65315/merge
---
2019-10-11T18:26:23.8970409Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-11T18:26:39.4315666Z error[E0308]: mismatched types
2019-10-11T18:26:39.4317264Z    --> src/librustc/mir/visit.rs:798:36
2019-10-11T18:26:39.4318191Z     |
2019-10-11T18:26:39.4318968Z 68  | / macro_rules! make_mir_visitor {
2019-10-11T18:26:39.4319665Z 69  | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
2019-10-11T18:26:39.4320672Z 70  | |         pub trait $visitor_trait_name<'tcx> {
2019-10-11T18:26:39.4321439Z 71  | |             // Override these, and call `self.super_xxx` to revert back to the
2019-10-11T18:26:39.4322011Z ...   |
2019-10-11T18:26:39.4322649Z 161 | |             visit_place_fns!($($mutability)?);
2019-10-11T18:26:39.4323901Z ...   |
2019-10-11T18:26:39.4324778Z 782 | |     }
2019-10-11T18:26:39.4325397Z 783 | | }
2019-10-11T18:26:39.4325397Z 783 | | }
2019-10-11T18:26:39.4325999Z     | |_- in this expansion of `make_mir_visitor!`
2019-10-11T18:26:39.4326562Z 784 | 
2019-10-11T18:26:39.4327167Z 785 | / macro_rules! visit_place_fns {
2019-10-11T18:26:39.4327786Z 786 | |     (mut) => (
2019-10-11T18:26:39.4328397Z 787 | |         fn tcx<'a>(&'a self) -> TyCtxt<'tcx>;
2019-10-11T18:26:39.4329527Z ...   |
2019-10-11T18:26:39.4329527Z ...   |
2019-10-11T18:26:39.4330591Z 798 | |                 place.projection = new_projection;
2019-10-11T18:26:39.4332650Z ...   |
2019-10-11T18:26:39.4333153Z 919 | |     );
2019-10-11T18:26:39.4333637Z 920 | | }
2019-10-11T18:26:39.4333637Z 920 | | }
2019-10-11T18:26:39.4334132Z     | |_- in this expansion of `visit_place_fns!`
2019-10-11T18:26:39.4334532Z ...
2019-10-11T18:26:39.4335017Z 923 |   make_mir_visitor!(MutVisitor,mut);
2019-10-11T18:26:39.4335964Z     |
2019-10-11T18:26:39.4335964Z     |
2019-10-11T18:26:39.4336649Z     = note: expected type `&'tcx ty::List<mir::ProjectionElem<mir::Local, &'tcx ty::TyS<'tcx>>>`
2019-10-11T18:26:39.4337230Z                found type `std::boxed::Box<[mir::ProjectionElem<mir::Local, &ty::TyS<'_>>]>`
2019-10-11T18:26:47.4559554Z error: aborting due to previous error
2019-10-11T18:26:47.4559826Z 
2019-10-11T18:26:47.4560338Z For more information about this error, try `rustc --explain E0308`.
2019-10-11T18:26:47.6201938Z error: could not compile `rustc`.
---
2019-10-11T18:26:47.6288942Z == clock drift check ==
2019-10-11T18:26:47.6309089Z   local time: Fri Oct 11 18:26:47 UTC 2019
2019-10-11T18:26:47.7711615Z   network time: Fri, 11 Oct 2019 18:26:47 GMT
2019-10-11T18:26:47.7715413Z == end clock drift check ==
2019-10-11T18:26:48.1760173Z ##[error]Bash exited with code '1'.
2019-10-11T18:26:48.1818373Z ##[section]Starting: Checkout
2019-10-11T18:26:48.1819966Z ==============================================================================
2019-10-11T18:26:48.1820016Z Task         : Get sources
2019-10-11T18:26:48.1820070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
