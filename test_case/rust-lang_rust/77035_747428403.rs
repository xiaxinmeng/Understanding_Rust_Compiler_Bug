plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=mibac138
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_a46817dc-8845-4e22-b4f9-982b9f1ebee7
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=fn-fat-arrow-return
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_a46817dc-8845-4e22-b4f9-982b9f1ebee7
GITHUB_REF=refs/pull/77035/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=428103634
GITHUB_RUN_NUMBER=21547
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-lnkhctf_/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmprqv7a7gcpip-wheel- --python-tag cp36:
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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/ty.rs at line 43:
     No,
 
-
-
 /// Signals whether parsing a type should recover `->`.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/ty.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 ///
 /// More specifically, when parsing a function like:
Build completed unsuccessfully in 0:00:18
== clock drift check ==
  local time: Thu Dec 17 13:07:24 UTC 2020
  network time: Thu, 17 Dec 2020 00:13:25 GMT
