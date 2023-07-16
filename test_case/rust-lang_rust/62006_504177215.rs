plain
$ sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb
Reading package lists...
Building dependency tree...
Reading state information...
Package gdb is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
E: Package 'gdb' has no installation candidate
travis_fold:start:apt-get.diagnostics
apt-get install failed
apt-get install failed
$ cat ${TRAVIS_HOME}/apt-get-update.log
Err:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial InRelease
  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
Err:2 http://apt.postgresql.org/pub/repos/apt xenial-pgdg InRelease
  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
Err:3 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates InRelease
  Unable to connect to apt.cache.travis-ci.com:http:
Err:4 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-backports InRelease
  Unable to connect to apt.cache.travis-ci.com:http:
Err:5 http://security.ubuntu.com/ubuntu xenial-security InRelease
  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
Reading package lists...
W: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/dists/xenial/InRelease  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
W: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/dists/xenial-updates/InRelease  Unable to connect to apt.cache.travis-ci.com:http:
W: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/dists/xenial-backports/InRelease  Unable to connect to apt.cache.travis-ci.com:http:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/xenial-security/InRelease  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
W: Failed to fetch http://apt.postgresql.org/pub/repos/apt/dists/xenial-pgdg/InRelease  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
travis_fold:end:apt-get.diagnostics
travis_fold:end:apt-get.diagnostics
The command "sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb" failed and exited with 100 during .
Your build has been stopped.
