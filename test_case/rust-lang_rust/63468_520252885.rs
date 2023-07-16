plain
2019-08-11T18:44:40.5727387Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T18:44:40.5877384Z ##[command]git config gc.auto 0
2019-08-11T18:44:40.5951716Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T18:44:40.5999654Z ##[command]git config --get-all http.proxy
2019-08-11T18:44:40.6132020Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
---
2019-08-11T18:45:15.2057763Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T18:45:15.2057796Z 
2019-08-11T18:45:15.2058048Z   git checkout -b <new-branch-name>
2019-08-11T18:45:15.2058083Z 
2019-08-11T18:45:15.2058136Z HEAD is now at a351c5e1c Merge a621d06091e063b15045e39efbbcfca16fbbb141 into 8a068699a24de306334a1f66b9a83552766d853c
2019-08-11T18:45:15.2215487Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T18:45:15.2218574Z ==============================================================================
2019-08-11T18:45:15.2218633Z Task         : Bash
2019-08-11T18:45:15.2218697Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T18:54:22.6293180Z    Compiling parking_lot_core v0.4.0
2019-08-11T18:54:26.2770248Z     Checking tempfile v3.0.5
2019-08-11T18:54:26.6363152Z     Checking parking_lot v0.7.1
2019-08-11T18:54:26.9482728Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-11T18:54:30.1125166Z error[E0560]: struct `doctree::Variant<'_>` has no field named `stab`
2019-08-11T18:54:30.1125694Z    --> src/librustdoc/visit_ast.rs:122:17
2019-08-11T18:54:30.1126001Z     |
2019-08-11T18:54:30.1126354Z 122 |                 stab: self.stability(v.id),
2019-08-11T18:54:30.1132954Z     |                 ^^^^ `doctree::Variant<'_>` does not have this field
2019-08-11T18:54:30.1133283Z     |
2019-08-11T18:54:30.1133650Z     = note: available fields are: `name`, `id`, `attrs`, `def`, `whence`
2019-08-11T18:54:30.1133696Z 
2019-08-11T18:54:30.1836498Z error[E0599]: no method named `stability` found for type `&mut visit_ast::RustdocVisitor<'a, 'tcx>` in the current scope
2019-08-11T18:54:30.1836921Z    --> src/librustdoc/visit_ast.rs:122:28
2019-08-11T18:54:30.1837223Z     |
2019-08-11T18:54:30.1837642Z 122 |                 stab: self.stability(v.id),
2019-08-11T18:54:30.1838032Z 
2019-08-11T18:54:30.1838032Z 
2019-08-11T18:54:30.1838343Z error[E0560]: struct `doctree::Variant<'_>` has no field named `depr`
2019-08-11T18:54:30.1838656Z    --> src/librustdoc/visit_ast.rs:123:17
2019-08-11T18:54:30.1838919Z     |
2019-08-11T18:54:30.1839245Z 123 |                 depr: self.deprecation(v.id),
2019-08-11T18:54:30.1839625Z     |                 ^^^^ `doctree::Variant<'_>` does not have this field
2019-08-11T18:54:30.1839900Z     |
2019-08-11T18:54:30.1840428Z     = note: available fields are: `name`, `id`, `attrs`, `def`, `whence`
2019-08-11T18:54:30.1840493Z 
2019-08-11T18:54:30.1894950Z error[E0599]: no method named `deprecation` found for type `&mut visit_ast::RustdocVisitor<'a, 'tcx>` in the current scope
2019-08-11T18:54:30.1895315Z    --> src/librustdoc/visit_ast.rs:123:28
2019-08-11T18:54:30.1895935Z     |
2019-08-11T18:54:30.1896268Z 123 |                 depr: self.deprecation(v.id),
2019-08-11T18:54:30.1896650Z 
2019-08-11T18:54:30.3236178Z error: aborting due to 4 previous errors
2019-08-11T18:54:30.3236292Z 
2019-08-11T18:54:30.3236636Z Some errors have detailed explanations: E0560, E0599.
---
2019-08-11T18:54:30.3653512Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
2019-08-11T18:54:30.3653676Z expected success, got: exit code: 101
2019-08-11T18:54:30.3668318Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-11T18:54:30.3668398Z Build completed unsuccessfully in 0:06:29
2019-08-11T18:54:32.3307446Z ##[error]Bash exited with code '1'.
2019-08-11T18:54:32.3341139Z ##[section]Starting: Checkout
2019-08-11T18:54:32.3343011Z ==============================================================================
2019-08-11T18:54:32.3343088Z Task         : Get sources
2019-08-11T18:54:32.3343142Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
