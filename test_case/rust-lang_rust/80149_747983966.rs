plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=newpavlov
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_fb5b0e38-416f-404e-a74d-df13867285dd
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=getrandom2
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_fb5b0e38-416f-404e-a74d-df13867285dd
GITHUB_REF=refs/pull/80149/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=430150006
GITHUB_RUN_NUMBER=21600
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-v8mrwflc/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmp73x6athbpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
    Checking object v0.22.0
    Checking hashbrown v0.9.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `]`, found `,`
     |
     |
2785 |         thread_local!(static KEYS: Cell<[u64, u64]> = {
     |                                             ^ expected one of 7 possible tokens
    ::: library/std/src/thread/local.rs:137:53
     |
     |
137  |     ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => (
     |                                                     ----- while parsing argument for this `ty` macro fragment

error[E0425]: cannot find value `KEYS` in this scope
     |
     |
2812 |         KEYS.with(|keys| {

error: unused import: `crate::cell::Cell`
 --> library/std/src/collections/hash/map.rs:9:5
  |
  |
9 | use crate::cell::Cell;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0425`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:41
== clock drift check ==
  local time: Fri Dec 18 09:42:19 UTC 2020
