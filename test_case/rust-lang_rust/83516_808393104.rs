plain
  0     0    0     0    0     0      0      0 --:--:--  0:01:21 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:22 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:23 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:24 --:--:--     0
curl: (28) Failed to connect to ci-mirrors.rust-lang.org port 443: Timed out
##[error]Process completed with exit code 28.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.30.2.windows.1
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
