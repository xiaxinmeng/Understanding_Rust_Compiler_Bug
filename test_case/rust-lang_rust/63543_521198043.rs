plain
2019-08-14T09:34:18.3398765Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T09:34:18.3682397Z ##[command]git config gc.auto 0
2019-08-14T09:34:18.3767282Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T09:34:18.3816960Z ##[command]git config --get-all http.proxy
2019-08-14T09:34:18.3971620Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63543/merge:refs/remotes/pull/63543/merge
---
2019-08-14T09:34:51.4929711Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T09:34:51.4929786Z 
2019-08-14T09:34:51.4929991Z   git checkout -b <new-branch-name>
2019-08-14T09:34:51.4930021Z 
2019-08-14T09:34:51.4930065Z HEAD is now at ebb5d7bd5 Merge 0ebd7f0a3411b6aaea0d69a0b47501a4a8d7b5ef into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-14T09:34:51.5114417Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T09:34:51.5117171Z ==============================================================================
2019-08-14T09:34:51.5117244Z Task         : Bash
2019-08-14T09:34:51.5117286Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T09:43:51.6186948Z    Compiling rand_pcg v0.1.1
2019-08-14T09:43:51.6960734Z    Compiling rand v0.6.1
2019-08-14T09:43:54.3943294Z     Checking tempfile v3.0.5
2019-08-14T09:43:54.9800988Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-14T09:43:58.4087155Z error[E0560]: struct `doctree::Variant<'_>` has no field named `stab`
2019-08-14T09:43:58.4091027Z    --> src/librustdoc/visit_ast.rs:136:17
2019-08-14T09:43:58.4091629Z     |
2019-08-14T09:43:58.4092368Z 136 |                 stab: self.stability(v.id),
2019-08-14T09:43:58.4092932Z     |                 ^^^^ `doctree::Variant<'_>` does not have this field
2019-08-14T09:43:58.4093347Z     |
2019-08-14T09:43:58.4094178Z     = note: available fields are: `name`, `id`, `attrs`, `def`, `whence`
2019-08-14T09:43:58.4094364Z 
2019-08-14T09:43:58.4801581Z error[E0599]: no method named `stability` found for type `&mut visit_ast::RustdocVisitor<'a, 'tcx>` in the current scope
2019-08-14T09:43:58.4805306Z    --> src/librustdoc/visit_ast.rs:136:28
2019-08-14T09:43:58.4805978Z     |
2019-08-14T09:43:58.4806568Z 136 |                 stab: self.stability(v.id),
2019-08-14T09:43:58.4807152Z 
2019-08-14T09:43:58.4807152Z 
2019-08-14T09:43:58.4807617Z error[E0560]: struct `doctree::Variant<'_>` has no field named `depr`
2019-08-14T09:43:58.4808022Z    --> src/librustdoc/visit_ast.rs:137:17
2019-08-14T09:43:58.4808380Z     |
2019-08-14T09:43:58.4808804Z 137 |                 depr: self.deprecation(v.id),
2019-08-14T09:43:58.4809264Z     |                 ^^^^ `doctree::Variant<'_>` does not have this field
2019-08-14T09:43:58.4809609Z     |
2019-08-14T09:43:58.4810039Z     = note: available fields are: `name`, `id`, `attrs`, `def`, `whence`
2019-08-14T09:43:58.4810190Z 
2019-08-14T09:43:58.4855728Z error[E0599]: no method named `deprecation` found for type `&mut visit_ast::RustdocVisitor<'a, 'tcx>` in the current scope
2019-08-14T09:43:58.4861560Z    --> src/librustdoc/visit_ast.rs:137:28
2019-08-14T09:43:58.4862603Z     |
2019-08-14T09:43:58.4863209Z 137 |                 depr: self.deprecation(v.id),
2019-08-14T09:43:58.4863892Z 
2019-08-14T09:43:58.6442449Z error: aborting due to 4 previous errors
2019-08-14T09:43:58.6442563Z 
2019-08-14T09:43:58.6442842Z Some errors have detailed explanations: E0560, E0599.
---
2019-08-14T09:43:58.6906274Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
2019-08-14T09:43:58.6906410Z expected success, got: exit code: 101
2019-08-14T09:43:58.6919471Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-14T09:43:58.6919781Z Build completed unsuccessfully in 0:06:35
2019-08-14T09:44:00.5490400Z ##[error]Bash exited with code '1'.
2019-08-14T09:44:00.5525359Z ##[section]Starting: Checkout
2019-08-14T09:44:00.5527255Z ==============================================================================
2019-08-14T09:44:00.5527308Z Task         : Get sources
2019-08-14T09:44:00.5527352Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
