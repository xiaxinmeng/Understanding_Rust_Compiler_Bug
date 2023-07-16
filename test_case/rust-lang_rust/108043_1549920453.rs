plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:af9a3f1b7e12a54c737d8aa371acc8d05cb83a8f)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Ign:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source

The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:1 http://archive.ubuntu.com/ubuntu jammy InRelease
Ign:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
E: Unable to locate package mingw-w64
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
EPackage ca-certificates is not available, but is referred to by another package.
EPackage ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
E: Unable to locate package mingw-w64
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:3 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Ign:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Err:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
E: Unable to locate package mingw-w64
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
Ign:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source

The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
##[error]Process completed with exit code 1.
Post job cleanup.
