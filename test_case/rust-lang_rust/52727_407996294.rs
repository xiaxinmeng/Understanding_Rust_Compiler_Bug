plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:2292c8f0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/1c/95/6bf862d284c9117eb50b3832cf39780248744e2bda3bbbcd9c5f12c489bf/awscli-1.15.65-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
---
[00:00:11] travis_fold:end:init_repo

[00:00:11] +src/ci/docker/run.sh dist-x86_64-linux
[00:00:11] travis_time:end:1b89cd4a:start=1532587906840070687,finish=1532587918513161304,duration=11673090617
---> Thu Jul 26 06:51:58 UTC 2018: try 0 github.com via 8.8.8.8:
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.113
[00:00:11] 192.30.253.113
[00:00:11] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:00:11] 54.231.235.29
[00:00:11] 54.231.235.29
[00:00:11] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 github.com via 8.8.4.4:
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.113
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.112
[00:00:11] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:00:11] 52.219.24.9
[00:00:11] 52.219.24.9
[00:00:11] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 github.com via 1.1.1.1:
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.113
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.112
[00:00:11] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:00:11] 52.219.28.1
[00:00:11] 52.219.28.1
[00:00:11] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 github.com via 1.0.0.1:
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.113
[00:00:11] 192.30.253.112
[00:00:11] 192.30.253.112
[00:00:11] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:00:12] 52.219.28.1
[00:00:12] 52.219.28.1
[00:00:12] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 github.com via 169.254.169.254:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:58 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:00:12] 54.231.235.33
[00:00:12] 54.231.235.33
[00:00:12] ---> Thu Jul 26 06:51:58 UTC 2018: try 1 github.com via 8.8.8.8:
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:58 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:00:12] 52.219.24.9
[00:00:12] 52.219.24.9
[00:00:12] ---> Thu Jul 26 06:51:58 UTC 2018: try 1 github.com via 8.8.4.4:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:00:12] 52.219.24.21
[00:00:12] 52.219.24.21
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 1 github.com via 1.1.1.1:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:00:12] 52.219.20.81
[00:00:12] 52.219.20.81
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 1 github.com via 1.0.0.1:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:00:12] 52.219.20.81
[00:00:12] 52.219.20.81
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 1 github.com via 169.254.169.254:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:00:12] 54.231.235.33
[00:00:12] 54.231.235.33
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 github.com via 8.8.8.8:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:00:12] 52.219.20.49
[00:00:12] 52.219.20.49
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 github.com via 8.8.4.4:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.113
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:00:12] 52.219.24.9
[00:00:12] 52.219.24.9
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 github.com via 1.1.1.1:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:00:12] 52.219.20.81
[00:00:12] 52.219.20.81
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 github.com via 1.0.0.1:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:00:12] 52.219.20.81
[00:00:12] 52.219.20.81
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 github.com via 169.254.169.254:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:00:12] 54.231.235.33
[00:00:12] 54.231.235.33
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 github.com via 8.8.8.8:
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.113
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:00:12] 52.219.20.97
[00:00:12] 52.219.20.97
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 github.com via 8.8.4.4:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.113
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:00:12] 52.219.20.49
[00:00:12] 52.219.20.49
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 github.com via 1.1.1.1:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:00:12] 52.219.20.81
[00:00:12] 52.219.20.81
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 github.com via 1.0.0.1:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:00:12] 52.219.20.81
[00:00:12] 52.219.20.81
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 github.com via 169.254.169.254:
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.113
[00:00:12] 192.30.253.112
[00:00:12] 192.30.253.112
[00:00:12] ---> Thu Jul 26 06:51:59 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:00:12] 54.231.235.33
[00:00:12] 54.231.235.33
[00:00:12] ===> Much Success
travis_time:start:18cccb73
Attempting to download s3://rust-lang-ci-sccache2/docker/80641fb0e3e2708e0249a1020a1741da14dcdf61ac94bb881e4a235bb3c51feb736a3f634f85c8b5cecb09b7b3d5b77bfd810413bb5f243d2dcb2d01bfd3a63f
[00:00:12] Attempting with retry: curl -f -L -C - -o /tmp/rustci_docker_cache https://s3-us-west-1.amazonaws.com/rust-lang-ci-sccache2/docker/80641fb0e3e2708e0249a1020a1741da14dcdf61ac94bb881e4a235bb3c51feb736a3f634f85c8b5cecb09b7b3d5b77bfd810413bb5f243d2dcb2d01bfd3a63f
[00:00:12]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
[00:02:53]  Paused: 0
[00:02:53]  Stopped: 0
[00:02:53] Images: 38
[00:02:53] Server Version: 17.09.0-ce
[00:02:53] Storage Driver: overlay2
[00:02:53]  Backing Filesystem: extfs
[00:02:53]  Supports d_type: true
[00:02:53]  Native Overlay Diff: true
[00:02:53] Logging Driver: json-file
[00:02:53] Cgroup Driver: cgroupfs
[00:02:53] Plugins:
[00:02:53]  Volume: local
[00:02:53]  Network: bridge host macvlan null overlay
[00:02:53]  Log: awslogs fluentd gcplogs gelf journald json-file logentries splunk syslog
[00:02:53] Swarm: inactive
[00:02:53] Runtimes: runc
[00:02:53] Default Runtime: runc
[00:02:53] Init Binary: docker-init
[00:02:53] containerd version: 06b9cb35161009dcb7123345749fef02f7cea8e0
[00:02:53] runc version: 3f2f8b84a77f73d38244dd690525642a72156c64
[00:02:53] init version: 949e6fa
[00:02:53] Operating System: Ubuntu 14.04.5 LTS
[00:02:53] OSType: linux
[00:02:53] Architecture: x86_64
[00:02:53] CPUs: 4
[00:02:53] CPUs: 4
[00:02:53] Total Memory: 14.69GiB
[00:02:53] Name: travis-job-976a48f9-9fae-46a0-b89a-85a1078bb2be
[00:02:53] ID: J77F:SVXQ:EP4C:IR3Y:V2DJ:PELZ:PRXV:M5XF:Q4MB:UZ2A:3WMW:XPUK
[00:02:53] Docker Root Dir: /var/lib/docker
[00:02:53] Debug Mode (client): false
[00:02:53] Debug Mode (server): false
[00:02:53] Registry: https://index.docker.io/v1/
[00:02:53] Insecure Registries:
[00:02:53]  127.0.0.0/8
[00:02:53]  127.0.0.0/8
[00:02:53] Live Restore Enabled: false
[00:02:53] 
[00:02:54] search c.eco-emissary-99515.internal google.internal
[00:02:54] nameserver 8.8.8.8
[00:02:54] nameserver 8.8.4.4
[00:02:54] nameserver 1.1.1.1
[00:02:54] nameserver 1.0.0.1
[00:02:54] ----
[00:02:54] core.repositoryformatversion=0
[00:02:54] core.filemode=true
[00:02:54] core.bare=false
[00:02:54] core.logallrefupdates=true
[00:02:54] remote.origin.url=https://github.com/rust-lang/rust.git
[00:02:54] remote.origin.fetch=+refs/heads/try:refs/remotes/origin/try
[00:02:54] branch.try.remote=origin
[00:02:54] branch.try.merge=refs/heads/try
[00:02:54] submodule.src/dlmalloc.active=true
[00:02:54] submodule.src/dlmalloc.url=https://github.com/alexcrichton/dlmalloc-rs.git
[00:02:54] submodule.src/doc/nomicon.active=true
[00:02:54] submodule.src/doc/nomicon.url=https://github.com/rust-lang-nursery/nomicon.git
[00:02:54] submodule.src/doc/reference.active=true
[00:02:54] submodule.src/doc/reference.url=https://github.com/rust-lang-nursery/reference.git
[00:02:54] submodule.src/jemalloc.active=true
[00:02:54] submodule.src/jemalloc.url=https://github.com/rust-lang/jemalloc.git
[00:02:54] submodule.src/libbacktrace.active=true
[00:02:54] submodule.src/libbacktrace.url=https://github.com/rust-lang-nursery/libbacktrace
[00:02:54] submodule.src/libcompiler_builtins.active=true
[00:02:54] submodule.src/libcompiler_builtins.url=https://github.com/rust-lang-nursery/compiler-builtins
[00:02:54] submodule.src/liblibc.active=true
[00:02:54] submodule.src/liblibc.url=https://github.com/rust-lang/libc.git
[00:02:54] submodule.src/stdsimd.active=true
[00:02:54] submodule.src/stdsimd.url=https://github.com/rust-lang-nursery/stdsimd
[00:02:54] submodule.src/tools/cargo.active=true
[00:02:54] submodule.src/tools/cargo.url=https://github.com/rust-lang/cargo.git
[00:02:54] submodule.src/tools/clippy.active=true
[00:02:54] submodule.src/tools/clippy.url=https://github.com/rust-lang-nursery/rust-clippy.git
[00:02:54] submodule.src/tools/lld.active=true
[00:02:54] submodule.src/tools/lld.url=https://github.com/rust-lang/lld.git
[00:02:54] submodule.src/tools/miri.active=true
[00:02:54] submodule.src/tools/miri.url=https://github.com/solson/miri.git
[00:02:54] submodule.src/tools/rls.active=true
[00:02:54] submodule.src/tools/rls.url=https://github.com/rust-lang-nursery/rls.git
[00:02:54] submodule.src/rust-installer.active=true
[00:02:54] submodule.src/rust-installer.url=https://github.com/rust-lang/rust-installer.git
[00:02:54] submodule.src/tools/rustfmt.active=true
[00:02:54] submodule.src/tools/rustfmt.url=https://github.com/rust-lang-nursery/rustfmt.git
[00:02:54] ----
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 github.com via 8.8.8.8:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:02:54] 52.219.24.41
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 github.com via 8.8.4.4:
[00:02:54] 192.30.253.113
[00:02:54] 192.30.253.113
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:02:54] 54.231.237.37
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 github.com via 1.1.1.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 github.com via 1.0.0.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 github.com via 169.254.169.254:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 169.254.169.254
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 0 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:02:54] 54.231.237.29
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 169.254.169.254
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 github.com via 8.8.8.8:
[00:02:54] 192.30.253.113
[00:02:54] 192.30.253.113
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:02:54] 52.219.20.49
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 github.com via 8.8.4.4:
[00:02:54] 192.30.253.113
[00:02:54] 192.30.253.113
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:02:54] 52.219.24.17
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 github.com via 1.1.1.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 github.com via 1.0.0.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 github.com via 169.254.169.254:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 169.254.169.254
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 1 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:02:54] 54.231.237.29
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 169.254.169.254
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 github.com via 8.8.8.8:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:02:54] 52.219.24.41
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 github.com via 8.8.4.4:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:02:54] 52.219.24.17
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 github.com via 1.1.1.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 github.com via 1.0.0.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 github.com via 169.254.169.254:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 169.254.169.254
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 2 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:02:54] 54.231.237.29
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 169.254.169.254
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 github.com via 8.8.8.8:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.8.8:
[00:02:54] 52.219.24.37
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.8.8
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 github.com via 8.8.4.4:
[00:02:54] 192.30.253.113
[00:02:54] 192.30.253.113
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 8.8.4.4:
[00:02:54] 52.219.20.49
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 8.8.4.4
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 github.com via 1.1.1.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.1.1.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.1.1.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 github.com via 1.0.0.1:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 1.0.0.1:
[00:02:54] 52.219.20.129
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 1.0.0.1
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 github.com via 169.254.169.254:
[00:02:54] 192.30.253.112
[00:02:54] 192.30.253.112
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup github.com via 169.254.169.254
[00:02:54] ---> Thu Jul 26 06:54:41 UTC 2018: try 3 s3-us-west-1.amazonaws.com via 169.254.169.254:
[00:02:54] 54.231.237.29
[00:02:54] dig: add_nameserver failed
[00:02:54]     x Failed to lookup s3-us-west-1.amazonaws.com via 169.254.169.254
[00:02:54] ===> Had 40 failure(s)

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:06e627c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0119aee0:start=1532588082223081626,finish=1532588082228651188,duration=5569562
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fd8e8c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b449aca
travis_time:start:0b449aca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:182c76ba
$ dmesg | grep -i kill
