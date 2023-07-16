plain
2019-08-01T00:30:02.0670478Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T00:30:02.0866209Z ##[command]git config gc.auto 0
2019-08-01T00:30:02.0943216Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T00:30:02.0996568Z ##[command]git config --get-all http.proxy
2019-08-01T00:30:02.1128680Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63180/merge:refs/remotes/pull/63180/merge
---
2019-08-01T00:30:39.6618346Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T00:30:39.6618379Z 
2019-08-01T00:30:39.6618597Z   git checkout -b <new-branch-name>
2019-08-01T00:30:39.6618647Z 
2019-08-01T00:30:39.6618711Z HEAD is now at c07107daf Merge 3adbffe4f42d5a4ab13a1980a5b1e81aa71bb709 into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-08-01T00:30:39.6782289Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T00:30:39.6784814Z ==============================================================================
2019-08-01T00:30:39.6784863Z Task         : Bash
2019-08-01T00:30:39.6784902Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T00:40:11.0012872Z    Compiling parking_lot_core v0.4.0
2019-08-01T00:40:14.1683824Z     Checking tempfile v3.0.5
2019-08-01T00:40:14.2645857Z     Checking parking_lot v0.7.1
2019-08-01T00:40:14.6054752Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-01T00:40:15.1680006Z error[E0412]: cannot find type `OpaqueTy` in module `hir`
2019-08-01T00:40:15.1680357Z    --> src/librustdoc/doctree.rs:166:31
2019-08-01T00:40:15.1680915Z     |
2019-08-01T00:40:15.1681249Z 166 |     pub opaque_ty: &'hir hir::OpaqueTy,
2019-08-01T00:40:15.1681620Z     |                               ^^^^^^^^ not found in `hir`
2019-08-01T00:40:15.1682210Z     |
2019-08-01T00:40:15.1682210Z     |
2019-08-01T00:40:15.1682529Z 3   | use crate::clean::OpaqueTy;
2019-08-01T00:40:15.1683013Z     |
2019-08-01T00:40:15.1683311Z 3   | use crate::doctree::OpaqueTy;
2019-08-01T00:40:15.1683580Z 
2019-08-01T00:40:15.1683580Z 
2019-08-01T00:40:16.3365947Z error[E0609]: no field `exist_ty` on type `&doctree::OpaqueTy<'_>`
2019-08-01T00:40:16.3366332Z     --> src/librustdoc/clean/mod.rs:3667:30
2019-08-01T00:40:16.3366571Z      |
2019-08-01T00:40:16.3367360Z 3667 |                 bounds: self.exist_ty.bounds.clean(cx),
2019-08-01T00:40:16.3367728Z 
2019-08-01T00:40:16.3367728Z 
2019-08-01T00:40:16.3373198Z error[E0609]: no field `exist_ty` on type `&doctree::OpaqueTy<'_>`
2019-08-01T00:40:16.3373537Z     --> src/librustdoc/clean/mod.rs:3668:32
2019-08-01T00:40:16.3373776Z      |
2019-08-01T00:40:16.3374132Z 3668 |                 generics: self.exist_ty.generics.clean(cx),
2019-08-01T00:40:16.3374475Z 
2019-08-01T00:40:16.3374475Z 
2019-08-01T00:40:18.0522256Z error[E0560]: struct `doctree::OpaqueTy<'_>` has no field named `exist_ty`
2019-08-01T00:40:18.0522671Z    --> src/librustdoc/visit_ast.rs:477:21
2019-08-01T00:40:18.0523481Z 477 |                     exist_ty,
2019-08-01T00:40:18.0523481Z 477 |                     exist_ty,
2019-08-01T00:40:18.0524070Z     |                     ^^^^^^^^ `doctree::OpaqueTy<'_>` does not have this field
2019-08-01T00:40:18.0524348Z     |
2019-08-01T00:40:18.0524673Z     = note: available fields are: `opaque_ty`, `name`, `id`, `attrs`, `whence` ... and 3 others
2019-08-01T00:40:18.1655276Z error: aborting due to 4 previous errors
2019-08-01T00:40:18.1655374Z 
2019-08-01T00:40:18.1655851Z Some errors have detailed explanations: E0412, E0560, E0609.
2019-08-01T00:40:18.1656490Z For more information about an error, try `rustc --explain E0412`.
2019-08-01T00:40:18.1656490Z For more information about an error, try `rustc --explain E0412`.
2019-08-01T00:40:18.2047980Z error: Could not compile `rustdoc`.
2019-08-01T00:40:18.2048077Z 
2019-08-01T00:40:18.2048369Z To learn more, run the command again with --verbose.
2019-08-01T00:40:18.2085546Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
2019-08-01T00:40:18.2085891Z expected success, got: exit code: 101
2019-08-01T00:40:18.2093291Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-01T00:40:18.2093362Z Build completed unsuccessfully in 0:06:48
2019-08-01T00:40:20.2346261Z ##[error]Bash exited with code '1'.
2019-08-01T00:40:20.2398385Z ##[section]Starting: Checkout
2019-08-01T00:40:20.2399844Z ==============================================================================
2019-08-01T00:40:20.2399891Z Task         : Get sources
2019-08-01T00:40:20.2399933Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
