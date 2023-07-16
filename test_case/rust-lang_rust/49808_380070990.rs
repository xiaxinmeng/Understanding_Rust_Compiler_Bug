plain
[00:00:01] tar: Skipping to next header
[00:00:02] src/ci/init_repo.sh: line 46:  3745 Segmentation fault      (core dumped) tar -C $module --strip-components=1 -xf $cached
---
[00:00:24] error during connect: Post http://%2Fvar%2Frun%2Fdocker.sock/v1.32/images/load?quiet=1: EOF
[00:00:24] Downloaded containers:\n
[00:00:24] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-powerpc64-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:24] Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?
[00:00:24] time="2018-04-10T11:47:28Z" level=error msg="failed to dial gRPC: cannot connect to the Docker daemon. Is 'docker daemon' running on this host?: dial unix /var/run/docker.sock: connect: connection refused"
[00:00:24] time="2018-04-10T11:47:28Z" level=error msg="Can't add file /home/travis/build/rust-lang/rust/src/ci/docker/armhf-gnu/vexpress_config to tar: io: read/write on closed pipe"
[00:00:24] time="2018-04-10T11:47:28Z" level=error msg="Can't close tar writer: io: read/write on closed pipe"
[00:00:24] Command failed. Attempt 2/5:
[00:00:25] time="2018-04-10T11:47:29Z" level=error msg="failed to dial gRPC: unable to upgrade to h2c, received 501"
[00:00:25] time="2018-04-10T11:47:29Z" level=error msg="Can't add file /home/travis/build/rust-lang/rust/src/ci/docker/dist-armhf-linux/arm-linux-gnueabihf.config to tar: io: read/write on closed pipe"
[00:00:25] time="2018-04-10T11:47:29Z" level=error msg="Can't close tar writer: io: read/write on closed pipe"
[00:00:25] context canceled
---
[00:01:05] Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?
[00:01:05] time="2018-04-10T11:48:10Z" level=error msg="failed to dial gRPC: cannot connect to the Docker daemon. Is 'docker daemon' running on this host?: dial unix /var/run/docker.sock: connect: connection refused"
[00:01:05] Command failed. Attempt 5/5:
[00:01:05] Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?
[00:01:05] The command has failed after 5 attempts.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:23008758:start=1523360890406521546,finish=1523360890414622333,duration=8100787
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:069eaaaf
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:069eaaaf:start=1523360890420113670,finish=1523360890425995570,duration=5881900
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13913500
$ dmesg | grep -i kill
[   10.573652] init: failsafe main process (1095) killed by TERM signal
[   33.307545] init: docker main process (1158) killed by ILL signal
[  133.074003] init: docker main process (4350) killed by SEGV signal
