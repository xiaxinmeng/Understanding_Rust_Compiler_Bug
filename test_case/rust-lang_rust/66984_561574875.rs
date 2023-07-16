plain
2019-12-04T09:09:36.5747699Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-04T09:09:36.5760142Z ##[command]git config gc.auto 0
2019-12-04T09:09:36.5762661Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-04T09:09:36.5764841Z ##[command]git config --get-all http.proxy
2019-12-04T09:09:36.5768428Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66984/merge:refs/remotes/pull/66984/merge
---
2019-12-04T09:18:24.2603897Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-12-04T09:18:24.2604490Z error[E0425]: cannot find value `LOCAL_CRATE` in this scope
2019-12-04T09:18:24.2611761Z   --> src/librustdoc/clean/types.rs:99:65
2019-12-04T09:18:24.2643831Z    |
2019-12-04T09:18:24.2644226Z 99 |     let ExternalCrate { name, src, primitives, keywords, .. } = LOCAL_CRATE.clean(cx);
2019-12-04T09:18:24.2644855Z    |
2019-12-04T09:18:24.2645142Z help: possible candidate is found in another module, you can import it into scope
2019-12-04T09:18:24.2645511Z    |
2019-12-04T09:18:24.2645807Z 1  | use rustc::hir::def_id::LOCAL_CRATE;
---
2019-12-04T09:18:24.2647878Z help: possible candidates are found in other modules, you can import them into scope
2019-12-04T09:18:24.2648085Z     |
2019-12-04T09:18:24.2648351Z 1   | use crate::clean::types::Visibility::Public;
2019-12-04T09:18:24.2648585Z     |
2019-12-04T09:18:24.2648851Z 1   | use minifier::js::Keyword::Public;
2019-12-04T09:18:24.2649339Z 1   | use rustc::hir::VisibilityKind::Public;
2019-12-04T09:18:24.2649541Z     |
2019-12-04T09:18:24.2649541Z     |
2019-12-04T09:18:24.2649810Z 1   | use rustc::middle::privacy::AccessLevel::Public;
2019-12-04T09:18:24.2650231Z       and 2 other candidates
2019-12-04T09:18:24.2650262Z 
2019-12-04T09:18:24.2650531Z error[E0425]: cannot find function `get_stability` in this scope
2019-12-04T09:18:24.2650910Z    --> src/librustdoc/clean/types.rs:111:28
---
2019-12-04T09:18:24.2659561Z help: possible candidates are found in other modules, you can import them into scope
2019-12-04T09:18:24.2659805Z     |
2019-12-04T09:18:24.2660124Z 1   | use crate::clean::types::Visibility::Public;
2019-12-04T09:18:24.2660541Z     |
2019-12-04T09:18:24.2660826Z 1   | use minifier::js::Keyword::Public;
2019-12-04T09:18:24.2661345Z 1   | use rustc::hir::VisibilityKind::Public;
2019-12-04T09:18:24.2661765Z     |
2019-12-04T09:18:24.2661765Z     |
2019-12-04T09:18:24.2662212Z 1   | use rustc::middle::privacy::AccessLevel::Public;
2019-12-04T09:18:24.2663237Z       and 2 other candidates
2019-12-04T09:18:24.2663271Z 
2019-12-04T09:18:24.2663571Z error[E0425]: cannot find function `get_stability` in this scope
2019-12-04T09:18:24.2663851Z    --> src/librustdoc/clean/types.rs:123:28
---
2019-12-04T09:18:24.2670345Z     |
2019-12-04T09:18:24.2670634Z 1   | use crate::clean::get_deprecation;
2019-12-04T09:18:24.2670859Z     |
2019-12-04T09:18:24.2670905Z 
2019-12-04T09:18:25.1925192Z error[E0599]: no method named `clean` found for type `rustc::hir::def_id::CrateNum` in the current scope
2019-12-04T09:18:25.1925739Z   --> src/librustdoc/clean/types.rs:72:34
2019-12-04T09:18:25.1926024Z    |
2019-12-04T09:18:25.1926305Z 72 |         externs.push((cnum, cnum.clean(cx)));
2019-12-04T09:18:25.1926660Z    |                                  ^^^^^ method not found in `rustc::hir::def_id::CrateNum`
2019-12-04T09:18:25.1927337Z   ::: src/librustdoc/clean/mod.rs:67:8
2019-12-04T09:18:25.1927581Z    |
2019-12-04T09:18:25.1927581Z    |
2019-12-04T09:18:25.1927873Z 67 |     fn clean(&self, cx: &DocContext<'_>) -> T;
2019-12-04T09:18:25.1928747Z    |        |
2019-12-04T09:18:25.1928747Z    |        |
2019-12-04T09:18:25.1929226Z    |        the method is available for `std::boxed::Box<rustc::hir::def_id::CrateNum>` here
2019-12-04T09:18:25.1929740Z    |        the method is available for `std::sync::Arc<rustc::hir::def_id::CrateNum>` here
2019-12-04T09:18:25.1930342Z    |        the method is available for `std::rc::Rc<rustc::hir::def_id::CrateNum>` here
2019-12-04T09:18:25.1931199Z    = help: items from traits can only be used if the trait is in scope
2019-12-04T09:18:25.1931199Z    = help: items from traits can only be used if the trait is in scope
2019-12-04T09:18:25.1931690Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-12-04T09:18:25.1931999Z            `use crate::clean::Clean;`
2019-12-04T09:18:25.1932063Z 
2019-12-04T09:18:25.1997537Z error[E0599]: no method named `clean` found for type `doctree::Module<'_>` in the current scope
2019-12-04T09:18:25.1998075Z   ::: src/librustdoc/doctree.rs:14:1
2019-12-04T09:18:25.1998283Z    |
2019-12-04T09:18:25.1998283Z    |
2019-12-04T09:18:25.1998533Z 14 | pub struct Module<'hir> {
2019-12-04T09:18:25.1998857Z    | ----------------------- method `clean` not found for this
2019-12-04T09:18:25.1999106Z   --> src/librustdoc/clean/types.rs:80:29
2019-12-04T09:18:25.1999330Z    |
2019-12-04T09:18:25.1999702Z 80 |     let mut module = module.clean(cx);
2019-12-04T09:18:25.2000090Z    |                             ^^^^^ method not found in `doctree::Module<'_>`
2019-12-04T09:18:25.2000563Z   ::: src/librustdoc/clean/mod.rs:67:8
2019-12-04T09:18:25.2000766Z    |
2019-12-04T09:18:25.2000766Z    |
2019-12-04T09:18:25.2001033Z 67 |     fn clean(&self, cx: &DocContext<'_>) -> T;
2019-12-04T09:18:25.2001535Z    |        |
2019-12-04T09:18:25.2001535Z    |        |
2019-12-04T09:18:25.2003598Z    |        the method is available for `std::boxed::Box<doctree::Module<'_>>` here
2019-12-04T09:18:25.2004216Z    |        the method is available for `std::sync::Arc<doctree::Module<'_>>` here
2019-12-04T09:18:25.2004532Z    |        the method is available for `std::rc::Rc<doctree::Module<'_>>` here
2019-12-04T09:18:25.2005043Z    = help: items from traits can only be used if the trait is in scope
2019-12-04T09:18:25.2005043Z    = help: items from traits can only be used if the trait is in scope
2019-12-04T09:18:25.2005344Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-12-04T09:18:25.2005579Z            `use crate::clean::Clean;`
2019-12-04T09:18:26.6035643Z error: aborting due to 9 previous errors
2019-12-04T09:18:26.6035798Z 
2019-12-04T09:18:26.6036098Z Some errors have detailed explanations: E0425, E0599.
2019-12-04T09:18:26.6036350Z For more information about an error, try `rustc --explain E0425`.
---
2019-12-04T09:18:26.6615607Z   local time: Wed Dec  4 09:18:26 UTC 2019
2019-12-04T09:18:26.7016642Z   network time: Wed, 04 Dec 2019 09:18:26 GMT
2019-12-04T09:18:26.7018973Z == end clock drift check ==
2019-12-04T09:18:28.2858539Z 
2019-12-04T09:18:28.2956955Z ##[error]Bash exited with code '1'.
2019-12-04T09:18:28.2987485Z ##[section]Starting: Checkout
2019-12-04T09:18:28.2989062Z ==============================================================================
2019-12-04T09:18:28.2989113Z Task         : Get sources
2019-12-04T09:18:28.2989172Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
