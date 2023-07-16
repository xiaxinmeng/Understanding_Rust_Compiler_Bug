plain
2019-08-07T20:24:40.6224954Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-07T20:24:40.6401022Z ##[command]git config gc.auto 0
2019-08-07T20:24:40.6464682Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-07T20:24:40.6517161Z ##[command]git config --get-all http.proxy
2019-08-07T20:24:40.6643953Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62497/merge:refs/remotes/pull/62497/merge
---
2019-08-07T20:25:17.9143167Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T20:25:17.9143193Z 
2019-08-07T20:25:17.9143557Z   git checkout -b <new-branch-name>
2019-08-07T20:25:17.9143811Z 
2019-08-07T20:25:17.9143852Z HEAD is now at 1260a9955 Merge 5d4ee06c0cea3e6a61961e7f566f044ecad81858 into 647ed20e439e48f7ea88cebc6cad17cb85e63a3d
2019-08-07T20:25:17.9300780Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T20:25:17.9303310Z ==============================================================================
2019-08-07T20:25:17.9303533Z Task         : Bash
2019-08-07T20:25:17.9303589Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T20:29:08.4923377Z    Compiling serde_derive v1.0.81
2019-08-07T20:29:24.9751873Z    Compiling toml v0.4.10
2019-08-07T20:29:24.9752575Z    Compiling serde_json v1.0.40
2019-08-07T20:29:28.6456726Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-08-07T20:29:31.2787402Z error[E0599]: no method named `sysroot_libdir_relative` found for type `builder::Builder<'a>::sysroot_libdir::Libdir` in the current scope
2019-08-07T20:29:31.2788237Z    --> src/bootstrap/builder.rs:632:32
2019-08-07T20:29:31.2788678Z 618 |         struct Libdir {
2019-08-07T20:29:31.2788678Z 618 |         struct Libdir {
2019-08-07T20:29:31.2789129Z     |         ------------- method `sysroot_libdir_relative` not found for this
2019-08-07T20:29:31.2789353Z ...
2019-08-07T20:29:31.2789562Z 632 |                 let lib = self.sysroot_libdir_relative(self.compiler);
2019-08-07T20:29:31.2790079Z 
2019-08-07T20:29:31.4651171Z error: aborting due to previous error
2019-08-07T20:29:31.4651722Z 
2019-08-07T20:29:31.4652471Z For more information about this error, try `rustc --explain E0599`.
2019-08-07T20:29:31.4652471Z For more information about this error, try `rustc --explain E0599`.
2019-08-07T20:29:31.5157015Z error: Could not compile `bootstrap`.
2019-08-07T20:29:31.5157720Z 
2019-08-07T20:29:31.5158681Z To learn more, run the command again with --verbose.
2019-08-07T20:29:31.5188437Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-07T20:29:31.5188808Z Build completed unsuccessfully in 0:01:17
2019-08-07T20:29:35.7836554Z ##[error]Bash exited with code '1'.
2019-08-07T20:29:35.7864818Z ##[section]Starting: Checkout
2019-08-07T20:29:35.7866478Z ==============================================================================
2019-08-07T20:29:35.7866691Z Task         : Get sources
2019-08-07T20:29:35.7866740Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
