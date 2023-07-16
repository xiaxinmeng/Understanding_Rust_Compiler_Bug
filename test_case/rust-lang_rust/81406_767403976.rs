plain
  2  516M    2 15.3M    0     0  25.6M      0  0:00:20 --:--:--  0:00:20 25.6M
  8  516M    8 41.8M    0     0  26.5M      0  0:00:19  0:00:01  0:00:18 26.5M
 12  516M   12 67.1M    0     0  26.2M      0  0:00:19  0:00:02  0:00:17 26.2M
 13  516M   13 68.4M    0     0  26.3M      0  0:00:19  0:00:02  0:00:17 26.3M
curl: (56) OpenSSL SSL_read: Connection was reset, errno 10054

gzip: stdin: unexpected end of file
tar: Unexpected EOF in archive
tar: Unexpected EOF in archive
tar: Error is not recoverable: exiting now
##[error]Process completed with exit code 2.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.29.2.windows.3
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
http.https://github.com/.extraheader
[command]"C:\Program Files\Git\bin\git.exe" config --local --unset-all http.https://github.com/.extraheader
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :"
Cleaning up orphan processes
Terminate orphan process: pid (3004) (node)
Terminate orphan process: pid (4472) (python)
Terminate orphan process: pid (5224) (vctip)
