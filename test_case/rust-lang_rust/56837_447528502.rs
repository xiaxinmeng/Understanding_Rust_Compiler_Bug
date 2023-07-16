plain
travis_time:end:0931346a:start=1544837382368592687,finish=1544837459535084865,duration=77166492178
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 61kB 10.2MB/s 
Collecting botocore==1.12.66 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/51/da/3ed787b6ca3d33f626c1ba4e014449825db0d557981c4bef71f886fb1424/botocore-1.12.66-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 42.1MB/s eta 0:00:01
    0% |▏                               | 20kB 42.8MB/s eta 0:00:01
    0% |▏                               | 30kB 48.8MB/s eta 0:00:01
    0% |▎                               | 40kB 52.8MB/s eta 0:00:01
---
[00:34:37]    Compiling rand v0.6.1
[00:34:40]    Compiling parking_lot_core v0.3.0
[00:34:42]    Compiling tempfile v3.0.5
[00:34:42]    Compiling parking_lot v0.6.4
[00:34:46] error[E0599]: no method named `def_id` found for type `std::option::Option<rustc::ty::Binder<rustc::ty::ExistentialTraitRef<'_>>>` in the current scope
[00:34:46]     --> src/librustdoc/clean/mod.rs:2642:37
[00:34:46] 2642 |                 let did = principal.def_id();
[00:34:46]      |                                     ^^^^^^
[00:34:46]      |
[00:34:46]      |
[00:34:46]      = note: the method `def_id` exists but the following trait bounds were not satisfied:
[00:34:46]              `std::option::Option<rustc::ty::Binder<rustc::ty::ExistentialTraitRef<'_>>> : clean::GetDefId`
[00:34:46]      = help: items from traits can only be used if the trait is implemented and in scope
[00:34:46]      = note: the following trait defines an item `def_id`, perhaps you need to implement it:
[00:34:46]              candidate #1: `clean::GetDefId`
[00:34:46] 
[00:34:46] error[E0599]: no method named `skip_binder` found for type `std::option::Option<rustc::ty::Binder<rustc::ty::ExistentialTraitRef<'_>>>` in the current scope
[00:34:46]     --> src/librustdoc/clean/mod.rs:2673:48
[00:34:4
