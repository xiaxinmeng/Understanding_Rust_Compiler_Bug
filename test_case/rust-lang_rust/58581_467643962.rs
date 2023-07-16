plain
travis_time:end:0dbb2618:start=1551216252251260506,finish=1551216253180761047,duration=929500541
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/4c/cf/9935187b758dccb7a85ee697d3008fe9ea50c5f0e7ae56a76caec0a46954/awscli-1.16.113-py2.py3-none-any.whl (1.4MB)
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.0MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
    3% |█▏                              | 51kB 2.4MB/s eta 0:00:01
---
[01:11:14] .................................................................................................... 1400/5417
[01:11:17] .................................................................................................... 1500/5417
[01:11:19] .................................................................................................... 1600/5417
[01:11:23] ...............i.................................................................................... 1700/5417
[01:11:26] ...................................................................F................................ 1800/5417
[01:11:34] .................................................................................................... 2000/5417
[01:11:37] ..................................................i................................................. 2100/5417
[01:11:42] .................................................................................................... 2200/5417
[01:11:46] .................................................................................................... 2300/5417
---
[01:13:49] failures:
[01:13:49] 
[01:13:49] ---- [ui] ui/impl-trait/impl-generic-mismatch.rs stdout ----
[01:13:49] 
[01:13:49] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:13:49] status: exit code: 101
[01:13:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/auxiliary" "-A" "unused"
[01:13:49] ------------------------------------------
[01:13:49] 
[01:13:49] ------------------------------------------
[01:13:49] stderr:
[01:13:49] stderr:
[01:13:49] ------------------------------------------
[01:13:49] {"message":"method `foo` has incompatible signature for trait","code":{"code":"E0643","explanation":"\nThis error indicates that there is a mismatch between generic parameters and\nimpl Trait parameters in a trait declaration versus its impl.\n\n