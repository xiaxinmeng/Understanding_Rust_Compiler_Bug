plain
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-gnu
  SCRIPT: make ci-mingw-subset-2
  WIX: /d/a/rust/rust/wix
##[endgroup]
cp: '/c/hostedtoolcache/windows/Python/3.9.1/x64/python.exe' and '/c/hostedtoolcache/windows/Python/3.9.1/x64/python3.exe' are the same file
##[error]Process completed with exit code 1.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.30.0.windows.2
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
http.https://github.com/.extraheader
[command]"C:\Program Files\Git\bin\git.exe" config --local --unset-all http.https://github.com/.extraheader
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :"
Cleaning up orphan processes
Terminate orphan process: pid (6464) (node)
Terminate orphan process: pid (7816) (python)
Terminate orphan process: pid (1768) (vctip)
