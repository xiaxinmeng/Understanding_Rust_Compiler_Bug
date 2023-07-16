plain
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_cd4d2b08-ea1c-4263-8f21-454e91eed318
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=printf-alias
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_cd4d2b08-ea1c-4263-8f21-454e91eed318
GITHUB_REF=refs/pull/79232/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=430859652
GITHUB_RUN_NUMBER=21619
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-34nmcu7q/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmp7id7lrbfpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
.................................................................................................... 9000/11178
.................................................................................................... 9100/11178
......................................................................i......i...................... 9200/11178
.................................................................................................... 9300/11178
.........iiiiii..iiiiii.i........................................................................... 9400/11178
.................................................................................................... 9600/11178
.................................................................................................... 9700/11178
.................................................................................................... 9800/11178
.................................................................................................... 9900/11178
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.066 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.387 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Checking "from_u" ... OK
Checking "keyword" ... OK
Checking "macro-check" ... OK
Checking "macro-print" ... FAILED
==> '{"path":"std","name":"dbg"}' was supposed to be before '{"crate":"std","name":"dbg","path":"std","desc":"Prints and returns the value of a given expression for …","ty":14,"type":null,"is_alias":true,"alias":"macro:print","displayPath":"<span>std::</span>","fullPath":"<span>std::</span>dbg","href":"../std/macro.dbg.html"}'
==> '{"path":"std","name":"eprint"}' was supposed to be before '{"crate":"std","name":"eprint","path":"std","desc":"Prints to the standard error.","ty":14,"type":null,"is_alias":true,"alias":"macro:print","displayPath":"<span>std::</span>","fullPath":"<span>std::</span>eprint","href":"../std/macro.eprint.html"}'
==> '{"path":"std","name":"println"}' was supposed to be before '{"crate":"std","name":"println","path":"std","desc":"Prints to the standard output, with a newline.","ty":14,"type":null,"is_alias":true,"alias":"macro:print","displayPath":"<span>std::</span>","fullPath":"<span>std::</span>println","href":"../std/macro.println.html"}'
==> '{"path":"std","name":"eprintln"}' was supposed to be before '{"crate":"std","name":"eprintln","path":"std","desc":"Prints to the standard error, with a newline.","ty":14,"type":null,"is_alias":true,"alias":"macro:print","displayPath":"<span>std::</span>","fullPath":"<span>std::</span>eprintln","href":"../std/macro.eprintln.html"}'
Checking "never" ... OK
Checking "quoted" ... OK
Checking "return-specific-literal" ... OK
Checking "return-specific" ... OK
Checking "return-specific" ... OK
Checking "should-fail" ... OK
Checking "string-from_ut" ... OK
Checking "struct-vec" ... OK
Checking "vec-new" ... OK


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.50.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:34:10
