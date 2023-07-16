plain
 58  516M   58  301M    0     0  24.3M      0  0:00:21  0:00:12  0:00:09 24.5M
 60  516M   60  313M    0     0  24.4M      0  0:00:21  0:00:12  0:00:09 24.6M
curl: (56) OpenSSL SSL_read: Connection was reset, errno 10054

gzip: stdin: unexpected end of file
tar: Unexpected EOF in archive
tar: Unexpected EOF in archive
tar: Error is not recoverable: exiting now
##[error]Process completed with exit code 2.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.31.1.windows.1
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
