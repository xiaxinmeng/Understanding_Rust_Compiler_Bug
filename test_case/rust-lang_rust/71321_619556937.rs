plain
2020-04-26T14:05:50.2019921Z ========================== Starting Command Output ===========================
2020-04-26T14:05:50.2024439Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e0ab4704-626a-4ef0-8020-55314308c406.sh
2020-04-26T14:05:50.2024858Z 
2020-04-26T14:05:50.2029208Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-26T14:05:50.2046099Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71321/merge to s
2020-04-26T14:05:50.2048999Z Task         : Get sources
2020-04-26T14:05:50.2049262Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-26T14:05:50.2049502Z Version      : 1.0.0
2020-04-26T14:05:50.2049667Z Author       : Microsoft
---
2020-04-26T14:05:51.4310550Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-26T14:05:51.4320038Z ##[command]git config gc.auto 0
2020-04-26T14:05:51.4325461Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-26T14:05:52.4215662Z ##[command]git config --get-all http.proxy
2020-04-26T14:05:52.4238434Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71321/merge:refs/remotes/pull/71321/merge
---
2020-04-26T14:09:26.8244988Z  ---> f7353ccad5b1
2020-04-26T14:09:26.8245203Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-26T14:09:26.8245517Z  ---> Using cache
2020-04-26T14:09:26.8245791Z  ---> ed38efbaa060
2020-04-26T14:09:26.8246892Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-26T14:09:26.8247966Z  ---> c5008ef7ae8e
2020-04-26T14:09:26.8248132Z Successfully built c5008ef7ae8e
2020-04-26T14:09:26.8248469Z Successfully tagged rust-ci:latest
2020-04-26T14:09:26.8731626Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-26T14:09:26.8731626Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-26T14:09:26.8745356Z Looks like docker image is the same as before, not uploading
2020-04-26T14:09:35.8511915Z [CI_JOB_NAME=mingw-check]
2020-04-26T14:09:35.8747766Z [CI_JOB_NAME=mingw-check]
2020-04-26T14:09:35.8796947Z == clock drift check ==
2020-04-26T14:09:35.8806405Z   local time: Sun Apr 26 14:09:35 UTC 2020
2020-04-26T14:09:36.0151456Z   network time: Sun, 26 Apr 2020 14:09:36 GMT
2020-04-26T14:09:36.0178637Z Starting sccache server...
2020-04-26T14:09:36.1216629Z configure: processing command line
2020-04-26T14:09:36.1216927Z configure: 
2020-04-26T14:09:36.1217802Z configure: rust.parallel-compiler := True
---
2020-04-26T14:11:32.5909108Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-04-26T14:11:33.0238775Z error: cannot specialize on trait `core::marker::Copy`
2020-04-26T14:11:33.0239456Z     --> src/liballoc/rc.rs:1096:1
2020-04-26T14:11:33.0239852Z      |
2020-04-26T14:11:33.0240651Z 1096 | / impl<T: Copy> RcFromSlice<T> for Rc<[T]> {
2020-04-26T14:11:33.0241656Z 1097 | |     #[inline]
2020-04-26T14:11:33.0242572Z 1098 | |     fn from_slice(v: &[T]) -> Self {
2020-04-26T14:11:33.0243544Z 1099 | |         unsafe { Rc::copy_from_slice(v) }
2020-04-26T14:11:33.0245240Z 1101 | | }
2020-04-26T14:11:33.0245716Z      | |_^
2020-04-26T14:11:33.0245879Z 
2020-04-26T14:11:33.0250167Z error: cannot specialize on trait `rc::MarkerEq`
2020-04-26T14:11:33.0250167Z error: cannot specialize on trait `rc::MarkerEq`
2020-04-26T14:11:33.0251255Z     --> src/liballoc/rc.rs:1238:1
2020-04-26T14:11:33.0251938Z      |
2020-04-26T14:11:33.0252731Z 1238 | / impl<T: ?Sized + MarkerEq> RcEqIdent<T> for Rc<T> {
2020-04-26T14:11:33.0253592Z 1239 | |     #[inline]
2020-04-26T14:11:33.0256045Z 1240 | |     fn eq(&self, other: &Rc<T>) -> bool {
2020-04-26T14:11:33.0261280Z 1241 | |         Rc::ptr_eq(self, other) || **self == **other
2020-04-26T14:11:33.0275231Z 1247 | |     }
2020-04-26T14:11:33.0276020Z 1248 | | }
2020-04-26T14:11:33.0276698Z      | |_^
2020-04-26T14:11:33.0277058Z 
2020-04-26T14:11:33.0277058Z 
2020-04-26T14:11:33.0277693Z error: cannot specialize on trait `core::iter::TrustedLen`
2020-04-26T14:11:33.0278403Z     --> src/liballoc/rc.rs:1572:1
2020-04-26T14:11:33.0278973Z      |
2020-04-26T14:11:33.0279785Z 1572 | / impl<T, I: iter::TrustedLen<Item = T>> ToRcSlice<T> for I {
2020-04-26T14:11:33.0281384Z 1573 | |     fn to_rc_slice(self) -> Rc<[T]> {
2020-04-26T14:11:33.0286535Z 1574 | |         // This is the case for a `TrustedLen` iterator.
2020-04-26T14:11:33.0294505Z 1575 | |         let (low, high) = self.size_hint();
2020-04-26T14:11:33.0307873Z 1592 | |     }
2020-04-26T14:11:33.0309286Z 1593 | | }
2020-04-26T14:11:33.0313461Z      | |_^
2020-04-26T14:11:33.0322189Z 
2020-04-26T14:11:33.0322189Z 
2020-04-26T14:11:33.0323088Z error: cannot specialize on trait `core::marker::Copy`
2020-04-26T14:11:33.0324721Z    --> src/liballoc/sync.rs:976:1
2020-04-26T14:11:33.0325128Z     |
2020-04-26T14:11:33.0325733Z 976 | / impl<T: Copy> ArcFromSlice<T> for Arc<[T]> {
2020-04-26T14:11:33.0326382Z 977 | |     #[inline]
2020-04-26T14:11:33.0327072Z 978 | |     fn from_slice(v: &[T]) -> Self {
2020-04-26T14:11:33.0327859Z 979 | |         unsafe { Arc::copy_from_slice(v) }
2020-04-26T14:11:33.0329086Z 981 | | }
2020-04-26T14:11:33.0329551Z     | |_^
2020-04-26T14:11:33.0329712Z 
2020-04-26T14:11:33.0330125Z error: cannot specialize on trait `rc::MarkerEq`
2020-04-26T14:11:33.0330125Z error: cannot specialize on trait `rc::MarkerEq`
2020-04-26T14:11:33.0330653Z     --> src/liballoc/sync.rs:1782:1
2020-04-26T14:11:33.0331466Z      |
2020-04-26T14:11:33.0332094Z 1782 | / impl<T: ?Sized + crate::rc::MarkerEq> ArcEqIdent<T> for Arc<T> {
2020-04-26T14:11:33.0332793Z 1783 | |     #[inline]
2020-04-26T14:11:33.0333488Z 1784 | |     fn eq(&self, other: &Arc<T>) -> bool {
2020-04-26T14:11:33.0334313Z 1785 | |         Arc::ptr_eq(self, other) || **self == **other
2020-04-26T14:11:33.0335523Z 1791 | |     }
2020-04-26T14:11:33.0336275Z 1792 | | }
2020-04-26T14:11:33.0336919Z      | |_^
2020-04-26T14:11:33.0337105Z 
2020-04-26T14:11:33.0337105Z 
2020-04-26T14:11:33.0337614Z error: cannot specialize on trait `core::iter::TrustedLen`
2020-04-26T14:11:33.0338212Z     --> src/liballoc/sync.rs:2123:1
2020-04-26T14:11:33.0338652Z      |
2020-04-26T14:11:33.0339689Z 2123 | / impl<T, I: iter::TrustedLen<Item = T>> ToArcSlice<T> for I {
2020-04-26T14:11:33.0340472Z 2124 | |     fn to_arc_slice(self) -> Arc<[T]> {
2020-04-26T14:11:33.0341335Z 2125 | |         // This is the case for a `TrustedLen` iterator.
2020-04-26T14:11:33.0342256Z 2126 | |         let (low, high) = self.size_hint();
2020-04-26T14:11:33.0343564Z 2143 | |     }
2020-04-26T14:11:33.0344143Z 2144 | | }
2020-04-26T14:11:33.0344632Z      | |_^
2020-04-26T14:11:33.0344795Z 
2020-04-26T14:11:33.0344795Z 
2020-04-26T14:11:33.0345242Z error: cannot specialize on trait `vec::IsZero`
2020-04-26T14:11:33.0345847Z     --> src/liballoc/vec.rs:1816:1
2020-04-26T14:11:33.0346218Z      |
2020-04-26T14:11:33.0346789Z 1816 | / impl<T: Clone + IsZero> SpecFromElem for T {
2020-04-26T14:11:33.0347416Z 1817 | |     #[inline]
2020-04-26T14:11:33.0348112Z 1818 | |     fn from_elem(elem: T, n: usize) -> Vec<T> {
2020-04-26T14:11:33.0348798Z 1819 | |         if elem.is_zero() {
2020-04-26T14:11:33.0349914Z 1825 | |     }
2020-04-26T14:11:33.0350601Z 1826 | | }
2020-04-26T14:11:33.0351111Z      | |_^
2020-04-26T14:11:33.0351298Z 
2020-04-26T14:11:33.0351298Z 
2020-04-26T14:11:33.0386076Z error: cannot specialize on trait `core::iter::TrustedLen`
2020-04-26T14:11:33.0386771Z     --> src/liballoc/vec.rs:2087:1
2020-04-26T14:11:33.0387190Z      |
2020-04-26T14:11:33.0387764Z 2087 | / impl<T, I> SpecExtend<T, I> for Vec<T>
2020-04-26T14:11:33.0388398Z 2088 | | where
2020-04-26T14:11:33.0389072Z 2089 | |     I: TrustedLen<Item = T>,
2020-04-26T14:11:33.0389690Z 2090 | | {
2020-04-26T14:11:33.0390684Z 2123 | |     }
2020-04-26T14:11:33.0391231Z 2124 | | }
2020-04-26T14:11:33.0391710Z      | |_^
2020-04-26T14:11:33.0391872Z 
2020-04-26T14:11:33.0391872Z 
2020-04-26T14:11:33.0392293Z error: cannot specialize on trait `core::marker::Copy`
2020-04-26T14:11:33.0392977Z     --> src/liballoc/vec.rs:2165:1
2020-04-26T14:11:33.0393385Z      |
2020-04-26T14:11:33.0394005Z 2165 | / impl<'a, T: 'a> SpecExtend<&'a T, slice::Iter<'a, T>> for Vec<T>
2020-04-26T14:11:33.0394678Z 2166 | | where
2020-04-26T14:11:33.0396002Z 2168 | | {
2020-04-26T14:11:33.0396482Z ...    |
2020-04-26T14:11:33.0396982Z 2178 | |     }
2020-04-26T14:11:33.0397529Z 2179 | | }
---
2020-04-26T14:11:33.0453011Z error: could not compile `alloc`.
2020-04-26T14:11:33.0462130Z 
2020-04-26T14:11:33.0462720Z To learn more, run the command again with --verbose.
2020-04-26T14:11:33.0463456Z warning: build failed, waiting for other jobs to finish...
2020-04-26T14:11:33.5258558Z {"reason":"build-finished","success":false}
2020-04-26T14:11:33.5346905Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-26T14:11:33.5347599Z expected success, got: exit code: 101
2020-04-26T14:11:33.5357249Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-26T14:11:33.5357577Z Build completed unsuccessfully in 0:01:57
2020-04-26T14:11:33.5357577Z Build completed unsuccessfully in 0:01:57
2020-04-26T14:11:33.5454084Z == clock drift check ==
2020-04-26T14:11:33.5469717Z   local time: Sun Apr 26 14:11:33 UTC 2020
2020-04-26T14:11:33.8120506Z   network time: Sun, 26 Apr 2020 14:11:33 GMT
2020-04-26T14:11:35.0604916Z 
2020-04-26T14:11:35.0604916Z 
2020-04-26T14:11:35.0671560Z ##[error]Bash exited with code '1'.
2020-04-26T14:11:35.0685054Z ##[section]Finishing: Run build
2020-04-26T14:11:35.0734153Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71321/merge to s
2020-04-26T14:11:35.0739230Z Task         : Get sources
2020-04-26T14:11:35.0739529Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-26T14:11:35.0739796Z Version      : 1.0.0
2020-04-26T14:11:35.0740115Z Author       : Microsoft
2020-04-26T14:11:35.0740115Z Author       : Microsoft
2020-04-26T14:11:35.0740407Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-26T14:11:35.0740729Z ==============================================================================
2020-04-26T14:11:35.4098771Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-26T14:11:35.4150671Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71321/merge to s
2020-04-26T14:11:35.4236174Z Cleaning up task key
2020-04-26T14:11:35.4237305Z Start cleaning up orphan processes.
2020-04-26T14:11:35.4446466Z Terminate orphan process: pid (3504) (python)
2020-04-26T14:11:35.4636585Z ##[section]Finishing: Finalize Job
