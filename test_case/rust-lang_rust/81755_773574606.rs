plain
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: make ci-subset-1
  WIX: /d/a/rust/rust/wix
##[endgroup]
cp: '/c/hostedtoolcache/windows/Python/3.9.1/x64/python.exe' and '/c/hostedtoolcache/windows/Python/3.9.1/x64/python3.exe' are the same file
##[error]Process completed with exit code 1.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.30.0.windows.2
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
