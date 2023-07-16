plain
2020-03-22T16:49:12.7438978Z ========================== Starting Command Output ===========================
2020-03-22T16:49:12.7442566Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f331809d-d471-41c1-a650-bae12fe052fc.sh
2020-03-22T16:49:12.7443025Z 
2020-03-22T16:49:12.7447009Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T16:49:12.7471190Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70226/merge to s
2020-03-22T16:49:12.7474758Z Task         : Get sources
2020-03-22T16:49:12.7475057Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T16:49:12.7475425Z Version      : 1.0.0
2020-03-22T16:49:12.7475621Z Author       : Microsoft
---
2020-03-22T16:49:13.9272975Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T16:49:13.9284078Z ##[command]git config gc.auto 0
2020-03-22T16:49:13.9291665Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T16:49:13.9312307Z ##[command]git config --get-all http.proxy
2020-03-22T16:49:13.9321711Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70226/merge:refs/remotes/pull/70226/merge
---
2020-03-22T16:58:12.0178953Z configure: build.locked-deps    := True
2020-03-22T16:58:12.0180555Z configure: llvm.ccache          := sccache
2020-03-22T16:58:12.0181337Z configure: build.cargo-native-static := True
2020-03-22T16:58:12.0182218Z configure: dist.missing-tools   := True
2020-03-22T16:58:12.0183055Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-22T16:58:12.0184220Z configure: writing `config.toml` in current directory
2020-03-22T16:58:12.0184809Z configure: 
2020-03-22T16:58:12.0185665Z configure: run `python /checkout/x.py --help`
2020-03-22T16:58:12.0186122Z configure: 
---
2020-03-22T17:06:53.6394473Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-22T17:06:53.6399191Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-22T17:06:53.6404793Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-22T17:06:53.6409662Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-22T17:07:00.5194855Z Diff in /checkout/src/librustc/mir/interpret/allocation.rs at line 303:
2020-03-22T17:07:00.5195446Z          let offset = usize::try_from(ptr.offset.bytes()).unwrap();
2020-03-22T17:07:00.5196297Z          Ok(match self.bytes[offset..].iter().position(|&c| c == 0) {
2020-03-22T17:07:00.5196621Z              Some(size) => {
2020-03-22T17:07:00.5198066Z -                    Size::from_bytes(size.checked_add(1).unwrap());
2020-03-22T17:07:00.5198501Z +                let size_with_null = Size::from_bytes(size.checked_add(1).unwrap());
2020-03-22T17:07:00.5198501Z +                let size_with_null = Size::from_bytes(size.checked_add(1).unwrap());
2020-03-22T17:07:00.5198938Z                  // Go through `get_bytes` for checks and AllocationExtra hooks.
2020-03-22T17:07:00.5199542Z                  // We read the null, so we include it in the request, but we want it removed
2020-03-22T17:07:00.5200069Z                  // from the result, so we do subslicing.
2020-03-22T17:07:00.5200603Z Diff in /checkout/src/librustc/mir/interpret/allocation.rs at line 392:
2020-03-22T17:07:00.5200870Z          } else {
2020-03-22T17:07:00.5201107Z              match self.relocations.get(&ptr.offset) {
2020-03-22T17:07:00.5201406Z                  Some(&(tag, alloc_id)) => {
2020-03-22T17:07:00.5202132Z -                    let ptr = Pointer::new_with_tag(
2020-03-22T17:07:00.5203017Z -                        Size::from_bytes(bits),
2020-03-22T17:07:00.5203452Z -                        tag,
2020-03-22T17:07:00.5204165Z -                    );
2020-03-22T17:07:00.5204165Z -                    );
2020-03-22T17:07:00.5204515Z +                    let ptr = Pointer::new_with_tag(alloc_id, Size::from_bytes(bits), tag);
2020-03-22T17:07:00.5204982Z                      return Ok(ScalarMaybeUndef::Scalar(ptr.into()));
2020-03-22T17:07:00.5205458Z                  None => {}
2020-03-22T17:07:00.5205458Z                  None => {}
2020-03-22T17:07:00.5206393Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc/mir/interpret/allocation.rs"` failed.
2020-03-22T17:07:00.5207351Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-22T17:07:00.5223523Z Build completed unsuccessfully in 0:00:50
2020-03-22T17:07:00.5277445Z == clock drift check ==
2020-03-22T17:07:00.5291469Z   local time: Sun Mar 22 17:07:00 UTC 2020
2020-03-22T17:07:00.8192596Z   network time: Sun, 22 Mar 2020 17:07:00 GMT
2020-03-22T17:07:00.8192596Z   network time: Sun, 22 Mar 2020 17:07:00 GMT
2020-03-22T17:07:00.8194081Z == end clock drift check ==
2020-03-22T17:07:02.1178871Z 
2020-03-22T17:07:02.1258740Z ##[error]Bash exited with code '1'.
2020-03-22T17:07:02.1272842Z ##[section]Finishing: Run build
2020-03-22T17:07:02.1327365Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70226/merge to s
2020-03-22T17:07:02.1332419Z Task         : Get sources
2020-03-22T17:07:02.1332768Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T17:07:02.1333366Z Version      : 1.0.0
2020-03-22T17:07:02.1333930Z Author       : Microsoft
2020-03-22T17:07:02.1333930Z Author       : Microsoft
2020-03-22T17:07:02.1334271Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T17:07:02.1334688Z ==============================================================================
2020-03-22T17:07:02.5145138Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T17:07:02.5192412Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70226/merge to s
2020-03-22T17:07:02.5286906Z Cleaning up task key
2020-03-22T17:07:02.5288170Z Start cleaning up orphan processes.
2020-03-22T17:07:02.5509511Z Terminate orphan process: pid (3648) (python)
2020-03-22T17:07:02.5778819Z ##[section]Finishing: Finalize Job
