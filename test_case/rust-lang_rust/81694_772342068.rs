plain
 40  516M   40  211M    0     0  28.2M      0  0:00:18  0:00:07  0:00:11 28.7M
 46  516M   46  238M    0     0  28.1M      0  0:00:18  0:00:08  0:00:10 28.6M
 51  516M   51  265M    0     0  28.0M      0  0:00:18  0:00:09  0:00:09 28.1M
 55  516M   55  284M    0     0  28.1M      0  0:00:18  0:00:10  0:00:08 28.5M
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
