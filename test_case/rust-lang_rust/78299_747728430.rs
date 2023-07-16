plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=seanchen1991
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_249d1ac0-901f-4495-acb2-144733715881
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=master
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_249d1ac0-901f-4495-acb2-144733715881
GITHUB_REF=refs/pull/78299/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=429132655
GITHUB_RUN_NUMBER=21583
---
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Failed building wheel for PyYAML
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-cwqt4sc4/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpw6r8snlkpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
    Checking hashbrown v0.9.0
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error[E0520]: `Item` specializes an item from a parent `impl`, but that item is not marked `default`
    |
    |
520 |     type Item = BacktraceFrame;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `Item`
    = note: parent implementation is in crate `core`


error[E0520]: `IntoIter` specializes an item from a parent `impl`, but that item is not marked `default`
    |
    |
521 |     type IntoIter = alloc_crate::vec::IntoIter<Self::Item>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `IntoIter`
    = note: parent implementation is in crate `core`


error[E0520]: `into_iter` specializes an item from a parent `impl`, but that item is not marked `default`
    |
523 | /     fn into_iter(self) -> Self::IntoIter {
524 | |         self.inner.into_iter()
525 | |     }
525 | |     }
    | |_____^ cannot specialize default item `into_iter`
    = note: parent implementation is in crate `core`

error: aborting due to 3 previous errors


For more information about this error, try `rustc --explain E0520`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:40
== clock drift check ==
  local time: Thu Dec 17 22:01:02 UTC 2020
