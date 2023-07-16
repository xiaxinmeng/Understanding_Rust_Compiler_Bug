plain
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  3  516M    3 19.3M    0     0  24.6M      0  0:00:20 --:--:--  0:00:20 24.6M
  8  516M    8 43.3M    0     0  24.3M      0  0:00:21  0:00:01  0:00:20 24.3M
 12  516M   12 64.0M    0     0  24.3M      0  0:00:21  0:00:02  0:00:19 24.3M
curl: (56) OpenSSL SSL_read: Connection was reset, errno 10054

gzip: stdin: unexpected end of file
tar: Unexpected EOF in archive
tar: Unexpected EOF in archive
tar: Error is not recoverable: exiting now
##[error]Process completed with exit code 2.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.30.0.windows.1
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
http.https://github.com/.extraheader
[command]"C:\Program Files\Git\bin\git.exe" config --local --unset-all http.https://github.com/.extraheader
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :"
Cleaning up orphan processes
Terminate orphan process: pid (5124) (node)
Terminate orphan process: pid (4404) (python)
Terminate orphan process: pid (916) (vctip)
