plain
Need to get 2,649 kB of archives.
After this operation, 7,904 kB of additional disk space will be used.
Ign:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace1 amd64 1.3.2-1
Ign:2 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace-ctf1 amd64 1.3.2-1
Err:3 http://security.ubuntu.com/ubuntu xenial-security/main amd64 gdb amd64 7.11.1-0ubuntu1~16.5
  Unable to connect to apt.cache.travis-ci.com:http:
Ign:2 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace-ctf1 amd64 1.3.2-1
Ign:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace1 amd64 1.3.2-1
Ign:2 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace-ctf1 amd64 1.3.2-1
Ign:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace1 amd64 1.3.2-1
Ign:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace1 amd64 1.3.2-1
Ign:2 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace-ctf1 amd64 1.3.2-1
Ign:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace1 amd64 1.3.2-1
Ign:2 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace-ctf1 amd64 1.3.2-1
Err:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace1 amd64 1.3.2-1
  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
Err:2 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 libbabeltrace-ctf1 amd64 1.3.2-1
  Unable to connect to apt.cache.travis-ci.com:http:
Fetched 2,526 kB in 30s (81.5 kB/s)
Fetched 2,526 kB in 30s (81.5 kB/s)
E: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/pool/main/b/babeltrace/libbabeltrace1_1.3.2-1_amd64.deb  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
E: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/pool/main/b/babeltrace/libbabeltrace-ctf1_1.3.2-1_amd64.deb  Unable to connect to apt.cache.travis-ci.com:http:
E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
travis_fold:start:apt-get.diagnostics
apt-get install failed
apt-get install failed
$ cat ${TRAVIS_HOME}/apt-get-update.log
Get:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial InRelease [247 kB]
Get:2 http://apt.postgresql.org/pub/repos/apt xenial-pgdg InRelease [51.5 kB]
Get:4 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates InRelease [109 kB]
Get:5 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-backports InRelease [107 kB]
Get:5 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-backports InRelease [107 kB]
Get:6 http://apt.postgresql.org/pub/repos/apt xenial-pgdg/main amd64 Packages [205 kB]
Get:7 http://apt.postgresql.org/pub/repos/apt xenial-pgdg/main i386 Packages [205 kB]
Get:8 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main Sources [1,103 kB]
Get:9 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/restricted Sources [5,179 B]
Get:10 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/universe Sources [9,802 kB]
Get:12 http://security.ubuntu.com/ubuntu xenial-security/main Sources [185 kB]
Get:13 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 Packages [1,558 kB]
Get:14 http://security.ubuntu.com/ubuntu xenial-security/restricted Sources [2,243 B]
Get:15 http://security.ubuntu.com/ubuntu xenial-security/universe Sources [131 kB]
Get:15 http://security.ubuntu.com/ubuntu xenial-security/universe Sources [131 kB]
Get:16 http://security.ubuntu.com/ubuntu xenial-security/multiverse Sources [3,517 B]
Get:17 http://security.ubuntu.com/ubuntu xenial-security/main amd64 Packages [868 kB]
Get:18 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main i386 Packages [1,552 kB]
Get:19 http://security.ubuntu.com/ubuntu xenial-security/main i386 Packages [709 kB]
Get:20 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main Translation-en [799 kB]
Get:21 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/restricted amd64 Packages [14.1 kB]
Get:22 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/restricted i386 Packages [14.5 kB]
Get:23 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/restricted Translation-en [3,019 B]
Get:24 http://security.ubuntu.com/ubuntu xenial-security/main Translation-en [385 kB]
Get:25 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/universe amd64 Packages [9,827 kB]
Get:27 http://security.ubuntu.com/ubuntu xenial-security/restricted i386 Packages [12.7 kB]
Get:27 http://security.ubuntu.com/ubuntu xenial-security/restricted i386 Packages [12.7 kB]
Get:28 http://security.ubuntu.com/ubuntu xenial-security/restricted Translation-en [2,204 B]
Get:30 http://security.ubuntu.com/ubuntu xenial-security/universe i386 Packages [484 kB]
Get:31 http://security.ubuntu.com/ubuntu xenial-security/universe Translation-en [242 kB]
Get:32 http://security.ubuntu.com/ubuntu xenial-security/multiverse amd64 Packages [6,121 B]
Get:33 http://security.ubuntu.com/ubuntu xenial-security/multiverse i386 Packages [6,297 B]
Get:33 http://security.ubuntu.com/ubuntu xenial-security/multiverse i386 Packages [6,297 B]
Get:34 http://security.ubuntu.com/ubuntu xenial-security/multiverse Translation-en [2,699 B]
Get:35 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/universe i386 Packages [9,804 kB]
Get:36 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/universe Translation-en [6,256 kB]
Get:38 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/multiverse i386 Packages [172 kB]
Get:39 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/multiverse Translation-en [131 kB]
Get:40 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/main Sources [425 kB]
Get:41 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/restricted Sources [2,696 B]
Get:41 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/restricted Sources [2,696 B]
Get:42 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/universe Sources [323 kB]
Get:43 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/multiverse Sources [9,428 B]
Get:44 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/main amd64 Packages [1,261 kB]
Get:45 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/main i386 Packages [1,076 kB]
Get:46 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/main Translation-en [549 kB]
Get:47 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/restricted amd64 Packages [13.1 kB]
Get:48 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/restricted i386 Packages [13.1 kB]
Get:49 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/restricted Translation-en [2,337 B]
Get:51 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates/universe i386 Packages [887 kB]
Get:52 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-
