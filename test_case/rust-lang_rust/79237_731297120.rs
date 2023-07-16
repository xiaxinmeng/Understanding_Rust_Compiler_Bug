plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +d868d2c3cc3c076705b9f0ffe66bcbf0a50283d1:refs/remotes/pull/79237/merge
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
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
error: hidden lifetime parameters in types are deprecated
   --> library/std/src/../../backtrace/src/symbolize/gimli/coff.rs:104:68
    |
104 |     pub(super) fn search_object_map(&self, _addr: u64) -> Option<(&Context, u64)> {
    |                                                                    ^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
    |
    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:32
== clock drift check ==
  local time: Fri Nov 20 17:21:01 UTC 2020
