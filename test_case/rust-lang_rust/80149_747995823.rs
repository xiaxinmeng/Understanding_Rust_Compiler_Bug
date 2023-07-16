plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=newpavlov
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_9f4b5940-b122-4a49-b345-5ebfbafb558d
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=getrandom2
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_9f4b5940-b122-4a49-b345-5ebfbafb558d
GITHUB_REF=refs/pull/80149/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=430180498
GITHUB_RUN_NUMBER=21601
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-ygglwwrj/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpz_znj3wepip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
extracting /checkout/obj/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
error: failed to parse manifest at `/checkout/library/std/Cargo.toml`

Caused by:
  failed to parse `not(any(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown")), all(target_arch = "aarch64", target_os = "hermit"))` as a cfg expression: expected `)`, found `,`
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
== clock drift check ==
  local time: Fri Dec 18 09:58:28 UTC 2020
  network time: Fri, 18 Dec 2020 00:13:25 GMT
== end clock drift check ==
