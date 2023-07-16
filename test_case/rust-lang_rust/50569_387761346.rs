plain
[01:26:09] travis_fold:start:test_stage1-rustc
travis_time:start:test_stage1-rustc
Testing rustc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:09]    Compiling rustc v0.0.0 (file:///checkout/src/librustc)
[01:26:13] error[E0433]: failed to resolve. Use of undeclared type or module `CrossLangLto`
[01:26:13]     --> librustc/session/config.rs:3159:46
[01:26:13]      |
[01:26:13] 3159 |         opts.debugging_opts.cross_lang_lto = CrossLangLto::NoLink;
[01:26:13]      |                                              ^^^^^^^^^^^^ Use of undeclared type or module `CrossLangLto`
travis_time:end:15c363b6:start=1525871517867028686,finish=1525876723455167254,duration=5205588138568

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03ecd41a
