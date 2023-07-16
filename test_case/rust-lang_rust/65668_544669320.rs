plain
2019-10-21T19:06:24.8372976Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T19:06:24.8551576Z ##[command]git config gc.auto 0
2019-10-21T19:06:25.4080068Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T19:06:25.4087110Z ##[command]git config --get-all http.proxy
2019-10-21T19:06:25.4093117Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65668/merge:refs/remotes/pull/65668/merge
---
2019-10-21T19:17:20.4558454Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-21T19:17:33.2276943Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-10-21T19:18:50.8897148Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-21T19:22:20.4458083Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-10-21T19:22:24.5149446Z error[E0560]: struct `check::generator_interior::InteriorVisitor<'_, '_>` has no field named `prev_span`
2019-10-21T19:22:24.5151916Z    --> src/librustc_typeck/check/generator_interior.rs:120:9
2019-10-21T19:22:24.5153754Z 120 |         prev_span: None,
2019-10-21T19:22:24.5153754Z 120 |         prev_span: None,
2019-10-21T19:22:24.5154492Z     |         ^^^^^^^^^ `check::generator_interior::InteriorVisitor<'_, '_>` does not have this field
2019-10-21T19:22:24.5155720Z     |
2019-10-21T19:22:24.5156498Z     = note: available fields are: `fcx`, `types`, `region_scope_tree`, `expr_count`, `kind`, `prev_unresolved_span`
2019-10-21T19:22:25.6776052Z error: aborting due to previous error
2019-10-21T19:22:25.6783139Z 
2019-10-21T19:22:25.6783917Z For more information about this error, try `rustc --explain E0560`.
2019-10-21T19:22:25.6784417Z error: could not compile `rustc_typeck`.
---
2019-10-21T19:27:12.0869443Z   local time: Mon Oct 21 19:27:12 UTC 2019
2019-10-21T19:27:12.1856415Z   network time: Mon, 21 Oct 2019 19:27:12 GMT
2019-10-21T19:27:12.1858528Z == end clock drift check ==
2019-10-21T19:27:15.0034639Z 
2019-10-21T19:27:15.0170921Z ##[error]Bash exited with code '1'.
2019-10-21T19:27:15.0209142Z ##[section]Starting: Checkout
2019-10-21T19:27:15.0211647Z ==============================================================================
2019-10-21T19:27:15.0211709Z Task         : Get sources
2019-10-21T19:27:15.0211772Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
