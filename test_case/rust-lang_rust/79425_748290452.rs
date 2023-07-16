plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=sivadeilra
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_db32f4a9-3ac5-481a-903f-363029a71a31
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=user/ardavis/better_syms
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_db32f4a9-3ac5-481a-903f-363029a71a31
GITHUB_REF=refs/pull/79425/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=431216777
GITHUB_RUN_NUMBER=21635
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-54a0o6lr/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpkb0llc2upip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Found 434 error codes
Found 0 error codes with no tests
Done!
* 312 features
tidy error: Dependencies not explicitly permitted:
* phf 0.8.0 (registry+https://github.com/rust-lang/crates.io-index)
* phf_codegen 0.8.0 (registry+https://github.com/rust-lang/crates.io-index)
* phf_generator 0.8.0 (registry+https://github.com/rust-lang/crates.io-index)
* phf_shared 0.8.0 (registry+https://github.com/rust-lang/crates.io-index)
* siphasher 0.3.3 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

