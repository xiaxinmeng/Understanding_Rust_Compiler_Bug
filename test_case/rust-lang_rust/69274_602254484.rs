plain
2020-03-22T17:48:41.0168059Z ========================== Starting Command Output ===========================
2020-03-22T17:48:41.0172318Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f14a6c45-69fb-4a71-a8d8-1d35dadb9c56.sh
2020-03-22T17:48:41.0172630Z 
2020-03-22T17:48:41.0177610Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T17:48:41.0198305Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-22T17:48:41.0201824Z Task         : Get sources
2020-03-22T17:48:41.0202150Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T17:48:41.0202465Z Version      : 1.0.0
2020-03-22T17:48:41.0202696Z Author       : Microsoft
---
2020-03-22T17:48:42.0047313Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T17:48:42.0053204Z ##[command]git config gc.auto 0
2020-03-22T17:48:42.0057732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T17:48:42.0061642Z ##[command]git config --get-all http.proxy
2020-03-22T17:48:42.0072007Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-03-22T17:57:37.5719490Z configure: build.locked-deps    := True
2020-03-22T17:57:37.5720020Z configure: llvm.ccache          := sccache
2020-03-22T17:57:37.5720756Z configure: build.cargo-native-static := True
2020-03-22T17:57:37.5721591Z configure: dist.missing-tools   := True
2020-03-22T17:57:37.5723198Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-22T17:57:37.5724824Z configure: writing `config.toml` in current directory
2020-03-22T17:57:37.5725157Z configure: 
2020-03-22T17:57:37.5725734Z configure: run `python /checkout/x.py --help`
2020-03-22T17:57:37.5726077Z configure: 
---
2020-03-22T18:05:40.3490090Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-22T18:05:42.5275588Z Diff in /checkout/src/librustc_passes/check_attr.rs at line 210:
2020-03-22T18:05:42.5276017Z      }
2020-03-22T18:05:42.5276217Z  
2020-03-22T18:05:42.5276605Z      /// Checks if the `#[target_feature]` attribute on `item` is valid. Returns `true` if valid.
2020-03-22T18:05:42.5278334Z -    fn check_target_feature(&self, attr: &Attribute, hir_id: HirId, span: &Span, target: Target) -> bool {
2020-03-22T18:05:42.5279074Z +        &self,
2020-03-22T18:05:42.5279295Z +        attr: &Attribute,
2020-03-22T18:05:42.5279556Z +        hir_id: HirId,
2020-03-22T18:05:42.5279791Z +        span: &Span,
2020-03-22T18:05:42.5279791Z +        span: &Span,
2020-03-22T18:05:42.5280028Z +        target: Target,
2020-03-22T18:05:42.5280546Z +    ) -> bool {
2020-03-22T18:05:42.5280763Z          match target {
2020-03-22T18:05:42.5280995Z              Target::Fn
2020-03-22T18:05:42.5281355Z              | Target::Method(MethodKind::Trait { body: true })
2020-03-22T18:05:42.5281767Z Diff in /checkout/src/librustc_passes/check_attr.rs at line 217:
2020-03-22T18:05:42.5282404Z -            | Target::Method(MethodKind::Inherent) => {},
2020-03-22T18:05:42.5282830Z +            | Target::Method(MethodKind::Inherent) => {}
2020-03-22T18:05:42.5283357Z                  self.tcx
2020-03-22T18:05:42.5283613Z                      .sess
2020-03-22T18:05:42.5283923Z Diff in /checkout/src/librustc_passes/check_attr.rs at line 228:
2020-03-22T18:05:42.5283923Z Diff in /checkout/src/librustc_passes/check_attr.rs at line 228:
2020-03-22T18:05:42.5292111Z          let def_id = self.tcx.hir().local_def_id(hir_id);
2020-03-22T18:05:42.5292487Z          if !self.tcx.features().target_feature_11 {
2020-03-22T18:05:42.5292784Z              // Check that function is unsafe
2020-03-22T18:05:42.5293748Z -            if self.tcx.is_closure(def_id) || self.tcx.fn_sig(def_id).unsafety() == hir::Unsafety::Normal {
2020-03-22T18:05:42.5294207Z +            if self.tcx.is_closure(def_id)
2020-03-22T18:05:42.5294576Z +                || self.tcx.fn_sig(def_id).unsafety() == hir::Unsafety::Normal
2020-03-22T18:05:42.5295120Z                  let mut err = feature_err(
2020-03-22T18:05:42.5295396Z                      &self.tcx.sess.parse_sess,
2020-03-22T18:05:42.5295684Z                      sym::target_feature_11,
2020-03-22T18:05:42.5296027Z Diff in /checkout/src/librustc_passes/check_attr.rs at line 242:
2020-03-22T18:05:42.5296027Z Diff in /checkout/src/librustc_passes/check_attr.rs at line 242:
2020-03-22T18:05:42.5296710Z          } else if self.tcx.fn_sig(def_id).unsafety() == hir::Unsafety::Normal {
2020-03-22T18:05:42.5297152Z              // Check that function is unsafe if it is a trait implementation
2020-03-22T18:05:42.5297511Z              let node = self.tcx.hir().get(hir_id);
2020-03-22T18:05:42.5298279Z -            if let hir::Node::ImplItem(hir::ImplItem { kind: hir::ImplItemKind::Fn(..), .. }) = node {
2020-03-22T18:05:42.5298850Z +            if let hir::Node::ImplItem(hir::ImplItem { kind: hir::ImplItemKind::Fn(..), .. }) = node
2020-03-22T18:05:42.5299227Z +            {
2020-03-22T18:05:42.5299495Z                  let parent_id = self.tcx.hir().get_parent_item(hir_id);
2020-03-22T18:05:42.5299865Z                  let parent_item = self.tcx.hir().expect_item(parent_id);
2020-03-22T18:05:42.5300309Z                  if let hir::ItemKind::Impl { of_trait: Some(_), .. } = parent_item.kind {
2020-03-22T18:05:42.5300728Z Diff in /checkout/src/librustc_passes/check_attr.rs at line 249:
2020-03-22T18:05:42.5301216Z -                    self.tcx.sess
2020-03-22T18:05:42.5301465Z +                    self.tcx
2020-03-22T18:05:42.5301684Z +                        .sess
2020-03-22T18:05:42.5301927Z                          .struct_span_err(
2020-03-22T18:05:42.5302201Z                              attr.span,
2020-03-22T18:05:42.5302734Z                              "`#[target_feature(..)]` cannot be applied to safe trait method",
2020-03-22T18:05:42.5303836Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_passes/check_attr.rs"` failed.
2020-03-22T18:05:42.5304852Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-22T18:05:42.5313631Z Build completed unsuccessfully in 0:00:39
2020-03-22T18:05:42.5367263Z == clock drift check ==
2020-03-22T18:05:42.5385183Z   local time: Sun Mar 22 18:05:42 UTC 2020
2020-03-22T18:05:43.3702453Z   network time: Sun, 22 Mar 2020 18:05:42 GMT
2020-03-22T18:05:43.3702453Z   network time: Sun, 22 Mar 2020 18:05:42 GMT
2020-03-22T18:05:43.3703466Z == end clock drift check ==
2020-03-22T18:05:44.1595391Z 
2020-03-22T18:05:44.1679021Z ##[error]Bash exited with code '1'.
2020-03-22T18:05:44.1695156Z ##[section]Finishing: Run build
2020-03-22T18:05:44.1767878Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-22T18:05:44.1772450Z Task         : Get sources
2020-03-22T18:05:44.1772766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T18:05:44.1773041Z Version      : 1.0.0
2020-03-22T18:05:44.1773234Z Author       : Microsoft
2020-03-22T18:05:44.1773234Z Author       : Microsoft
2020-03-22T18:05:44.1773557Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T18:05:44.1773913Z ==============================================================================
2020-03-22T18:05:44.5467196Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T18:05:44.5519718Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-22T18:05:44.5620349Z Cleaning up task key
2020-03-22T18:05:44.5621779Z Start cleaning up orphan processes.
2020-03-22T18:05:44.5832095Z Terminate orphan process: pid (3308) (python)
2020-03-22T18:05:44.6179486Z ##[section]Finishing: Finalize Job
