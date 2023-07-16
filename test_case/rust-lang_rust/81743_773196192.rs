plain
 69  516M   69  356M    0     0  23.6M      0  0:00:21  0:00:15  0:00:06 23.0M
 73  516M   73  381M    0     0  23.7M      0  0:00:21  0:00:16  0:00:05 23.5M
 78  516M   78  405M    0     0  23.8M      0  0:00:21  0:00:17  0:00:04 24.3M
 81  516M   81  422M    0     0  23.8M      0  0:00:21  0:00:17  0:00:04 24.2M
curl: (56) OpenSSL SSL_read: Connection was reset, errno 10054

gzip: stdin: unexpected end of file
tar: Unexpected EOF in archive
tar: Unexpected EOF in archive
tar: Error is not recoverable: exiting now
##[error]Process completed with exit code 2.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.30.0.windows.2
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
