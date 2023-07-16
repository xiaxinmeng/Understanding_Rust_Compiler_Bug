plain

[00:01:51] travis_time:end:0b44838b:start=1533900155365736761,finish=1533900197292481266,duration=41926744505
[CI_JOB_NAME=dist-mips-linux]
[00:01:51] [CI_JOB_NAME=dist-mips-linux]
[00:08:41] DiG 9.10.3-P4-Ubuntu
[00:08:41] travis_fold:start:check_dns_9696
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[00:08:41] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
[00:08:41] search c.travis-ci-prod-4.internal google.internal
[00:08:41] nameserver 8.8.4.4
[00:08:41] nameserver 1.1.1.1
[00:08:41] nameserver 1.0.0.1
[00:08:41] travis_fold:end:resolv-conf
[00:08:41] travis_fold:end:resolv-conf
travis_fold:start:nsswitch
cat /etc/nsswitch.conf ------------------------------------------
[00:08:41] # /etc/nsswitch.conf
[00:08:41] #
[00:08:41] # Example configuration of GNU Name Service Switch functionality.
[00:08:41] # If you have the `glibc-doc-reference' and `info' packages installed, try:
[00:08:41] # `info libc "Name Service Switch"' for information about this file.
[00:08:41] passwd:         compat
[00:08:41] group:          compat
[00:08:41] shadow:         compat
[00:08:41] gshadow:        files
---
[00:08:41] services:       db files
[00:08:41] ethers:         db files
[00:08:41] rpc:            db files
[00:08:41] 
[00:08:41] netgroup:       nis
[00:08:41] travis_fold:end:nsswitch
travis_fold:start:systemd-status
systemd-resolve --status ----------------------------------------
[00:08:41] travis_fold:end:systemd-status
travis_fold:start:network-interfaces
cat /etc/network/interfaces -------------------------------------
[00:08:41] systemd-resolve: unrecognized option '--status'
[00:08:41] cat: /etc/network/interfaces: No such file or directory
[00:08:41] travis_fold:end:network-interfaces
travis_fold:start:dig
[00:08:41] 192.30.253.112
[00:08:41] 192.30.253.113
[00:08:41] ---> Fri Aug 10 11:30:07 UTC 2018: try 0 github.com via 8.8.8.8 (IPv6)
[00:08:41] 192.30.253.112
[00:08:41] 192.30.253.112
[00:08:41] 192.30.253.113
[00:08:41] ---> Fri Aug 10 11:30:08 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:08:41] 52.219.24.9
[00:08:41] ---> Fri Aug 10 11:30:08 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:08:41] 54.231.237.37
[00:08:41] ---> Fri Aug 10 11:30:08 UTC 2018: try 0 github.com via 8.8.4.4 (IPv4)
[00:08:56] ;; connection timed out; no servers could be reached
[00:08:56] 192.30.253.113
[00:08:56] 192.30.253.112
[00:08:56] ---> Fri Aug 10 11:30:23 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:08:56] 52.219.28.41
[00:08:56] 52.219.28.41
[00:08:56] ---> Fri Aug 10 11:30:23 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:08:56] 52.219.20.17
[00:08:56] ---> Fri Aug 10 11:30:23 UTC 2018: try 0 github.com via 1.1.1.1 (IPv4)
[00:09:11] ;; connection timed out; no servers could be reached
[00:09:11] 192.30.253.112
[00:09:11] 192.30.253.113
[00:09:11] ---> Fri Aug 10 11:30:38 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:09:11] 52.219.24.41
---
[00:09:12] ---> Fri Aug 10 11:30:38 UTC 2018: try 1 github.com via 8.8.8.8 (IPv4)
[00:09:12] 192.30.253.112
[00:09:12] 192.30.253.113
[00:09:12] ---> Fri Aug 10 11:30:38 UTC 2018: try 1 github.com via 8.8.8.8 (IPv6)
[00:09:27] ;; connection timed out; no servers could be reached
[00:09:27] ---> Fri Aug 10 11:30:53 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:09:42] ;; connection timed out; no servers could be reached
[00:09:42] 52.219.24.105
[00:09:42] ---> Fri Aug 10 11:31:08 UTC 2018: try 1 github.com via 8.8.4.4 (IPv4)
[00:09:42] ---> Fri Aug 10 11:31:08 UTC 2018: try 1 github.com via 8.8.4.4 (IPv4)
[00:09:57] ;; connection timed out; no servers could be reached
[00:09:57] ---> Fri Aug 10 11:31:23 UTC 2018: try 1 github.com via 8.8.4.4 (IPv6)
[00:10:12] ;; connection timed out; no servers could be reached
[00:10:12] 52.219.20.73
[00:10:12] ---> Fri Aug 10 11:31:38 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:10:12] ---> Fri Aug 10 11:31:38 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:10:27] ;; connection timed out; no servers could be reached
[00:10:27] 192.30.253.112
[00:10:27] 192.30.253.113
[00:10:27] ---> Fri Aug 10 11:31:53 UTC 2018: try 1 github.com via 1.1.1.1 (IPv6)
[00:10:27] ---> Fri Aug 10 11:31:53 UTC 2018: try 1 github.com via 1.1.1.1 (IPv6)
[00:10:42] ;; connection timed out; no servers could be reached
[00:10:42] ---> Fri Aug 10 11:32:08 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:10:57] ;; connection timed out; no servers could be reached
[00:10:57] 54.231.237.29
[00:10:57] ---> Fri Aug 10 11:32:23 UTC 2018: try 1 github.com via 1.0.0.1 (IPv4)
[00:10:57] 192.30.253.112
[00:10:57] 192.30.253.113
[00:10:57] 192.30.253.113
[00:10:57] ---> Fri Aug 10 11:32:23 UTC 2018: try 1 github.com via 1.0.0.1 (IPv6)
[00:10:57] 192.30.253.112
[00:10:57] 192.30.253.113
[00:10:57] ---> Fri Aug 10 11:32:23 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:10:57] 54.231.237.29
[00:10:57] ---> Fri Aug 10 11:32:23 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:11:12] ;; connection timed out; no servers could be reached
[00:11:12] 192.30.253.113
[00:11:12] 192.30.253.112
[00:11:12] ---> Fri Aug 10 11:32:38 UTC 2018: try 1 github.com via 169.254.169.254 (IPv6)
[00:11:12] 192.30.253.113
---
[00:11:13] 192.30.253.113
[00:11:13] ---> Fri Aug 10 11:32:38 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:11:13] 52.219.24.137
[00:11:13] ---> Fri Aug 10 11:32:38 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:11:28] ;; connection timed out; no servers could be reached
[00:11:28] ---> Fri Aug 10 11:32:53 UTC 2018: try 2 github.com via 8.8.4.4 (IPv4)
[00:11:43] ;; connection timed out; no servers could be reached
[00:11:43] 192.30.253.113
[00:11:43] 192.30.253.112
[00:11:43] ---> Fri Aug 10 11:33:08 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:11:43] ---> Fri Aug 10 11:33:08 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:11:58] ;; connection timed out; no servers could be reached
[00:11:58] 52.219.20.9
[00:11:58] ---> Fri Aug 10 11:33:24 UTC 2018: try 2 github.com via 1.1.1.1 (IPv4)
[00:11:58] ---> Fri Aug 10 11:33:24 UTC 2018: try 2 github.com via 1.1.1.1 (IPv4)
[00:12:13] ;; connection timed out; no servers could be reached
[00:12:13] 192.30.253.112
[00:12:13] 192.30.253.113
[00:12:13] ---> Fri Aug 10 11:33:39 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:12:13] 52.219.28.81
---
[00:12:13] ---> Fri Aug 10 11:33:39 UTC 2018: try 2 github.com via 1.0.0.1 (IPv6)
[00:12:13] 192.30.253.112
[00:12:13] 192.30.253.113
[00:12:13] ---> Fri Aug 10 11:33:39 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:12:28] ;; connection timed out; no servers could be reached
[00:12:28] 52.219.20.1
[00:12:28] ---> Fri Aug 10 11:33:54 UTC 2018: try 2 github.com via 169.254.169.254 (IPv4)
[00:12:28] 192.30.253.112
[00:12:28] 192.30.253.113
---
[00:12:28] 52.219.28.137
[00:12:28] ---> Fri Aug 10 11:33:54 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:12:28] 52.219.28.137
[00:12:28] ---> Fri Aug 10 11:33:54 UTC 2018: try 3 github.com via 8.8.8.8 (IPv4)
[00:12:43] ;; connection timed out; no servers could be reached
[00:12:43] ---> Fri Aug 10 11:34:09 UTC 2018: try 3 github.com via 8.8.8.8 (IPv6)
[00:12:58] ;; connection timed out; no servers could be reached
[00:12:58] 52.219.28.5
[00:12:58] ---> Fri Aug 10 11:34:24 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:12:58] ---> Fri Aug 10 11:34:24 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:13:13] ;; connection timed out; no servers could be reached
[00:13:13] ---> Fri Aug 10 11:34:39 UTC 2018: try 3 github.com via 8.8.4.4 (IPv4)
[00:13:28] ;; connection timed out; no servers could be reached
[00:13:28] 192.30.253.112
[00:13:28] 192.30.253.113
[00:13:28] ---> Fri Aug 10 11:34:54 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:13:28] 52.219.28.5
[00:13:28] 52.219.28.5
[00:13:28] ---> Fri Aug 10 11:34:54 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:13:28] 52.219.20.29
[00:13:28] ---> Fri Aug 10 11:34:54 UTC 2018: try 3 github.com via 1.1.1.1 (IPv4)
[00:13:43] ;; connection timed out; no servers could be reached
[00:13:43] 192.30.253.112
[00:13:43] 192.30.253.113
[00:13:43] ---> Fri Aug 10 11:35:09 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:13:43] 52.219.20.65
[00:13:43] 52.219.20.65
[00:13:43] ---> Fri Aug 10 11:35:09 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:13:43] 52.219.20.65
[00:13:43] ---> Fri Aug 10 11:35:09 UTC 2018: try 3 github.com via 1.0.0.1 (IPv4)
[00:13:43] 192.30.253.112
[00:13:43] 192.30.253.113
[00:13:43] ---> Fri Aug 10 11:35:09 UTC 2018: try 3 github.com via 1.0.0.1 (IPv6)
[00:13:58] ;; connection timed out; no servers could be reached
[00:13:58] ---> Fri Aug 10 11:35:24 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:14:13] ;; connection timed out; no servers could be reached
[00:14:13] ---> Fri Aug 10 11:35:39 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:14:28] ;; connection timed out; no servers could be reached
[00:14:28] 192.30.253.112
[00:14:28] 192.30.253.113
[00:14:28] ---> Fri Aug 10 11:35:54 UTC 2018: try 3 github.com via 169.254.169.254 (IPv6)
[00:14:28] 192.30.253.112
---
[01:51:15] Dist clippy stage2 (mips-unknown-linux-gnu)
[01:51:15] Unable to build clippy, skipping dist
[01:54:47] [TIMING] Extended { stage: 2, host: "x86_64-unknown-linux-gnu", target: "mips-unknown-linux-gnu" } -- 211.775
[01:54:47] Build completed successfully in 1:36:22
[01:54:47] travis_fold:start:check_dns_9696
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[01:54:47] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
[01:54:47] search c.travis-ci-prod-4.internal google.internal
[01:54:47] nameserver 8.8.4.4
[01:54:47] nameserver 1.1.1.1
[01:54:47] nameserver 1.0.0.1
[01:54:47] travis_fold:end:resolv-conf
[01:54:47] travis_fold:end:resolv-conf
travis_fold:start:nsswitch
cat /etc/nsswitch.conf ------------------------------------------
[01:54:47] # /etc/nsswitch.conf
[01:54:47] #
[01:54:47] # Example configuration of GNU Name Service Switch functionality.
[01:54:47] # If you have the `glibc-doc-reference' and `info' packages installed, try:
[01:54:47] # `info libc "Name Service Switch"' for information about this file.
[01:54:47] passwd:         compat
[01:54:47] group:          compat
[01:54:47] shadow:         compat
[01:54:47] gshadow:        files
---
[01:54:47] services:       db files
[01:54:47] ethers:         db files
[01:54:47] rpc:            db files
[01:54:47] 
[01:54:47] netgroup:       nis
[01:54:47] travis_fold:end:nsswitch
travis_fold:start:systemd-status
systemd-resolve --status ----------------------------------------
[01:54:47] systemd-resolve: unrecognized option '--status'
[01:54:47] travis_fold:end:systemd-status
travis_fold:start:network-interfaces
cat /etc/network/interfaces -------------------------------------
[01:54:47] cat: /etc/network/interfaces: No such file or directory
[01:54:47] travis_fold:end:network-interfaces
travis_fold:start:dig
[01:54:47] 192.30.253.112
[01:54:47] 192.30.253.113
[01:54:47] ---> Fri Aug 10 13:16:13 UTC 2018: try 0 github.com via 8.8.8.8 (IPv6)
[01:54:47] 192.30.253.113
---
[01:54:47] 52.219.20.29
[01:54:47] ---> Fri Aug 10 13:16:13 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[01:54:47] 52.219.28.29
[01:54:47] ---> Fri Aug 10 13:16:13 UTC 2018: try 0 github.com via 1.1.1.1 (IPv4)
[01:55:02] ;; connection timed out; no servers could be reached
[01:55:02] ---> Fri Aug 10 13:16:28 UTC 2018: try 0 github.com via 1.1.1.1 (IPv6)
[01:55:17] ;; connection timed out; no servers could be reached
[01:55:17] ---> Fri Aug 10 13:16:43 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[01:55:32] ;; connection timed out; no servers could be reached
[01:55:32] ---> Fri Aug 10 13:16:58 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[01:55:47] ;; connection timed out; no servers could be reached
[01:55:47] ---> Fri Aug 10 13:17:13 UTC 2018: try 0 github.com via 1.0.0.1 (IPv4)
[01:56:02] ;; connection timed out; no servers could be reached
[01:56:02] 192.30.253.112
[01:56:02] 192.30.253.113
[01:56:02] ---> Fri Aug 10 13:17:28 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[01:56:02] 52.219.24.5
---
[01:56:02] 192.30.253.112
[01:56:02] ---> Fri Aug 10 13:17:29 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[01:56:02] 52.219.20.9
[01:56:02] ---> Fri Aug 10 13:17:29 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[01:56:17] ;; connection timed out; no servers could be reached
[01:56:17] ---> Fri Aug 10 13:17:44 UTC 2018: try 1 github.com via 8.8.4.4 (IPv4)
[01:56:32] ;; connection timed out; no servers could be reached
[01:56:32] 192.30.253.112
[01:56:32] 192.30.253.113
[01:56:32] ---> Fri Aug 10 13:17:59 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[01:56:32] 52.219.28.25
[01:56:32] 52.219.28.25
[01:56:32] ---> Fri Aug 10 13:17:59 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[01:56:33] 52.219.20.21
[01:56:33] ---> Fri Aug 10 13:17:59 UTC 2018: try 1 github.com via 1.1.1.1 (IPv4)
[01:56:48] ;; connection timed out; no servers could be reached
[01:56:48] 192.30.253.112
[01:56:48] 192.30.253.113
[01:56:48] ---> Fri Aug 10 13:18:14 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[01:56:48] 52.219.20.25
[01:56:48] 52.219.20.25
[01:56:48] ---> Fri Aug 10 13:18:14 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[01:57:03] ;; connection timed out; no servers could be reached
[01:57:03] 192.30.253.112
[01:57:03] 192.30.253.113
[01:57:03] ---> Fri Aug 10 13:18:29 UTC 2018: try 1 github.com via 1.0.0.1 (IPv6)
[01:57:03] 192.30.253.112
[01:57:03] 192.30.253.112
[01:57:03] 192.30.253.113
[01:57:03] ---> Fri Aug 10 13:18:29 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[01:57:18] ;; connection timed out; no servers could be reached
[01:57:18] 54.231.236.37
[01:57:18] ---> Fri Aug 10 13:18:44 UTC 2018: try 1 github.com via 169.254.169.254 (IPv4)
[01:57:18] 192.30.253.112
[01:57:18] 192.30.253.113
---
[01:57:18] 52.219.28.49
[01:57:18] ---> Fri Aug 10 13:18:44 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[01:57:18] 52.219.28.49
[01:57:18] ---> Fri Aug 10 13:18:44 UTC 2018: try 2 github.com via 8.8.8.8 (IPv4)
[01:57:33] ;; connection timed out; no servers could be reached
[01:57:33] 192.30.253.113
[01:57:33] 192.30.253.112
[01:57:33] ---> Fri Aug 10 13:18:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[01:57:33] 54.231.235.29
[01:57:33] 54.231.235.29
[01:57:33] ---> Fri Aug 10 13:18:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[01:57:33] 54.231.237.25
[01:57:33] ---> Fri Aug 10 13:18:59 UTC 2018: try 2 github.com via 8.8.4.4 (IPv4)
[01:57:48] ;; connection timed out; no servers could be reached
[01:57:48] 192.30.253.113
[01:57:48] 192.30.253.112
[01:57:48] ---> Fri Aug 10 13:19:14 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[01:57:48] ---> Fri Aug 10 13:19:14 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[01:58:03] ;; connection timed out; no servers could be reached
[01:58:03] 52.219.28.25
[01:58:03] ---> Fri Aug 10 13:19:29 UTC 2018: try 2 github.com via 1.1.1.1 (IPv4)
[01:58:03] ---> Fri Aug 10 13:19:29 UTC 2018: try 2 github.com via 1.1.1.1 (IPv4)
[01:58:18] ;; connection timed out; no servers could be reached
[01:58:18] 192.30.253.112
[01:58:18] 192.30.253.113
[01:58:18] ---> Fri Aug 10 13:19:44 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[01:58:18] 52.219.20.121
[01:58:18] 52.219.20.121
[01:58:18] ---> Fri Aug 10 13:19:44 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[01:58:18] 52.219.20.121
[01:58:18] ---> Fri Aug 10 13:19:44 UTC 2018: try 2 github.com via 1.0.0.1 (IPv4)
[01:58:18] 192.30.253.112
[01:58:18] 192.30.253.113
[01:58:18] ---> Fri Aug 10 13:19:44 UTC 2018: try 2 github.com via 1.0.0.1 (IPv6)
[01:58:33] ;; connection timed out; no servers could be reached
[01:58:33] 54.231.235.45
[01:58:33] ---> Fri Aug 10 13:19:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[01:58:33] ---> Fri Aug 10 13:19:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[01:58:48] ;; connection timed out; no servers could be reached
[01:58:48] 192.30.253.113
[01:58:48] 192.30.253.112
[01:58:48] ---> Fri Aug 10 13:20:14 UTC 2018: try 2 github.com via 169.254.169.254 (IPv6)
[01:58:48] 192.30.253.113
---
[01:58:48] ---> Fri Aug 10 13:20:15 UTC 2018: try 3 github.com via 8.8.8.8 (IPv6)
[01:58:48] 192.30.253.112
[01:58:48] 192.30.253.113
[01:58:48] ---> Fri Aug 10 13:20:15 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[01:59:03] ;; connection timed out; no servers could be reached
[01:59:03] ---> Fri Aug 10 13:20:30 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[01:59:18] ;; connection timed out; no servers could be reached
[01:59:18] 192.30.253.112
[01:59:18] 192.30.253.113
[01:59:18] ---> Fri Aug 10 13:20:45 UTC 2018: try 3 github.com via 8.8.4.4 (IPv6)
[01:59:18] 192.30.253.112
[01:59:18] 192.30.253.112
[01:59:18] 192.30.253.113
[01:59:18] ---> Fri Aug 10 13:20:45 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[01:59:33] ;; connection timed out; no servers could be reached
[01:59:33] 52.219.28.121
[01:59:33] ---> Fri Aug 10 13:21:00 UTC 2018: try 3 github.com via 1.1.1.1 (IPv4)
[01:59:33] 192.30.253.112
[01:59:33] 192.30.253.113
[01:59:33] 192.30.253.113
[01:59:33] ---> Fri Aug 10 13:21:00 UTC 2018: try 3 github.com via 1.1.1.1 (IPv6)
[01:59:33] 192.30.253.112
[01:59:33] 192.30.253.113
[01:59:33] ---> Fri Aug 10 13:21:00 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[01:59:49] ;; connection timed out; no servers could be reached
[01:59:49] 52.219.24.17
[01:59:49] ---> Fri Aug 10 13:21:15 UTC 2018: try 3 github.com via 1.0.0.1 (IPv4)
[01:59:49] 192.30.253.112
[01:59:49] 192.30.253.113
[01:59:49] 192.30.253.113
[01:59:49] ---> Fri Aug 10 13:21:15 UTC 2018: try 3 github.com via 1.0.0.1 (IPv6)
[02:00:04] ;; connection timed out; no servers could be reached
[02:00:04] ---> Fri Aug 10 13:21:30 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[02:00:19] ;; connection timed out; no servers could be reached
[02:00:19] ---> Fri Aug 10 13:21:45 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[02:00:34] ;; connection timed out; no servers could be reached
[02:00:34] 192.30.253.113
[02:00:34] 192.30.253.112
[02:00:34] ---> Fri Aug 10 13:22:00 UTC 2018: try 3 github.com via 169.254.169.254 (IPv6)
[02:00:34] 192.30.253.113
---
Preparing deploy
travis_fold:end:dpl.2
travis_fold:start:dpl.3
Deploying application
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-analysis-nightly-mips-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rustc-nightly-mips-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-src-nightly.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/llvm-tools-nightly-mips-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-std-nightly-mips-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rustc-nightly-mips-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-analysis-nightly-mips-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-std-nightly-mips-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rustfmt-nightly-mips-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-src-nightly.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rustfmt-nightly-mips-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-nightly-mips-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/cargo-nightly-mips-unknown-linux-gnu.tar.gz" with {:content_type=>"application/gzip", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/rust-nightly-mips-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/llvm-tools-nightly-mips-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
uploading "4fd5a1d188b25ee6b66063dfee71847c015b609d/cargo-nightly-mips-unknown-linux-gnu.tar.xz" with {:content_type=>"application/x-xz", :acl=>"public-read"}
/home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:879:in `initialize': execution expired (Seahorse::Client::NetworkingError)
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:879:in `open'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:879:in `block in connect'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/timeout.rb:88:in `block in timeout'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/timeout.rb:98:in `call'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/timeout.rb:98:in `timeout'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:878:in `connect'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:863:in `do_start'
 from /home/travis/.rvm/rubies/ruby-2.2.7/lib/ruby/2.2.0/net/http.rb:858:in `start'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/net_http/connection_pool.rb:285:in `start_session'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/net_http/connection_pool.rb:92:in `session_for'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/net_http/handler.rb:119:in `session'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/net_http/handler.rb:71:in `transmit'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/net_http/handler.rb:45:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/plugins/content_length.rb:12:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_request_signer.rb:88:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_request_signer.rb:23:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_host_id.rb:14:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/xml/error_handler.rb:8:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/helpful_socket_errors.rb:10:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_request_signer.rb:65:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_redirects.rb:15:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:89:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:120:in `retry_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:103:in `retry_if_possible'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:91:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:120:in `retry_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:103:in `retry_if_possible'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:91:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:120:in `retry_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:103:in `retry_if_possible'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/retry_errors.rb:91:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_dualstack.rb:32:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_accelerate.rb:49:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_md5s.rb:31:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_expect_100_continue.rb:21:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_bucket_name_restrictions.rb:12:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_bucket_dns.rb:31:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/rest/handler.rb:7:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/user_agent.rb:12:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/plugins/endpoint.rb:41:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/param_validator.rb:21:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/plugins/raise_response_errors.rb:14:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_sse_cpk.rb:19:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_dualstack.rb:24:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/s3_accelerate.rb:34:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/jsonvalue_converter.rb:20:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/idempotency_token.rb:18:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/aws-sdk-core/plugins/param_converter.rb:20:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/plugins/response_target.rb:21:in `call'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/request.rb:70:in `send_request'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-core-2.11.105/lib/seahorse/client/base.rb:207:in `block (2 levels) in define_operation_methods'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.105/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:52:in `initiate_upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.105/lib/aws-sdk-resources/services/s3/multipart_file_uploader.rb:43:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.105/lib/aws-sdk-resources/services/s3/file_uploader.rb:32:in `upload'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/aws-sdk-resources-2.11.105/lib/aws-sdk-resources/services/s3/object.rb:252:in `upload_file'
 from /home/travis/.rvm/gems/ruby-2.2.7/gems/dpl-s3-1.9.8/lib/dpl/provider/s3.rb:99:in `block (2 levels) in upload_multithreaded'
failed to deploy
