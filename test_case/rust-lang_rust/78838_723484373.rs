plain
##[debug]Evaluating job defaults
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v2'
##[debug]Download 'https://api.github.com/repos/actions/checkout/tarball/5a4ac9002d0be2fb38bd78e4b4dbde5606d7042f' to '/home/runner/work/_actions/_temp_5ba9f228-14e0-47c6-91e8-23c545d69291/c6f8c45f-9a50-4373-b727-46118571b2a8.tar.gz'
##[debug]Unwrap 'actions-checkout-5a4ac90' to '/home/runner/work/_actions/actions/checkout/v2'
##[debug]Archive '/home/runner/work/_actions/_temp_5ba9f228-14e0-47c6-91e8-23c545d69291/c6f8c45f-9a50-4373-b727-46118571b2a8.tar.gz' has been unzipped into '/home/runner/work/_actions/actions/checkout/v2'.
Download action repository 'rust-lang/simpleinfra@master'
##[debug]Download 'https://api.github.com/repos/rust-lang/simpleinfra/tarball/825b781f5ce1c08493fb921b0d847d3bf6bc7d91' to '/home/runner/work/_actions/_temp_41f93e60-8b91-42b4-9327-43a4dcd5eb94/42d5334c-168a-491f-b53d-36db23cf4dad.tar.gz'
##[debug]Unwrap 'rust-lang-simpleinfra-825b781' to '/home/runner/work/_actions/rust-lang/simpleinfra/master'
##[debug]Archive '/home/runner/work/_actions/_temp_41f93e60-8b91-42b4-9327-43a4dcd5eb94/42d5334c-168a-491f-b53d-36db23cf4dad.tar.gz' has been unzipped into '/home/runner/work/_actions/rust-lang/simpleinfra/master'.
##[debug]action.yml for action: '/home/runner/work/_actions/rust-lang/simpleinfra/master/github-actions/cancel-outdated-builds/action.yml'.
##[debug]Set step 'run1' display name to: 'disable git crlf conversion'
##[debug]Set step 'actionscheckout' display name to: 'checkout the source code'
##[debug]Set step 'run2' display name to: 'configure the PR in which the error message will be posted'
---
::endgroup::
##[endgroup]
::group::Fetching the repository
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +c5765371250b855f20075334e18485ae7e217e8b:refs/remotes/origin/auto
