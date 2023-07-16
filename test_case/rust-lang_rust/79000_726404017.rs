plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +8bcd6d407b79021af614497296a568c5bf721032:refs/remotes/pull/79000/merge
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/lev_distance/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/compiler/rustc_span/src/lev_distance/tests.rs at line 29:
             Some(Symbol::intern("aaab"))
         );
-        assert_eq!(
-        assert_eq!(
-            find_best_match_for_name(&input, Symbol::intern("1111111111"), None),
-            None
-        );
+        assert_eq!(find_best_match_for_name(&input, Symbol::intern("1111111111"), None), None);
 
         let input = vec![Symbol::intern("aAAA")];
         assert_eq!(
Build completed unsuccessfully in 0:00:13
== clock drift check ==
  local time: Thu Nov 12 23:28:00 UTC 2020
  network time: Thu, 12 Nov 2020 09:26:07 GMT
