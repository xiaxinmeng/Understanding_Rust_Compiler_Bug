plain
travis_time:end:0082f334:start=1551175199493104002,finish=1551175272346776090,duration=72853672088
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
Attempting to download s3://rust-lang-ci-sccache2/docker/b5493eee5f15270054538ad6f259fd35cdcde8a43e617233529277e394d7185f136e6026d0c390061a3c94b487e75625efa873039c2128199e11805e10d47851
[00:00:15] Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://s3-us-west-1.amazonaws.com/rust-lang-ci-sccache2/docker/b5493eee5f15270054538ad6f259fd35cdcde8a43e617233529277e394d7185f136e6026d0c390061a3c94b487e75625efa873039c2128199e11805e10d47851
[00:00:15]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:00:15]                                  Dload  Upload   Total   Spent    Left  Speed
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
