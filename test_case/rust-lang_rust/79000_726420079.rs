plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +4664b982e6f1908bf49adbca1b0c87b64b9383cf:refs/remotes/pull/79000/merge
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
Diff in /checkout/compiler/rustc_span/src/lev_distance.rs at line 56:
         .iter()
         .filter_map(|&name| {
             let dist = lev_distance(lookup, &name.as_str());
-            if dist <= max_dist {
-                Some((name, dist))
-                None
-            }
-            }
+            if dist <= max_dist { Some((name, dist)) } else { None }
         })
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/lev_distance.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         // Here we are collecting the next structure:
         // (case_insensitive_match, (levenshtein_match, levenshtein_distance))
Build completed unsuccessfully in 0:00:15
== clock drift check ==
  local time: Fri Nov 13 00:12:04 UTC 2020
  network time: Thu, 12 Nov 2020 00:26:09 GMT
