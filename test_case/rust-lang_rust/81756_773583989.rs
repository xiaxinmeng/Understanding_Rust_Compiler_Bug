plain
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-gnu
  SCRIPT: make ci-mingw-subset-1
  WIX: /d/a/rust/rust/wix
##[endgroup]
cp: '/c/hostedtoolcache/windows/Python/3.9.1/x64/python.exe' and '/c/hostedtoolcache/windows/Python/3.9.1/x64/python3.exe' are the same file
##[error]Process completed with exit code 1.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.30.0.windows.2
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
