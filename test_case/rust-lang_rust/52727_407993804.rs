plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:06793748
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
[00:03:50]  Paused: 0
[00:03:50]  Stopped: 0
[00:03:50] Images: 38
[00:03:50] Server Version: 17.09.0-ce
[00:03:50] Storage Driver: overlay2
[00:03:50]  Backing Filesystem: extfs
[00:03:50]  Supports d_type: true
[00:03:50]  Native Overlay Diff: true
[00:03:50] Logging Driver: json-file
[00:03:50] Cgroup Driver: cgroupfs
[00:03:50] Plugins:
[00:03:50]  Volume: local
[00:03:50]  Network: bridge host macvlan null overlay
[00:03:50]  Log: awslogs fluentd gcplogs gelf journald json-file logentries splunk syslog
[00:03:50] Swarm: inactive
[00:03:50] Runtimes: runc
[00:03:50] Default Runtime: runc
[00:03:50] Init Binary: docker-init
[00:03:50] containerd version: 06b9cb35161009dcb7123345749fef02f7cea8e0
[00:03:50] runc version: 3f2f8b84a77f73d38244dd690525642a72156c64
[00:03:50] init version: 949e6fa
[00:03:50] Operating System: Ubuntu 14.04.5 LTS
[00:03:50] OSType: linux
[00:03:50] Architecture: x86_64
[00:03:50] CPUs: 4
[00:03:50] CPUs: 4
[00:03:50] Total Memory: 14.69GiB
[00:03:50] Name: travis-job-8a6f012a-0029-41d5-823e-d019bf6b5b93
[00:03:50] ID: J77F:SVXQ:EP4C:IR3Y:V2DJ:PELZ:PRXV:M5XF:Q4MB:UZ2A:3WMW:XPUK
[00:03:50] Docker Root Dir: /var/lib/docker
[00:03:50] Debug Mode (client): false
[00:03:50] Debug Mode (server): false
[00:03:50] Registry: https://index.docker.io/v1/
[00:03:50] Insecure Registries:
[00:03:50]  127.0.0.0/8
[00:03:50]  127.0.0.0/8
[00:03:50] Live Restore Enabled: false
[00:03:50] 
[00:03:50] search c.eco-emissary-99515.internal google.internal
[00:03:50] nameserver 8.8.8.8
[00:03:50] nameserver 8.8.4.4
[00:03:50] nameserver 1.1.1.1
[00:03:50] nameserver 1.0.0.1
[00:03:50] ----
[00:03:50] cat: /etc/network/interfaces: No such file or directory

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0eb1de1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:087b27e8:start=1532587432078588932,finish=1532587432084882879,duration=6293947
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cf929c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c3315c0
travis_time:start:0c3315c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bbeb353
$ dmesg | grep -i kill
