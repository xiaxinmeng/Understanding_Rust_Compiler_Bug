plain
##[section]Starting: Request a runner to run this job
Can't find any online and idle self-hosted runner in current repository that matches the required labels: 'ubuntu-latest-xl'
Can't find any online and idle self-hosted runner in current repository's organization account that matches the required labels: 'ubuntu-latest-xl'
Found online and idle hosted runner in current repository's organization account that matches the required labels: 'ubuntu-latest-xl'
##[section]Finishing: Request a runner to run this job
##[debug]Starting: try (dist-x86_64-linux, ubuntu-latest-xl)
##[debug]Starting: Set up job
Current runner version: '2.273.6'
##[group]Operating System
Ubuntu
---
##[debug]Evaluating job defaults
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v2'
##[debug]Download 'https://api.github.com/repos/actions/checkout/tarball/5a4ac9002d0be2fb38bd78e4b4dbde5606d7042f' to '/home/runner/work/_actions/_temp_42d6ca9f-d06f-40ff-9dd6-059505422b0f/624c3343-e4c5-44a0-a6a1-58c9467342d5.tar.gz'
##[debug]Unwrap 'actions-checkout-5a4ac90' to '/home/runner/work/_actions/actions/checkout/v2'
##[debug]Archive '/home/runner/work/_actions/_temp_42d6ca9f-d06f-40ff-9dd6-059505422b0f/624c3343-e4c5-44a0-a6a1-58c9467342d5.tar.gz' has been unzipped into '/home/runner/work/_actions/actions/checkout/v2'.
Download action repository 'rust-lang/simpleinfra@master'
##[debug]Download 'https://api.github.com/repos/rust-lang/simpleinfra/tarball/825b781f5ce1c08493fb921b0d847d3bf6bc7d91' to '/home/runner/work/_actions/_temp_80d2c7b5-bdf7-45bf-9d11-bd19ff9efd3a/b068d976-be7c-406d-bc3b-c15508cd6386.tar.gz'
##[debug]Unwrap 'rust-lang-simpleinfra-825b781' to '/home/runner/work/_actions/rust-lang/simpleinfra/master'
##[debug]Archive '/home/runner/work/_actions/_temp_80d2c7b5-bdf7-45bf-9d11-bd19ff9efd3a/b068d976-be7c-406d-bc3b-c15508cd6386.tar.gz' has been unzipped into '/home/runner/work/_actions/rust-lang/simpleinfra/master'.
##[debug]action.yml for action: '/home/runner/work/_actions/rust-lang/simpleinfra/master/github-actions/cancel-outdated-builds/action.yml'.
##[debug]Set step 'run1' display name to: 'disable git crlf conversion'
##[debug]Set step 'actionscheckout' display name to: 'checkout the source code'
##[debug]Set step 'run2' display name to: 'configure the PR in which the error message will be posted'
---
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
##[endgroup]
##[debug]Overwrite 'shell' base on job defaults.
##[debug]/bin/bash --noprofile --norc -e -o pipefail /home/runner/work/_temp/5aab460d-53d7-4fc4-87cf-4dd0a6476258.sh
##[debug]Evaluating condition for step: 'checkout the source code'
##[debug]Evaluating: success()
##[debug]Evaluating success:
##[debug]=> true
---
::endgroup::
##[endgroup]
::group::Fetching the repository
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +e1ed1bb65d7f01dd2fb49e022313bc28e06d9560:refs/remotes/origin/try
---
   Compiling tempfile v3.1.0
   Compiling serde_json v1.0.59
   Compiling lint-docs v0.1.0 (/checkout/src/tools/lint-docs)
    Finished release [optimized] target(s) in 8.70s
error: could not determine lint name in /checkout/compiler/rustc_lint_defs/src/builtin.rs:2557: lint name should end with comma, line was `//FIXME: Add explanation.`


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/lint-docs" "--src" "/checkout/compiler" "--out" "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc/rustc/src/lints" "--rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustc-target" "x86_64-unknown-linux-gnu"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:22:42
Build completed unsuccessfully in 0:22:42
== clock drift check ==
  local time: Thu Nov  5 20:31:35 UTC 2020
  network time: Wed, 04 Nov 2020 23:05:40 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
##[debug]Evaluating: env.ARTIFACTS_AWS_ACCESS_KEY_ID
##[debug]Evaluating Index:
##[debug]..Evaluating env:
##[debug]..=> Object
---
##[debug]Evaluating And:
##[debug]..Evaluating success:
##[debug]..=> false
##[debug]=> false
##[debug]Expanded: (false && !env['SKIP_JOB'] && ((github['event_name'] == 'push') || (env['DEPLOY'] == '1') || (env['DEPLOY_ALT'] == '1')))
##[debug]Evaluating condition for step: 'Post checkout the source code'
##[debug]Evaluating: always()
##[debug]Evaluating always:
##[debug]=> true
---
Completed runner diagnostic log upload
Cleaning up orphan processes
Terminate orphan process: pid (6176) (python)
##[debug]Finishing: Complete job
##[debug]Finishing: try (dist-x86_64-linux, ubuntu-latest-xl)
