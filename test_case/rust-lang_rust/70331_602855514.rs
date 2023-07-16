plain
2020-03-23T19:20:29.3896925Z ========================== Starting Command Output ===========================
2020-03-23T19:20:29.3899419Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/60657d4b-14e8-44ca-a740-05a6a9ae3f0d.sh
2020-03-23T19:20:29.3899714Z 
2020-03-23T19:20:29.3903435Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T19:20:29.3922688Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70331/merge to s
2020-03-23T19:20:29.3925903Z Task         : Get sources
2020-03-23T19:20:29.3926215Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T19:20:29.3926528Z Version      : 1.0.0
2020-03-23T19:20:29.3926729Z Author       : Microsoft
---
2020-03-23T19:20:30.3898782Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T19:20:30.3904165Z ##[command]git config gc.auto 0
2020-03-23T19:20:30.3907788Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T19:20:30.3911143Z ##[command]git config --get-all http.proxy
2020-03-23T19:20:30.3917154Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70331/merge:refs/remotes/pull/70331/merge
---
2020-03-23T19:28:36.2729436Z configure: build.locked-deps    := True
2020-03-23T19:28:36.2730308Z configure: llvm.ccache          := sccache
2020-03-23T19:28:36.2730847Z configure: build.cargo-native-static := True
2020-03-23T19:28:36.2731318Z configure: dist.missing-tools   := True
2020-03-23T19:28:36.2731919Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T19:28:36.2732467Z configure: writing `config.toml` in current directory
2020-03-23T19:28:36.2733063Z configure: 
2020-03-23T19:28:36.2733816Z configure: run `python /checkout/x.py --help`
2020-03-23T19:28:36.2734054Z configure: 
---
2020-03-23T19:35:55.8278466Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-23T19:35:55.8282864Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-23T19:35:55.8287047Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-23T19:35:55.8291706Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-23T19:35:56.6638507Z Diff in /checkout/src/librustc_privacy/lib.rs at line 1023:
2020-03-23T19:35:56.6638999Z          span: Span,            // span of the field pattern, e.g., `x: 0`
2020-03-23T19:35:56.6640165Z          def: &'tcx ty::AdtDef, // definition of the struct or enum
2020-03-23T19:35:56.6640683Z          field: &'tcx ty::FieldDef,
2020-03-23T19:35:56.6641104Z -        in_update_syntax: bool
2020-03-23T19:35:56.6641757Z +        in_update_syntax: bool,
2020-03-23T19:35:56.6642148Z          // definition of the field
2020-03-23T19:35:56.6642148Z          // definition of the field
2020-03-23T19:35:56.6642459Z          let ident = Ident::new(kw::Invalid, use_ctxt);
2020-03-23T19:35:56.6662712Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_privacy/lib.rs"` failed.
2020-03-23T19:35:56.6663736Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-23T19:35:56.6664609Z Build completed unsuccessfully in 0:00:35
2020-03-23T19:35:56.6713807Z == clock drift check ==
2020-03-23T19:35:56.6726867Z   local time: Mon Mar 23 19:35:56 UTC 2020
2020-03-23T19:35:56.7643387Z   network time: Mon, 23 Mar 2020 19:35:56 GMT
2020-03-23T19:35:56.7643387Z   network time: Mon, 23 Mar 2020 19:35:56 GMT
2020-03-23T19:35:56.7659719Z == end clock drift check ==
2020-03-23T19:35:58.3248353Z 
2020-03-23T19:35:58.3315662Z ##[error]Bash exited with code '1'.
2020-03-23T19:35:58.3329295Z ##[section]Finishing: Run build
2020-03-23T19:35:58.3375213Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70331/merge to s
2020-03-23T19:35:58.3380002Z Task         : Get sources
2020-03-23T19:35:58.3380352Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T19:35:58.3380681Z Version      : 1.0.0
2020-03-23T19:35:58.3380928Z Author       : Microsoft
2020-03-23T19:35:58.3380928Z Author       : Microsoft
2020-03-23T19:35:58.3381291Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T19:35:58.3381707Z ==============================================================================
2020-03-23T19:35:58.6710627Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T19:35:58.6753759Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70331/merge to s
2020-03-23T19:35:58.6928875Z Cleaning up task key
2020-03-23T19:35:58.6930075Z Start cleaning up orphan processes.
2020-03-23T19:35:58.7099130Z Terminate orphan process: pid (3667) (python)
2020-03-23T19:35:58.7267273Z ##[section]Finishing: Finalize Job
