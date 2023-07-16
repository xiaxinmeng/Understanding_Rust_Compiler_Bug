plain
2019-09-21T19:58:22.1231347Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T19:58:22.1431483Z ##[command]git config gc.auto 0
2019-09-21T19:58:22.1514614Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T19:58:22.1572429Z ##[command]git config --get-all http.proxy
2019-09-21T19:58:22.1714853Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64670/merge:refs/remotes/pull/64670/merge
---
2019-09-21T20:06:09.8258565Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-09-21T20:06:18.4466068Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-21T20:06:19.8864856Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-21T20:06:21.1107622Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-21T20:06:28.8538686Z error[E0599]: no method named `poly_trait_ref` found for type `&ext::base::ExtCtxt<'a>` in the current scope
2019-09-21T20:06:28.8539056Z    --> src/libsyntax/ext/build.rs:140:39
2019-09-21T20:06:28.8539302Z     |
2019-09-21T20:06:28.8539560Z 140 |         ast::GenericBound::Trait(self.poly_trait_ref(path.span, path),
2019-09-21T20:06:28.8539856Z 
2019-09-21T20:06:29.3176215Z error: aborting due to previous error
2019-09-21T20:06:29.3176325Z 
2019-09-21T20:06:29.3176648Z For more information about this error, try `rustc --explain E0599`.
---
2019-09-21T20:06:29.3800360Z == clock drift check ==
2019-09-21T20:06:29.3811766Z   local time: Sat Sep 21 20:06:29 UTC 2019
2019-09-21T20:06:29.4676673Z   network time: Sat, 21 Sep 2019 20:06:29 GMT
2019-09-21T20:06:29.4676830Z == end clock drift check ==
2019-09-21T20:06:30.6599914Z ##[error]Bash exited with code '1'.
2019-09-21T20:06:30.6632128Z ##[section]Starting: Checkout
2019-09-21T20:06:30.6633603Z ==============================================================================
2019-09-21T20:06:30.6633646Z Task         : Get sources
2019-09-21T20:06:30.6633701Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
