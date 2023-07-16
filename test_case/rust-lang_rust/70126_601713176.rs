plain
2020-03-20T12:21:29.6800085Z ========================== Starting Command Output ===========================
2020-03-20T12:21:29.6803135Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/eed5eacf-52bc-481d-9236-ef8feebdb3a3.sh
2020-03-20T12:21:29.6803369Z 
2020-03-20T12:21:29.6806767Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T12:21:29.6826763Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70126/merge to s
2020-03-20T12:21:29.6830200Z Task         : Get sources
2020-03-20T12:21:29.6830458Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T12:21:29.6830883Z Version      : 1.0.0
2020-03-20T12:21:29.6831570Z Author       : Microsoft
---
2020-03-20T12:21:30.9602594Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T12:21:30.9608306Z ##[command]git config gc.auto 0
2020-03-20T12:21:30.9613389Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T12:21:30.9618078Z ##[command]git config --get-all http.proxy
2020-03-20T12:21:30.9626906Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70126/merge:refs/remotes/pull/70126/merge
---
2020-03-20T12:29:40.6754345Z configure: build.locked-deps    := True
2020-03-20T12:29:40.6754736Z configure: llvm.ccache          := sccache
2020-03-20T12:29:40.6755466Z configure: build.cargo-native-static := True
2020-03-20T12:29:40.6756169Z configure: dist.missing-tools   := True
2020-03-20T12:29:40.6756779Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-20T12:29:40.6757356Z configure: writing `config.toml` in current directory
2020-03-20T12:29:40.6757622Z configure: 
2020-03-20T12:29:40.6758016Z configure: run `python /checkout/x.py --help`
2020-03-20T12:29:40.6758251Z configure: 
---
2020-03-20T12:37:19.4177298Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-20T12:37:19.4184078Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-20T12:37:19.4187887Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-20T12:37:19.4192559Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-20T12:37:24.2018331Z Diff in /checkout/src/librustc_codegen_ssa/mir/rvalue.rs at line 296:
2020-03-20T12:37:24.2019447Z                                      let discr_layout = bx.cx().layout_of(discr.ty);
2020-03-20T12:37:24.2020044Z                                      let discr_t = bx.cx().immediate_backend_type(discr_layout);
2020-03-20T12:37:24.2020645Z                                      let discr_val = bx.cx().const_uint_big(discr_t, discr.val);
2020-03-20T12:37:24.2021909Z -                                    let discr_val = bx.intcast(discr_val, ll_t_out, discr.ty.is_signed());
2020-03-20T12:37:24.2023073Z +                                    let discr_val =
2020-03-20T12:37:24.2023830Z +                                        bx.intcast(discr_val, ll_t_out, discr.ty.is_signed());
2020-03-20T12:37:24.2024798Z                                      return (
2020-03-20T12:37:24.2025319Z                                          bx,
2020-03-20T12:37:24.2025319Z                                          bx,
2020-03-20T12:37:24.2026522Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_codegen_ssa/mir/rvalue.rs"` failed.
2020-03-20T12:37:24.2027703Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-20T12:37:24.2035991Z Build completed unsuccessfully in 0:00:42
2020-03-20T12:37:24.2086217Z == clock drift check ==
2020-03-20T12:37:24.2099033Z   local time: Fri Mar 20 12:37:24 UTC 2020
2020-03-20T12:37:24.2532138Z   network time: Fri, 20 Mar 2020 12:37:24 GMT
2020-03-20T12:37:24.2532138Z   network time: Fri, 20 Mar 2020 12:37:24 GMT
2020-03-20T12:37:24.2532631Z == end clock drift check ==
2020-03-20T12:37:25.7684673Z 
2020-03-20T12:37:25.7754442Z ##[error]Bash exited with code '1'.
2020-03-20T12:37:25.7768148Z ##[section]Finishing: Run build
2020-03-20T12:37:25.7824432Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70126/merge to s
2020-03-20T12:37:25.7829557Z Task         : Get sources
2020-03-20T12:37:25.7829954Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T12:37:25.7830300Z Version      : 1.0.0
2020-03-20T12:37:25.7830531Z Author       : Microsoft
2020-03-20T12:37:25.7830531Z Author       : Microsoft
2020-03-20T12:37:25.7830930Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T12:37:25.7831381Z ==============================================================================
2020-03-20T12:37:26.0947326Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T12:37:26.0991273Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70126/merge to s
2020-03-20T12:37:26.1076997Z Cleaning up task key
2020-03-20T12:37:26.1078324Z Start cleaning up orphan processes.
2020-03-20T12:37:26.1292843Z Terminate orphan process: pid (3742) (python)
2020-03-20T12:37:26.1473627Z ##[section]Finishing: Finalize Job
