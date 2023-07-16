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
Fetched 2,526 kB in 30s (81.6 kB/s)
Fetched 2,526 kB in 30s (81.6 kB/s)
E: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/pool/main/b/babeltrace/libbabeltrace1_1.3.2-1_amd64.deb  Could not connect to apt.cache.travis-ci.com:80 (34.96.81.152), connection timed out
E: Failed to fetch http://us-east-1.ec2.archive.ubuntu.com/ubuntu/pool/main/b/babeltrace/libbabeltrace-ctf1_1.3.2-1_amd64.deb  Unable to connect to apt.cache.travis-ci.com:http:
E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
travis_fold:start:apt-get.diagnostics
apt-get install failed
apt-get install failed
$ cat ${TRAVIS_HOME}/apt-get-update.log
Get:1 http://security.ubuntu.com/ubuntu xenial-security InRelease [109 kB]
Get:2 http://apt.postgresql.org/pub/repos/apt xenial-pgdg InRelease [51.5 kB]
Get:3 http://apt.postgresql.org/pub/repos/apt xenial-pgdg/main amd64 Packages [205 kB]
Get:4 http://apt.postgresql.org/pub/repos/apt xenial-pgdg/main i386 Packages [204 kB]
Get:6 http://security.ubuntu.com/ubuntu xenial-security/main Sources [185 kB]
Get:7 http://security.ubuntu.com/ubuntu xenial-security/restricted Sources [2,243 B]
Get:8 http://security.ubuntu.com/ubuntu xenial-security/universe Sources [130 kB]
Get:9 http://security.ubuntu.com/ubuntu xenial-security/multiverse Sources [3,517 B]
Get:9 http://security.ubuntu.com/ubuntu xenial-security/multiverse Sources [3,517 B]
Get:10 http://security.ubuntu.com/ubuntu xenial-security/main amd64 Packages [857 kB]
Get:11 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-updates InRelease [109 kB]
Get:12 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial-backports InRelease [107 kB]
Get:13 http://security.ubuntu.com/ubuntu xenial-security/main i386 Packages [704 kB]
Get:14 http://security.ubuntu.com/ubuntu xenial-security/main Translation-en [382 kB]
Get:15 http://security.ubuntu.com/ubuntu xenial-security/restricted amd64 Packages [12.7 kB]
Get:16 http://security.ubuntu.com/ubuntu xenial-security/restricted i386 Packages [12.7 kB]
Get:17 http://security.ubuntu.com/ubuntu xenial-security/restricted Translation-en [2,204 B]
Get:19 http://security.ubuntu.com/ubuntu xenial-security/universe i386 Packages [482 kB]
Get:20 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main Sources [1,103 kB]
Get:21 http://security.ubuntu.com/ubuntu xenial-security/universe Translation-en [240 kB]
Get:22 http://security.ubuntu.com/ubuntu xenial-security/multiverse amd64 Packages [6,121 B]
