plain
##[command]git remote add origin https://github.com/rust-lang-ci/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang-ci/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/*
##[error]fatal: could not read Username for 'https://github.com': terminal prompts disabled
##[warning]Git fetch failed with exit code 128, back off 2.331 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/*
##[error]fatal: could not read Username for 'https://github.com': terminal prompts disabled
##[error]fatal: the remote end hung up unexpectedly
##[warning]Git fetch failed with exit code 128, back off 9.286 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/*
##[error]fatal: could not read Username for 'https://github.com': terminal prompts disabled
##[error]Git fetch failed with exit code: 128
##[error]Exit code 1 returned from process: file name '/home/runner/runners/2.273.5/bin/Runner.PluginHost', arguments 'action "GitHub.Runner.Plugins.Repository.v1_0.CheckoutTask, Runner.Plugins"'.
