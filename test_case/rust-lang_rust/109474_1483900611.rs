plain
Step 3/25 : RUN yum upgrade -y &&     yum install -y epel-release &&     yum install -y       automake       bzip2       file       cmake3       gcc       gcc-c++       git       glibc-devel.i686       glibc-devel.x86_64       libedit-devel       libstdc++-devel.i686       libstdc++-devel.x86_64       make       ncurses-devel       openssl-devel       patch       perl       pkgconfig       python3       unzip       wget       xz       zlib-devel.i686       zlib-devel.x86_64       && yum clean all
 ---> Running in c938b084017e
Loaded plugins: fastestmirror, ovl
Determining fastest mirrors
 * base: ziply.mm.fcix.net
 * extras: mirror.arizona.edu
 * updates: mnvoip.mm.fcix.net
--> Running transaction check
---> Package bash.x86_64 0:4.2.46-34.el7 will be updated
---> Package bash.x86_64 0:4.2.46-35.el7_9 will be an update
---> Package bind-license.noarch 32:9.11.4-26.P2.el7 will be updated
---

Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: ziply.mm.fcix.net
 * extras: mirror.arizona.edu
 * updates: mnvoip.mm.fcix.net
--> Running transaction check
---> Package epel-release.noarch 0:7-11 will be installed
--> Finished Dependency Resolution

---

Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: ziply.mm.fcix.net
 * epel: mirrors.syringanetworks.net
 * extras: mirror.arizona.edu
 * updates: mnvoip.mm.fcix.net
Trying other mirror.
Package 1:pkgconfig-0.27.1-4.el7.x86_64 already installed and latest version
Package xz-5.2.2-2.el7_9.x86_64 already installed and latest version
Resolving Dependencies
---
  0     0    0     0    0     0      0      0 --:--:--  0:00:05 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:06 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:07 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:08 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:09 --:--:--     0curl: (6) Could not resolve host: ftp.gnu.org; Unknown error
xzcat: (stdin): File format not recognized
tar: This does not look like a tar archive
The command '/bin/sh -c ./build-gcc.sh && yum remove -y gcc gcc-c++' returned a non-zero code: 2
Command failed. Attempt 2/5:
Sending build context to Docker daemon  687.1kB

---
  0     0    0     0    0     0      0      0 --:--:--  0:00:05 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:06 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:07 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:08 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:09 --:--:--     0curl: (6) Could not resolve host: ftp.gnu.org; Unknown error
xzcat: (stdin): File format not recognized
tar: This does not look like a tar archive
The command '/bin/sh -c ./build-gcc.sh && yum remove -y gcc gcc-c++' returned a non-zero code: 2
Command failed. Attempt 3/5:
Sending build context to Docker daemon  687.1kB

---
  0     0    0     0    0     0      0      0 --:--:--  0:00:05 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:06 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:07 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:08 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:09 --:--:--     0curl: (6) Could not resolve host: ftp.gnu.org; Unknown error
xzcat: (stdin): File format not recognized
tar: This does not look like a tar archive
The command '/bin/sh -c ./build-gcc.sh && yum remove -y gcc gcc-c++' returned a non-zero code: 2
Command failed. Attempt 4/5:
Sending build context to Docker daemon  687.1kB

---
  0     0    0     0    0     0      0      0 --:--:--  0:00:05 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:06 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:07 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:08 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:09 --:--:--     0curl: (6) Could not resolve host: ftp.gnu.org; Unknown error
xzcat: (stdin): File format not recognized
tar: This does not look like a tar archive
The command '/bin/sh -c ./build-gcc.sh && yum remove -y gcc gcc-c++' returned a non-zero code: 2
Command failed. Attempt 5/5:
Sending build context to Docker daemon  687.1kB

---
  0     0    0     0    0     0      0      0 --:--:--  0:00:05 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:06 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:07 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:08 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:09 --:--:--     0curl: (6) Could not resolve host: ftp.gnu.org; Unknown error
xzcat: (stdin): File format not recognized
tar: This does not look like a tar archive
The command '/bin/sh -c ./build-gcc.sh && yum remove -y gcc gcc-c++' returned a non-zero code: 2
The command has failed after 5 attempts.
##[error]Process completed with exit code 1.
Post job cleanup.
