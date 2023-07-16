plain
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :"
[command]"C:\Program Files\Git\bin\git.exe" config --local http.https://github.com/.extraheader "AUTHORIZATION: basic ***"
##[endgroup]
##[group]Fetching the repository
[command]"C:\Program Files\Git\bin\git.exe" -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +dadec646cd5075aed7357933089ba1d2ab0a1545:refs/remotes/origin/auto
