plain
  gdb
0 upgraded, 1 newly installed, 0 to remove and 216 not upgraded.
Need to get 2,199 kB of archives.
After this operation, 6,474 kB of additional disk space will be used.
Err:1 http://security.ubuntu.com/ubuntu trusty-security/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3
  Could not connect to build-cache.travisci.net:80 (10.80.1.2), connection timed out
Err:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3
  Could not connect to build-cache.travisci.net:80 (10.80.1.2), connection timed out
Err:1 http://security.ubuntu.com/ubuntu trusty-security/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3
  Could not connect to build-cache.travisci.net:80 (10.80.1.2), connection timed out
Err:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3
  Could not connect to build-cache.travisci.net:80 (10.80.1.2), connection timed out
Err:1 http://security.ubuntu.com/ubuntu trusty-security/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3
  Could not connect to build-cache.travisci.net:80 (10.80.1.2), connection timed out
Err:1 http://security.ubuntu.com/ubuntu trusty-security/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3
  Could not connect to build-cache.travisci.net:80 (10.80.1.2), connection timed out
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gdb/gdb_7.7.1-0ubuntu5~14.04.3_amd64.deb  Could not connect to build-cache.travisci.net:80 (10.80.1.2), connection timed out
E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
travis_fold:start:apt-get.diagnostics
apt-get install failed
apt-get install failed
$ cat ${TRAVIS_HOME}/apt-get-update.log
Get:2 http://dl.hhvm.com/ubuntu trusty InRelease [3,106 B]
Get:3 http://ppa.launchpad.net/git-core/ppa/ubuntu trusty InRelease [15.4 kB]
Get:4 http://ppa.launchpad.net/pollinate/ppa/ubuntu trusty InRelease [15.4 kB]
Get:4 http://ppa.launchpad.net/pollinate/ppa/ubuntu trusty InRelease [15.4 kB]
Get:6 http://apt.postgresql.org/pub/repos/apt trusty-pgdg InRelease [61.4 kB]
Get:8 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates InRelease [65.9 kB]
Get:9 http://dl.hhvm.com/ubuntu trusty/main amd64 Packages [1,820 B]
Get:10 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports InRelease [65.9 kB]
Get:11 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty Release [58.5 kB]
Get:11 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty Release [58.5 kB]
Get:12 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty Release.gpg [933 B]
Ign:5 http://toolbelt.heroku.com/ubuntu ./ InRelease
Get:13 http://toolbelt.heroku.com/ubuntu ./ Release [1,609 B]
Get:14 http://toolbelt.heroku.com/ubuntu ./ Release.gpg [473 B]
Get:16 http://security.ubuntu.com/ubuntu trusty-security/restricted Sources [5,050 B]
Get:17 http://security.ubuntu.com/ubuntu trusty-security/universe Sources [117 kB]
Get:18 http://security.ubuntu.com/ubuntu trusty-security/multiverse Sources [3,070 B]
Get:19 http://security.ubuntu.com/ubuntu trusty-security/main amd64 Packages [982 kB]
Get:19 http://security.ubuntu.com/ubuntu trusty-security/main amd64 Packages [982 kB]
Get:20 http://security.ubuntu.com/ubuntu trusty-security/main i386 Packages [898 kB]
Get:21 http://security.ubuntu.com/ubuntu trusty-security/main Translation-en [516 kB]
Get:22 http://security.ubuntu.com/ubuntu trusty-security/restricted amd64 Packages [18.1 kB]
Get:23 http://security.ubuntu.com/ubuntu trusty-security/restricted i386 Packages [17.8 kB]
Get:24 http://security.ubuntu.com/ubuntu trusty-security/restricted Translation-en [3,272 B]
Get:26 http://ppa.launchpad.net/git-core/ppa/ubuntu trusty/main amd64 Packages [3,491 B]
Get:27 http://ppa.launchpad.net/git-core/ppa/ubuntu trusty/main i386 Packages [3,488 B]
Get:27 http://ppa.launchpad.net/git-core/ppa/ubuntu trusty/main i386 Packages [3,488 B]
Get:28 http://ppa.launchpad.net/git-core/ppa/ubuntu trusty/main Translation-en [2,332 B]
Get:30 http://security.ubuntu.com/ubuntu trusty-security/universe Translation-en [190 kB]
Get:31 http://security.ubuntu.com/ubuntu trusty-security/multiverse amd64 Packages [4,726 B]
Get:32 http://security.ubuntu.com/ubuntu trusty-security/multiverse i386 Packages [4,882 B]
Get:32 http://security.ubuntu.com/ubuntu trusty-security/multiverse i386 Packages [4,882 B]
Get:33 http://security.ubuntu.com/ubuntu trusty-security/multiverse Translation-en [2,426 B]
Get:34 https://download.docker.com/linux/ubuntu trusty InRelease [37.1 kB]
Get:36 http://ppa.launchpad.net/pollinate/ppa/ubuntu trusty/main i386 Packages [430 B]
Get:37 http://ppa.launchpad.net/pollinate/ppa/ubuntu trusty/main Translation-en [374 B]
Get:37 http://ppa.launchpad.net/pollinate/ppa/ubuntu trusty/main Translation-en [374 B]
Get:38 http://apt.postgresql.org/pub/repos/apt trusty-pgdg/main amd64 Packages [198 kB]
Get:39 http://apt.postgresql.org/pub/repos/apt trusty-pgdg/main i386 Packages [198 kB]
Get:41 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/restricted Sources [6,449 B]
Get:42 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/universe Sources [282 kB]
Get:43 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/multiverse Sources [7,285 B]
Get:44 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main amd64 Packages [1,408 kB]
Get:44 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main amd64 Packages [1,408 kB]
Get:45 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main i386 Packages [1,318 kB]
Get:46 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main Translation-en [676 kB]
Get:47 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/restricted amd64 Packages [21.4 kB]
Get:48 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/restricted i386 Packages [21.1 kB]
Get:49 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/restricted Translation-en [3,704 B]
Get:51 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/universe i386 Packages [632 kB]
Get:52 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/universe Translation-en [335 kB]
Get:53 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/multiverse amd64 Packages [16.0 kB]
Get:54 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/multiverse i386 Packages [16.5 kB]
Get:54 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/multiverse i386 Packages [16.5 kB]
Get:55 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/multiverse Translation-en [7,680 B]
Get:56 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/main Sources [10.4 kB]
Get:57 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/restricted Sources [40 B]
Get:58 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/universe Sources [41.3 kB]
Get:59 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/multiverse Sources [1,747 B]
Get:60 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/main amd64 Packages [14.7 kB]
Get:61 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/main i386 Packages [14.7 kB]
Get:62 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/main Translation-en [7,426 B]
Get:63 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/restricted amd64 Packages [40 B]
Get:64 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/restricted i386 Packages [40 B]
Get:65 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/restricted Translation-en [40 B]
Get:66 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/universe amd64 Packages [52.5 kB]
Get:68 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/universe i386 Packages [52.4 kB]
Get:67 http://toolbelt.heroku.com/ubuntu ./ Packages [636 B]
Get:69 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/universe Translation-en [40.0 kB]
Get:70 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/multiverse amd64 Packages [1,392 B]
Get:71 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/multiverse i386 Packages [1,376 B]
Get:72 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-backports/multiverse Translation-en [1,165 B]
Get:73 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty/main Sources [1,335 kB]
Get:74 https://download.docker.com/linux/ubuntu trusty/stable amd64 Packages [4,619 B]
Get:75 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty/restricted Sources [5,335 B]
Get:76 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty/universe Sources [7,926 kB]
Get:77 https://download.docker.com/linux/ubuntu trusty/edge amd64 Packages [5,780 B]
Get:79 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty/main amd64 Packages [1,743 kB]
Get:80 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty/main i386 Packages [1,739 kB]
Get:81 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty/restricted amd64 Packages [16.0 kB]
Get:82 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty/restricted i386 Packages [16.4 kB]
---
Get:88 https://packagecloud.io/github/git-lfs/ubuntu trusty InRelease [23.2 kB]
Get:89 https://packagecloud.io/computology/apt-backport/ubuntu trusty/main Sources [843 B]
Get:90 https://packagecloud.io/computology/apt-backport/ubuntu trusty/main amd64 Packages [3,625 B]
Get:91 https://packagecloud.io/computology/apt-backport/ubuntu trusty/main i386 Packages [848 B]
Get:92 https://packagecloud.io/github/git-lfs/ubuntu trusty/main Sources [20 B]
Get:93 https://packagecloud.io/github/git-lfs/ubuntu trusty/main amd64 Packages [7,433 B]
Get:94 https://packagecloud.io/github/git-lfs/ubuntu trusty/main i386 Packages [7,192 B]
Reading package lists...
travis_fold:end:apt-get.diagnostics
travis_fold:end:apt-get.diagnostics
The command "sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb" failed and exited with 100 during .
Your build has been stopped.
