plain
travis_time:end:0b915fea:start=1543353817651526269,finish=1543353871544997340,duration=53893471071
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:18] .................................................................................................... 1800/5064
[00:47:21] .................................................................................................... 1900/5064
[00:47:24] ..........................i......................................................................... 2000/5064
[00:47:28] .................................................................................................... 2100/5064
[00:47:32] .........F.......................................................................................... 2200/5064
[00:47:39] .................................................................................................... 2400/5064
[00:47:42] .................................................................................................... 2500/5064
[00:47:46] .................................................................................................... 2600/5064
[00:47:50] .................................................................................................... 2700/5064
---
[00:48:58] 
[00:48:58] ---- [ui] ui/issues/issue-21763.rs stdout ----
[00:48:58] diff of stderr:
[00:48:58] 
[00:48:58] 4 LL |     foo::<HashMap<Rc<()>, Rc<()>>>();
[00:48:58] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
[00:48:58] 6    |
[00:48:58] -    = help: within `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
[00:48:58] +    = help: within `(std::rc::Rc<()>, std::rc::Rc<()>)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
[00:48:58] 8    = note: required because it appears within the type `(std::rc::Rc<()>, std::rc::Rc<()>)`
[00:48:58] -    = note: required because it appears within the type `std::marker::PhantomData<(std::rc::Rc<()>, std::rc::Rc<()>)>`
[00:48---------------------------------------
[00:48:58] ------------------------------------------
[00:48:58] ------------------------------------------
[00:48:58] {"message":"`std::rc::Rc<()>` cannot be sent between threads safely","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n