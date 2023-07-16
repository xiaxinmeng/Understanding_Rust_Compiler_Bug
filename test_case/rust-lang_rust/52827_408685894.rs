plain
[00:32:11]    Compiling aho-corasick v0.6.6
[00:32:15]    Compiling tempfile v3.0.2
[00:32:33]    Compiling minifier v0.0.14
[00:32:35]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:32:36] error: trait objects without an explicit `dyn` are deprecated
[00:32:36]   --> librustdoc/clean/def_ctor.rs:19:14
[00:32:36]    |
[00:32:36] 19 | where F: Fn(&Fn(DefId) -> Def) -> Vec<Item> {
[00:32:36]    |              ^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn(DefId) -> Def`
[00:32:36]    |
[00:32:36]    = note: requested on the command line with `-D bare-trait-objects`
[00:32:36] 
[00:32:36] error: trait objects without an explicit `dyn` are deprecated
[00:32:36]   --> librustdoc/clean/def_ctor.rs:56:14
[00:32:36]    |
[00:32:36] 56 | where F: Fn(&Fn(DefId) -> Def, String) -> Vec<Item> {
[00:32:36]    |              ^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn(DefId) -> Def`
travis_time:end:19d124f0:start=1532876703082642932,finish=1532878662741449220,duration=1959658806288

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0acacda8
