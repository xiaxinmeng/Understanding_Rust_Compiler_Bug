plain
 74  424M   74  314M    0     0  7305k      0  0:00:59  0:00:44  0:00:15 7595k
 76  424M   76  324M    0     0  7371k      0  0:00:58  0:00:45  0:00:13 8361k
 78  424M   78  332M    0     0  7380k      0  0:00:58  0:00:46  0:00:12 8306k
 79  424M   79  338M    0     0  7388k      0  0:00:58  0:00:46  0:00:12 8456k
curl: (18) transfer closed with 89405789 bytes remaining to read
clang+llvm-10.0.0-x86_64-apple-darwin/bin/bugpoint: Lzma library error:  No progress is possible
tar: Error exit delayed from previous errors.
##[error]Process completed with exit code 1.
[command]/usr/local/bin/git version
git version 2.31.1
[command]/usr/local/bin/git config --local --name-only --get-regexp core\.sshCommand
[command]/usr/local/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :
