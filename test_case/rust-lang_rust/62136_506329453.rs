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
Get:1 http://security.ubuntu.com/ubuntu xenial-security InRelease [109 kB]
Get:2 http://apt.postgresql.org/pub/repos/apt xenial-pgdg InRelease [51.5 kB]
Get:3 http://apt.postgresql.org/pub/repos/apt xenial-pgdg/main amd64 Packages [206 kB]
Get:4 http://apt.postgresql.org/pub/repos/apt xenial-pgdg/main i386 Packages [206 kB]
Get:6 http://security.ubuntu.com/ubuntu xenial-security/restricted Sources [2,243 B]
Get:7 http://security.ubuntu.com/ubuntu xenial-security/universe Sources [131 kB]
Get:8 http://security.ubuntu.com/ubuntu xenial-security/multiverse Sources [3,517 B]
Get:9 http://security.ubuntu.com/ubuntu xenial-security/main amd64 Packages [882 kB]
Get:9 http://security.ubuntu.com/ubuntu xenial-security/main amd64 Packages [882 kB]
Get:10 http://security.ubuntu.com/ubuntu xenial-security/main i386 Packages [722 kB]
Get:11 http://security.ubuntu.com/ubuntu xenial-security/main Translation-en [391 kB]
Get:12 http://security.ubuntu.com/ubuntu xenial-security/restricted amd64 Packages [12.7 kB]
Get:13 http://security.ubuntu.com/ubuntu xenial-security/restricted i386 Packages [12.7 kB]
Get:14 http://security.ubuntu.com/ubuntu xenial-security/restricted Translation-en [2,204 B]
Get:16 http://security.ubuntu.com/ubuntu xenial-security/universe i386 Packages [491 kB]
Get:17 http://security.ubuntu.com/ubuntu xenial-security/universe Translation-en [243 kB]
Get:18 http://security.ubuntu.com/ubuntu xenial-security/multiverse amd64 Packages [6,121 B]
Get:19 http://security.ubuntu.com/ubuntu xenial-security/multiverse i386 Packages [6,297 B]
Get:19 http://security.ubuntu.com/ubuntu xenial-security/multiverse i386 Packages [6,297 B]
Get:20 http://security.ubuntu.com/ubuntu xenial-security/multiverse Translation-en [2,699 B]
Err:21 http://archive.ubuntu.com/ubuntu xenial InRelease
  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
Err:22 http://archive.ubuntu.com/ubuntu xenial-updates InRelease
  Unable to connect to apt.cache.travis-ci.com:http:
Err:23 http://archive.ubuntu.com/ubuntu xenial-backports InRelease
  Unable to connect to apt.cache.travis-ci.com:http:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/xenial/InRelease  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/xenial-updates/InRelease  Unable to connect to apt.cache.travis-ci.com:http:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/xenial-backports/InRelease  Unable to connect to apt.cache.travis-ci.com:http:
travis_fold:end:apt-get.diagnostics
travis_fold:end:apt-get.diagnostics
The command "sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb" failed and exited with 100 during .
Your build has been stopped.
