plain
travis_time:end:04e51579:start=1555535594957630027,finish=1555535734954411176,duration=139996781149
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
    15% |█████                           | 20kB 36.9MB/s eta 0:00:01
    23% |███████▌                        | 30kB 42.5MB/s eta 0:00:01
    31% |██████████                      | 40kB 45.7MB/s eta 0:00:01
    38% |████████████▍                   | 51kB 46.5MB/s eta 0:00:01
---
[00:24:07]    Compiling itertools v0.8.0
[00:24:08] error[E0599]: no function or associated item named `default` found for type `_` in the current scope
[00:24:08]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/itertools-0.8.0/src/concat_impl.rs:21:86
[00:24:08]    |
[00:24:08] 21 |     iterable.into_iter().fold1(|mut a, b| { a.extend(b); a }).unwrap_or_else(|| <_>::default())
[00:24:08]    |                                                                                      ^^^^^^^ function or associated item not found in `_`
[00:24:08]    = help: items from traits can only be used if the trait is in scope
[00:24:08]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:24:08]            `use std::default::Default;`
[00:24:08] help: you are looking for the module in `std`, not the primitive type
[00:24:08] help: you are looking for the module in `std`, not the primitive type
[00:24:08]    |
[00:24:08] 21 |     iterable.into_iter().fold1(|mut a, b| { a.extend(b); a }).unwrap_or_else(|| std::<_>::default())
[00:24:08] 
[00:24:09] error: aborting due to previous error
[00:24:09] 
[00:24:09] For more information about this error, try `rustc --explain E0599`.
