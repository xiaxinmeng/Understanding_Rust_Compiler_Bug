plain
travis_time:end:0e06134c:start=1546163949861786982,finish=1546164007111771877,duration=57249984895
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:09] .................................................................................................... 4500/5211
[01:02:12] .................................................................................................... 4600/5211
[01:02:15] .................................................................................................... 4700/5211
[01:02:19] .................................................................................................... 4800/5211
[01:02:23] ......................................F............................................................. 4900/5211
[01:02:31] ..................................................i................................................. 5200/5211
obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/auxiliary" "-A" "unused"
[01:02:32] stdout:
[01:02:32] ------------------------------------------
[01:02:32] ------------------------------------------
[01:02:32] 
[01:02:32] ------------------------------------------
[01:02:32] stderr:
[01:02:32] ------------------------------------------
[01:02:32] {"message":"cannot transmute between types of different sizes, or dependently-sized types","code":{"code":"E0512","explanation":"\nTransmute with two differently sized types was attempted. Erroneous code\nexample:\n\n