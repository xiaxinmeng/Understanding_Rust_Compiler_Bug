plain
travis_time:end:0347da80:start=1543592600735758224,finish=1543592603160872561,duration=2425114337
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:24] .................................................................................................... 2100/5102
[00:47:28] .................................................................................................... 2200/5102
[00:47:31] .................................................................................................... 2300/5102
[00:47:35] .................................................................................................... 2400/5102
[00:47:39] ...................................................F................................................ 2500/5102
[00:47:46] .................................................................................................... 2700/5102
[00:47:50] .................................................................................................... 2800/5102
[00:47:53] .................................................................................................... 2900/5102
[00:47:56] .................................................................................................... 3000/5102
---
[00:48:52] .................................................................i.................................. 4700/5102
[00:48:55] .................................................................................................... 4800/5102
[00:48:58] .................................................................................................... 4900/5102
[00:49:01] .................................................................................................... 5000/5102
pected identifier, found keyword `Self`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-36638.rs","byte_start":527,"byte_end":531,"line_start":13,"line_end":13,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"struct Foo<Self>(Self);","highlight_start":12,"highlight_end":16}],"label":"expected identifier, found keyword","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected identifier, found keyword `Self`\n  --> /checkout/src/test/ui/issues/issue-36638.rs:13:12\n   |\nLL | struct Foo<Self>(Self);\n   |            ^^^^ expected identifier, found keyword\n\n"}
[00:49:04] {"message":"expected identifier, found keyword `Self`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-36638.rs","byte_start":604,"byte_end":608,"line_start":16,"line_end":16,"column_start":11,"column_end":15,"is_primary":true,"text":[{"text":"trait Bar<Self> {}","highlight_start":11,"highlight_end":15}],"label":"expected identifier, found keyword","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected identifier, found keyword `Self`\n  --> /checkout/src/test/ui/issues/issue-36638.rs:16:11\n   |\nLL | trait Bar<Self> {}\n   |           ^^^^ expected identifier, found keyword\n\n"}
[00:49:04] {"message":"parameter `Self` is never used","code":{"code":"E0392","explanation":"\nThis error indicates that a type or lifetime parameter has been declared\nbut not actually used. Here is an example that demonstrates the error:\n\n