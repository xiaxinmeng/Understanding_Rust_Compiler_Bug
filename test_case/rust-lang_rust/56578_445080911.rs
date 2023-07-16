plain
travis_time:end:00b2ce0b:start=1544142348311954434,finish=1544142351321123440,duration=3009169006
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ff/94/838f88e12e6d3aadb427955e657f3fe3c763ee1ad9290d4601a287fed7d1/awscli-1.16.71-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 37.0MB/s eta 0:00:01
    1% |▌                               | 20kB 2.2MB/s eta 0:00:01
    2% |▊                               | 30kB 3.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.1MB/s eta 0:00:01
---
[00:05:04]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:04]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:08]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:28]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:29] error[E0252]: the name `hir` is defined multiple times
[00:06:29]   --> src/librustc/traits/util.rs:19:11
[00:06:29] 12 | use hir::{self};
[00:06:29] 12 | use hir::{self};
[00:06:29]    |           ---- previous import of the module `hir` here
[00:06:29] 19 | use hir::{self};
[00:06:29] 19 | use hir::{self};
[00:06:29]    |           ^^^^ `hir` reimported here
[00:06:29]    |
[00:06:29]    = note: `hir` must be defined only once in the type namespace of this module
[00:06:29] help: you can use `as` to change the binding name of the import
[00:06:29]    |
[00:06:29] 19 | use hir::{self as other_hir};
[00:06:29] 
[00:06:29] error[E0252]: the name `Debug` is defined multiple times
[00:06:29]   --> src/librustc/traits/mod.rs:50:5
[00:06:29]    |
[00:06:29]    |
[00:06:29] 47 | use std::fmt::Debug;
[00:06:29]    |     --------------- previous import of the trait `Debug` here
[00:06:29] 50 | use std::fmt::Debug;
[00:06:29] 50 | use std::fmt::Debug;
[00:06:29]    |     ^^^^^^^^^^^^^^^ `Debug` reimported here
[00:06:29]    |
[00:06:29]    = note: `Debug` must be defined only once in the type namespace of this module
[00:06:29] help: you can use `as` to change the binding name of the import
[00:06:29]    |
[00:06:29] 50 | use std::fmt::Debug as OtherDebug;
[00:06:29] 
[00:06:29] 
[00:06:29] error[E0252]: the name `Rc` is defined multiple times
[00:06:29]    |
[00:06:29] 48 | use std::rc::Rc;
[00:06:29] 48 | use std::rc::Rc;
[00:06:29]    |     ----------- previous import of the type `Rc` here
[00:06:29] 51 | use std::rc::Rc;
[00:06:29] 51 | use std::rc::Rc;
[00:06:29]    |     ^^^^^^^^^^^ `Rc` reimported here
[00:06:29]    |
[00:06:29]    = note: `Rc` must be defined only once in the type namespace of this module
[00:06:29] help: you can use `as` to change the binding name of the import
[00:06:29]    |
[00:06:29] 51 | use std::rc::Rc as OtherRc;
[00:06:29] 
[00:06:29] 
[00:06:31] error[E0432]: unresolved imports `self::util::expand_trait_refs`, `self::util::TraitRefExpander`, `self::util::TraitRefExpansionInfoDignosticBuilder`
[00:06:31]    |
[00:06:31]    |
[00:06:31] 74 | pub use self::util::{expand_trait_refs, TraitRefExpander, TraitRefExpansionInfoDignosticBuilder};
[00:06:31]    |                      ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TraitRefExpansionInfoDignosticBuilder` in `traits::util`
[00:06:31]    |                      |                  |
[00:06:31]    |                      |                  no `TraitRefExpander` in `traits::util`
[00:06:31]    |                      no `expand_trait_refs` in `traits::util`
[00:06:32] error[E0412]: cannot find type `Substs` in this scope
[00:06:32]    --> src/librustc/traits/util.rs:374:63
[00:06:32]     |
[00:06:32]     |
[00:06:32] 374 |                                                 impl_substs: &Substs<'tcx>)
[00:06:32] help: possible candidate is found in another module, you can import it into scope
[00:06:32]     |
[00:06:32] 11  | use ty::subst::Substs;
[00:06:32]     |
[00:06:32]     |
[00:06:32] 
[00:06:32] error[E0412]: cannot find type `Kind` in this scope
[00:06:32]    --> src/librustc/traits/util.rs:441:46
[00:06:32]     |
[00:06:32] 441 |                                    params: &[Kind<'tcx>])
[00:06:32] help: possible candidate is found in another module, you can import it into scope
[00:06:32]     |
[00:06:32] 11  | use ty::subst::Kind;
[00:06:32]     |
---
[00:06:33]    |
[00:06:33] 51 | use std::rc::Rc;
[00:06:33]    |     ^^^^^^^^^^^
[00:06:33] 
[00:06:55] error[E0599]: no method named `subst` found for type `ty::sty::TraitRef<'_>` in the current scope
[00:06:55]    --> src/librustc/traits/util.rs:381:24
[00:06:55]     |
[00:06:55] 381 |         impl_trait_ref.subst(selcx.tcx(), impl_substs);
[00:06:55]     | 
[00:06:55]     | 
[00:06:55]    ::: src/librustc/ty/sty.rs:649:1
[00:06:55] 649 | pub struct TraitRef<'tcx> {
[00:06:55] 649 | pub struct TraitRef<'tcx> {
[00:06:55]     | ------------------------- method `subst` not found for this
[00:06:55]     = help: items from traits can only be used if the trait is in scope
[00:06:55]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:06:55]             `use ty::subst::Subst;`
[00:06:55] 
---
[00:07:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:01] expected success, got: exit code: 101
[00:07:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:01] Build completed unsuccessfully in 0:03:52
[00:07:01] make: *** [all] Error 1
[00:07:01] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c7e2b5c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 00:33:01 UTC 2018
