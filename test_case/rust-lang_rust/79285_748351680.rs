plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=yoshuawuyts
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_ca8f1ad1-b608-4e37-850c-af461e1f31bc
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=stabilize-arc_mutate_strong_count
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_ca8f1ad1-b608-4e37-850c-af461e1f31bc
GITHUB_REF=refs/pull/79285/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=431463215
GITHUB_RUN_NUMBER=21646
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-an5_bj6z/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpvx1izep2pip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.18
    Checking panic_abort v0.0.0 (/checkout/library/panic_abort)
error[E0599]: no function or associated item named `incr_strong_count` found for struct `Arc<_>` in the current scope
   --> library/alloc/src/task.rs:63:23
    |
63  |         unsafe { Arc::incr_strong_count(waker as *const W) };
    |                       |
    |                       |
    |                       function or associated item not found in `Arc<_>`
    |                       help: there is an associated function with a similar name: `increment_strong_count`
   ::: library/alloc/src/sync.rs:223:1
    |
    |
223 | pub struct Arc<T: ?Sized> {
    | ------------------------- function or associated item `incr_strong_count` not found for this

error[E0599]: no function or associated item named `decr_strong_count` found for struct `Arc<_>` in the current scope
   --> library/alloc/src/task.rs:84:23
    |
84  |         unsafe { Arc::decr_strong_count(waker as *const W) };
    |                       |
    |                       |
    |                       function or associated item not found in `Arc<_>`
    |                       help: there is an associated function with a similar name: `decrement_strong_count`
   ::: library/alloc/src/sync.rs:223:1
    |
    |
223 | pub struct Arc<T: ?Sized> {
    | ------------------------- function or associated item `decr_strong_count` not found for this
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `alloc`
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:30
== clock drift check ==
  local time: Fri Dec 18 22:32:07 UTC 2020
