plain
travis_time:end:0e602870:start=1555538071731456356,finish=1555538073903982712,duration=2172526356
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting futures<4.0.0,>=2.2.0; python_version == "2.6" or python_version == "2.7" (from s3transfer<0.3.0,>=0.2.0->awscli)
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Collecting urllib3<1.25,>=1.20; python_version == "2.7" (from botocore==1.12.132->awscli)
  Downloading https://files.pythonhosted.org/packages/df/1c/59cca3abf96f991f2ec3131a4ffe72ae3d9ea1f5894abe8a9c5e3c77cfee/urllib3-1.24.2-py2.py3-none-any.whl (131kB)
    15% |█████                           | 20kB 26.3MB/s eta 0:00:01
    23% |███████▌                        | 30kB 32.1MB/s eta 0:00:01
    31% |██████████                      | 40kB 34.8MB/s eta 0:00:01
    38% |████████████▍                   | 51kB 36.6MB/s eta 0:00:01
---
[00:08:09]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:46] error: unused variable: `value1`
[00:08:46]    --> src/librustc/infer/unify_key.rs:134:52
[00:08:46]     |
[00:08:46] 134 |                 ConstVariableValue::Known { value: value1 },
[00:08:46]     |                                                    ^^^^^^ help: consider prefixing with an underscore: `_value1`
[00:08:46]     = note: `-D unused-variables` implied by `-D warnings`
[00:08:46] 
[00:08:46] error: unused variable: `value2`
[00:08:46]    --> src/librustc/infer/unify_key.rs:135:52
[00:08:46]    --> src/librustc/infer/unify_key.rs:135:52
[00:08:46]     |
[00:08:46] 135 |                 ConstVariableValue::Known { value: value2 }
[00:08:46]     |                                                    ^^^^^^ help: consider prefixing with an underscore: `_value2`
[00:08:47] error: aborting due to 2 previous errors
[00:08:47] 
[00:08:47] error: Could not compile `rustc`.
[00:08:47] 
---
205996 ./obj/build/cache/2019-04-11
157492 ./obj/build/bootstrap/debug/incremental
156496 ./src/llvm-project/clang
142508 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc
142504 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc/s-fbe4eia0b3-5jds3l-3k7gym5px36ja
108532 ./src/llvm-project/lldb
106696 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
97584 ./src/llvm-project/clang/test
96736 ./.git
