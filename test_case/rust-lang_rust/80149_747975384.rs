plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=newpavlov
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_c556f861-0a85-4842-97e2-b1aa6155aaa3
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=getrandom2
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_c556f861-0a85-4842-97e2-b1aa6155aaa3
GITHUB_REF=refs/pull/80149/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=430137302
GITHUB_RUN_NUMBER=21599
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-g1owyljk/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmp0iw8t41dpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
   Compiling object v0.22.0
   Compiling hashbrown v0.9.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.14.0
error: expected one of `,`, `.`, `?`, `}`, or an operator, found `Err`
     |
     |
2805 |                 ))) => [2, 1]
     |                     --       - expected one of `,`, `.`, `?`, `}`, or an operator
     |                     |
     |                     while parsing the `match` arm starting here
2806 |                 Err(err) => panic!("failed to get system entropy:", err),


error: expected `;`, found `Cell`
     |
2807 |             }
2807 |             }
     |              ^ help: add `;` here
2808 |             Cell::new(seed)
     |             ---- unexpected token
error[E0308]: mismatched types
    --> library/std/src/collections/hash/map.rs:2808:23
     |
     |
2808 |             Cell::new(seed)
     |                       ^^^^ expected tuple, found array `[u64; 2]`
     |
     = note: expected tuple `(u64, u64)`
                found array `[u64; 2]`

error[E0529]: expected an array or slice, found `(u64, u64)`
     |
     |
2812 |             let [k0, k1] = keys.get();
     |                 ^^^^^^^^ pattern cannot match with input type `(u64, u64)`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0529.
For more information about an error, try `rustc --explain E0308`.
