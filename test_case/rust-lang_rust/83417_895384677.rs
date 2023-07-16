plain
##[error]fatal: unable to access 'https://github.com/rust-lang-ci/rust/': Failed to connect to github.com port 443: Timed out
The process 'C:\Program Files\Git\bin\git.exe' failed with exit code 128
Waiting 12 seconds before trying again
[command]"C:\Program Files\Git\bin\git.exe" -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +5b243cdd288c0c43b89060c71cfc549ee02c04fb:refs/remotes/origin/auto
##[error]fatal: unable to access 'https://github.com/rust-lang-ci/rust/': Failed to connect to github.com port 443: Timed out
##[error]The process 'C:\Program Files\Git\bin\git.exe' failed with exit code 128
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.32.0.windows.2
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
