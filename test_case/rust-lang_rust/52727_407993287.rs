plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/1c/95/6bf862d284c9117eb50b3832cf39780248744e2bda3bbbcd9c5f12c489bf/awscli-1.15.65-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:01:02]  Paused: 0
[00:01:02]  Stopped: 0
[00:01:02] Images: 6
[00:01:02] Server Version: 17.09.0-ce
[00:01:02] Storage Driver: overlay2
[00:01:02]  Backing Filesystem: extfs
[00:01:02]  Supports d_type: true
[00:01:02]  Native Overlay Diff: true
[00:01:02] Logging Driver: json-file
[00:01:02] Cgroup Driver: cgroupfs
[00:01:02] Plugins:
[00:01:02]  Volume: local
[00:01:02]  Network: bridge host macvlan null overlay
[00:01:02]  Log: awslogs fluentd gcplogs gelf journald json-file logentries splunk syslog
[00:01:02] Swarm: inactive
[00:01:02] Runtimes: runc
[00:01:02] Default Runtime: runc
[00:01:02] Init Binary: docker-init
[00:01:02] containerd version: 06b9cb35161009dcb7123345749fef02f7cea8e0
[00:01:02] runc version: 3f2f8b84a77f73d38244dd690525642a72156c64
[00:01:02] init version: 949e6fa
[00:01:02] Operating System: Ubuntu 14.04.5 LTS
[00:01:02] OSType: linux
[00:01:02] Architecture: x86_64
[00:01:02] CPUs: 4
[00:01:02] CPUs: 4
[00:01:02] Total Memory: 14.69GiB
[00:01:02] Name: travis-job-c1046101-8537-45a9-932a-4a611318830c
[00:01:02] ID: J77F:SVXQ:EP4C:IR3Y:V2DJ:PELZ:PRXV:M5XF:Q4MB:UZ2A:3WMW:XPUK
[00:01:02] Docker Root Dir: /var/lib/docker
[00:01:02] Debug Mode (client): false
[00:01:02] Debug Mode (server): false
[00:01:02] Registry: https://index.docker.io/v1/
[00:01:02] Insecure Registries:
[00:01:02]  127.0.0.0/8
[00:01:02]  127.0.0.0/8
[00:01:02] Live Restore Enabled: false
[00:01:02] 
[00:01:02] search c.eco-emissary-99515.internal google.internal
[00:01:02] nameserver 8.8.8.8
[00:01:02] nameserver 8.8.4.4
[00:01:02] nameserver 1.1.1.1
[00:01:02] nameserver 1.0.0.1
[00:01:02] ----
[00:01:02] cat: /etc/network/interfaces: No such file or directory

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1e2ed6e2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:3037d1a8:start=1532587267369220060,finish=1532587267378000379,duration=8780319
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1219be1b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01c23a5c
travis_time:start:01c23a5c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a720930
$ dmesg | grep -i kill
