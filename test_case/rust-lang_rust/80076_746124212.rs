plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +7beb6ef80dc2125fd26d93ecb57e0e5b5a281743:refs/remotes/pull/80076/merge
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
   Compiling tracing v0.1.19
   Compiling tracing-subscriber v0.2.13
   Compiling rustfix v0.5.1
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0063]: missing field `output_location` in initializer of `TestOpts`
    |
464 |     test::TestOpts {
    |     ^^^^^^^^^^^^^^ missing `output_location`

