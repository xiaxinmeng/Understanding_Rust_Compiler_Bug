plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/77285/merge:refs/remotes/pull/77285/merge
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
Average cache write          0.000 s
Average cache read miss      0.000 s
Average cache read hit       0.078 s
Cache location             S3, bucket: Bucket(name=rust-lang-ci-sccache2, base_url=http://rust-lang-ci-sccache2.s3.amazonaws.com/)
cat: 'build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/*.ll': No such file or directory
== printing ll file ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6268) (node)
Terminate orphan process: pid (6277) (python)
