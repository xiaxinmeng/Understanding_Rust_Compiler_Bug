plain
$ sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb
Reading package lists...
Building dependency tree...
Reading state information...
Some packages could not be installed. This may mean that you have
requested an impossible situation or if you are using the unstable
distribution that some required packages have not yet been created
or been moved out of Incoming.
The following information may help to resolve the situation:
The following packages have unmet dependencies:
 gdb : Depends: libbabeltrace-ctf1 (>= 1.2.1) but it is not installable
       Depends: libbabeltrace1 (>= 1.2.1) but it is not installable
E: Unable to correct problems, you have held broken packages.
travis_fold:start:apt-get.diagnostics
apt-get install failed
apt-get install failed
$ cat ${TRAVIS_HOME}/apt-get-update.log
Get:2 http://security.ubuntu.com/ubuntu xenial-security/main Sources [185 kB]
Get:3 http://security.ubuntu.com/ubuntu xenial-security/restricted Sources [2,243 B]
Get:4 http://security.ubuntu.com/ubuntu xenial-security/universe Sources [131 kB]
Get:5 http://security.ubuntu.com/ubuntu xenial-security/multiverse Sources [3,517 B]
Get:5 http://security.ubuntu.com/ubuntu xenial-security/multiverse Sources [3,517 B]
Get:6 http://security.ubuntu.com/ubuntu xenial-security/main amd64 Packages [868 kB]
Get:7 http://security.ubuntu.com/ubuntu xenial-security/main i386 Packages [709 kB]
Get:8 http://security.ubuntu.com/ubuntu xenial-security/main Translation-en [385 kB]
Get:9 http://security.ubuntu.com/ubuntu xenial-security/restricted amd64 Packages [12.7 kB]
Get:10 http://security.ubuntu.com/ubuntu xenial-security/restricted i386 Packages [12.7 kB]
Get:11 http://security.ubuntu.com/ubuntu xenial-security/restricted Translation-en [2,204 B]
Get:13 http://security.ubuntu.com/ubuntu xenial-security/universe i386 Packages [484 kB]
Get:14 http://security.ubuntu.com/ubuntu xenial-security/universe Translation-en [242 kB]
Get:15 http://security.ubuntu.com/ubuntu xenial-security/multiverse amd64 Packages [6,121 B]
Get:16 http://security.ubuntu.com/ubuntu xenial-security/multiverse i386 Packages [6,297 B]
Get:16 http://security.ubuntu.com/ubuntu xenial-security/multiverse i386 Packages [6,297 B]
Get:17 http://security.ubuntu.com/ubuntu xenial-security/multiverse Translation-en [2,699 B]
Err:18 http://apt.postgresql.org/pub/repos/apt xenial-pgdg InRelease
  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
Err:19 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial InRelease
  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
Err:20 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates InRelease
  Unable to connect to apt.cache.travis-ci.com:http:
Err:21 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-backports InRelease
  Unable to connect to apt.cache.travis-ci.com:http:
Reading package lists...
Reading package lists...
W: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/dists/xenial/InRelease  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
W: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/dists/xenial-updates/InRelease  Unable to connect to apt.cache.travis-ci.com:http:
W: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/dists/xenial-backports/InRelease  Unable to connect to apt.cache.travis-ci.com:http:
W: Failed to fetch http://apt.postgresql.org/pub/repos/apt/dists/xenial-pgdg/InRelease  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
travis_fold:end:apt-get.diagnostics
travis_fold:end:apt-get.diagnostics
The command "sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb" failed and exited with 100 during .
Your build has been stopped.
