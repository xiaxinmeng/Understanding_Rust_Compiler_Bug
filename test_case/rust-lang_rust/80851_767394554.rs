plain
 28  516M   28  147M    0     0  31.5M      0  0:00:16  0:00:04  0:00:12 31.5M
 34  516M   34  178M    0     0  31.4M      0  0:00:16  0:00:05  0:00:11 31.5M
 40  516M   40  209M    0     0  31.4M      0  0:00:16  0:00:06  0:00:10 31.6M
 43  516M   43  226M    0     0  31.0M      0  0:00:16  0:00:07  0:00:09 31.3M
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
Terminate orphan process: pid (2496) (python)
Terminate orphan process: pid (5864) (vctip)
