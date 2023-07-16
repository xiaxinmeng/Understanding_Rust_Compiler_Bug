plain
[00:01:08] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 24 70080   24 17124    0     0  52017      0  0:00:01 --:--:--  0:00:01 51890
100 70080  100 70080    0     0   140k      0 --:--:-- --:--:-- --:--:--  140k
[00:01:08] DiG 9.10.3-P4-Ubuntu
[00:01:08] travis_fold:start:check_dns_9696
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[00:01:08] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
[00:01:08] search c.travis-ci-prod-4.internal google.internal
[00:01:08] nameserver 8.8.4.4
[00:01:08] nameserver 1.1.1.1
[00:01:08] nameserver 1.0.0.1
[00:01:08] nameserver 1.0.0.1
[00:01:08] travis_fold:end:resolv-conf
travis_fold:start:nsswitch
cat /etc/nsswitch.conf ------------------------------------------
[00:01:08] # /etc/nsswitch.conf
[00:01:08] #
[00:01:08] # Example configuration of GNU Name Service Switch functionality.
[00:01:08] # If you have the `glibc-doc-reference' and `info' packages installed, try:
[00:01:08] # `info libc "Name Service Switch"' for information about this file.
[00:01:08] passwd:         compat
[00:01:08] group:          compat
[00:01:08] shadow:         compat
[00:01:08] shadow:         compat
[00:01:08] gshadow:        files
[00:01:08] hosts:          files dns
[00:01:08] networks:       files
[00:01:08] 
[00:01:08] protocols:      db files
[00:01:08] protocols:      db files
[00:01:08] services:       db files
[00:01:08] ethers:         db files
[00:01:08] rpc:            db files
[00:01:08] 
[00:01:08] netgroup:       nis
[00:01:08] travis_fold:end:nsswitch
travis_fold:start:systemd-status
systemd-resolve --status ----------------------------------------
[00:01:08] systemd-resolve: unrecognized option '--status'
[00:01:08] travis_fold:end:systemd-status
travis_fold:start:network-interfaces
cat /etc/network/interfaces -------------------------------------
[00:01:08] cat: /etc/network/interfaces: No such file or directory
[00:01:08] travis_fold:end:network-interfaces
travis_fold:start:dig
---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 8.8.8.8 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 8.8.8.8 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:01:09] 52.219.28.41
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 8.8.4.4 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 8.8.4.4 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:01:09] 52.219.28.41
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 1.1.1.1 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 1.1.1.1 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 1.0.0.1 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 1.0.0.1 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 169.254.169.254 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 github.com via 169.254.169.254 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 github.com via 8.8.8.8 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 github.com via 8.8.8.8 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:01:09] 52.219.20.1
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 github.com via 8.8.4.4 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 github.com via 8.8.4.4 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:01:09] 52.219.20.9
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 github.com via 1.1.1.1 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 github.com via 1.1.1.1 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:02 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 github.com via 1.0.0.1 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 github.com via 1.0.0.1 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 github.com via 169.254.169.254 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 github.com via 169.254.169.254 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 8.8.8.8 (IPv4)
[00:01:09] 192.30.253.112
[00:01:09] 192.30.253.112
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 8.8.8.8 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:01:09] 52.219.20.29
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:01:09] 52.219.20.1
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 8.8.4.4 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 8.8.4.4 (IPv6)
[00:01:09] 192.30.253.112
[00:01:09] 192.30.253.112
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:01:09] 52.219.24.37
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:01:09] 52.219.20.41
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 1.1.1.1 (IPv4)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 1.1.1.1 (IPv6)
[00:01:09] 192.30.253.113
[00:01:09] 192.30.253.113
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:01:09] 52.219.28.13
[00:01:09] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 1.0.0.1 (IPv4)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 1.0.0.1 (IPv6)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:01:10] 52.219.28.13
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:01:10] 52.219.28.13
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 169.254.169.254 (IPv4)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 github.com via 169.254.169.254 (IPv6)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:01:10] 52.219.24.37
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:01:10] 52.219.24.37
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 8.8.8.8 (IPv4)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 8.8.8.8 (IPv6)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:01:10] 52.219.20.97
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:01:10] 52.219.28.41
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 8.8.4.4 (IPv4)
[00:01:10] 192.30.253.112
[00:01:10] 192.30.253.112
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 8.8.4.4 (IPv6)
[00:01:10] 192.30.253.112
[00:01:10] 192.30.253.112
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:01:10] 52.219.20.1
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:01:10] 52.219.24.37
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 1.1.1.1 (IPv4)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 1.1.1.1 (IPv6)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:01:10] 52.219.28.13
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:01:10] 52.219.28.13
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 1.0.0.1 (IPv4)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 1.0.0.1 (IPv6)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:01:10] 52.219.28.13
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:01:10] 52.219.28.13
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 169.254.169.254 (IPv4)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 github.com via 169.254.169.254 (IPv6)
[00:01:10] 192.30.253.113
[00:01:10] 192.30.253.113
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:01:10] 52.219.24.37
[00:01:10] ---> Thu Aug  9 20:21:03 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:01:10] travis_fold:end:dig
travis_fold:end:check_dns_9696
Starting sccache server...
[00:01:10] travis_fold:start:configure
---

[00:04:10] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:10] tidy error: /checkout/src/ci/check-dns-bits.sh:14: line longer than 100 chars
[00:04:10] tidy error: /checkout/src/ci/check-dns-bits.sh:18: line longer than 100 chars
[00:04:10] tidy error: /checkout/src/ci/check-dns-bits.sh:19: line longer than 100 chars
[00:04:10] tidy error: /checkout/src/ci/check-dns-bits.sh:20: line longer than 100 chars
[00:04:10] tidy error: /checkout/src/ci/check-dns-bits.sh:21: line longer than 100 chars
[00:04:10] tidy error: /checkout/src/ci/check-dns-bits.sh:22: line longer than 100 chars
[00:04:10] tidy error: /checkout/src/ci/check-dns-bits.sh:23: line longer than 100 chars
[00:04:12] some tidy checks failed
[00:04:12] 
[00:04:12] 
[00:04:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:12] 
[00:04:12] 
[00:04:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:12] Build completed unsuccessfully in 0:00:52
[00:04:12] Build completed unsuccessfully in 0:00:52
[00:04:12] Makefile:79: recipe for target 'tidy' failed
[00:04:12] make: *** [tidy] Error 1
[00:04:12] travis_fold:start:check_dns_9696
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[00:04:12] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
[00:04:12] search c.travis-ci-prod-4.internal google.internal
[00:04:12] nameserver 8.8.4.4
[00:04:12] nameserver 1.1.1.1
[00:04:12] nameserver 1.0.0.1
[00:04:12] nameserver 1.0.0.1
[00:04:12] travis_fold:end:resolv-conf
travis_fold:start:nsswitch
cat /etc/nsswitch.conf ------------------------------------------
[00:04:12] # /etc/nsswitch.conf
[00:04:12] #
[00:04:12] # Example configuration of GNU Name Service Switch functionality.
[00:04:12] # If you have the `glibc-doc-reference' and `info' packages installed, try:
[00:04:12] # `info libc "Name Service Switch"' for information about this file.
[00:04:12] passwd:         compat
[00:04:12] group:          compat
[00:04:12] shadow:         compat
[00:04:12] shadow:         compat
[00:04:12] gshadow:        files
[00:04:12] hosts:          files dns
[00:04:12] networks:       files
[00:04:12] 
[00:04:12] protocols:      db files
[00:04:12] protocols:      db files
[00:04:12] services:       db files
[00:04:12] ethers:         db files
[00:04:12] rpc:            db files
[00:04:12] 
[00:04:12] netgroup:       nis
[00:04:12] travis_fold:end:nsswitch
travis_fold:start:systemd-status
systemd-resolve --status ----------------------------------------
[00:04:12] systemd-resolve: unrecognized option '--status'
[00:04:12] travis_fold:end:systemd-status
travis_fold:start:network-interfaces
cat /etc/network/interfaces -------------------------------------
[00:04:12] cat: /etc/network/interfaces: No such file or directory
[00:04:12] travis_fold:end:network-interfaces
travis_fold:start:dig
---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 8.8.8.8 (IPv4)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 8.8.8.8 (IPv6)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:04:12] 52.219.20.65
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:04:12] 52.219.24.105
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 8.8.4.4 (IPv4)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 8.8.4.4 (IPv6)
[00:04:12] 192.30.253.112
[00:04:12] 192.30.253.112
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:04:12] 52.219.28.105
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:04:12] 52.219.24.105
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 1.1.1.1 (IPv4)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 1.1.1.1 (IPv6)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:04:12] 54.231.237.25
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:04:12] 54.231.237.25
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 1.0.0.1 (IPv4)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:05 UTC 2018: try 0 github.com via 1.0.0.1 (IPv6)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:04:12] 54.231.237.25
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:04:12] 54.231.237.25
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 0 github.com via 169.254.169.254 (IPv4)
[00:04:12] 192.30.255.112
[00:04:12] 192.30.255.112
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 0 github.com via 169.254.169.254 (IPv6)
[00:04:12] 192.30.255.112
[00:04:12] 192.30.255.112
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:04:12] 52.219.24.105
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:04:12] 52.219.24.105
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 8.8.8.8 (IPv4)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 8.8.8.8 (IPv6)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:04:12] 52.219.20.97
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:04:12] 52.219.20.65
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 8.8.4.4 (IPv4)
[00:04:12] 192.30.253.113
[00:04:12] 192.30.253.113
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 8.8.4.4 (IPv6)
[00:04:12] 192.30.253.112
[00:04:12] 192.30.253.112
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:04:12] 52.219.20.105
[00:04:12] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:04:13] 52.219.24.129
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 1.1.1.1 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 1.1.1.1 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 1.0.0.1 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 1.0.0.1 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 169.254.169.254 (IPv4)
[00:04:13] 192.30.255.112
[00:04:13] 192.30.255.112
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 github.com via 169.254.169.254 (IPv6)
[00:04:13] 192.30.255.112
[00:04:13] 192.30.255.112
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:04:13] 52.219.24.105
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:04:13] 52.219.24.105
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 8.8.8.8 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 8.8.8.8 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:04:13] 52.219.24.105
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:04:13] 52.219.24.129
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 8.8.4.4 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 8.8.4.4 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:04:13] 52.219.28.37
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:04:13] 52.219.24.105
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 1.1.1.1 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 1.1.1.1 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 1.0.0.1 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 1.0.0.1 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 169.254.169.254 (IPv4)
[00:04:13] 192.30.255.112
[00:04:13] 192.30.255.112
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 github.com via 169.254.169.254 (IPv6)
[00:04:13] 192.30.255.112
[00:04:13] 192.30.255.112
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:04:13] 52.219.24.105
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:04:13] 52.219.24.105
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 3 github.com via 8.8.8.8 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 3 github.com via 8.8.8.8 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:06 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv4)
[00:04:13] 52.219.20.65
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8 (IPv6)
[00:04:13] 52.219.20.65
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 8.8.4.4 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 8.8.4.4 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv4)
[00:04:13] 52.219.20.97
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4 (IPv6)
[00:04:13] 52.219.28.37
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 1.1.1.1 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 1.1.1.1 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv4)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1 (IPv6)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 1.0.0.1 (IPv4)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 1.0.0.1 (IPv6)
[00:04:13] 192.30.253.113
[00:04:13] 192.30.253.113
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv4)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1 (IPv6)
[00:04:13] 54.231.237.25
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 169.254.169.254 (IPv4)
[00:04:13] 192.30.255.112
[00:04:13] 192.30.255.112
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 github.com via 169.254.169.254 (IPv6)
[00:04:13] 192.30.255.112
[00:04:13] 192.30.255.112
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv4)
[00:04:13] 52.219.24.105
[00:04:13] ---> Thu Aug  9 20:24:07 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 169.254.169.254 (IPv6)
[00:04:14] travis_fold:end:dig
travis_fold:end:check_dns_9696

travis_time:end:018e9d6e:start=1533845993333367716,finish=1533846247567817433,duration=254234449717
---
travis_time:end:030acbba:start=1533846247857275883,finish=1533846247865133300,duration=7857417
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c9e1723
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:068182d4
travis_time:start:068182d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:24371d28
$ dmesg | grep -i kill
