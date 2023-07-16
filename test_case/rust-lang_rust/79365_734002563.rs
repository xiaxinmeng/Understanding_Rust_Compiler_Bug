
Step 15/21 : RUN ./build-powerpc64le-toolchain.sh
 ---> Running in 715bcfb93bb7
+ source shared.sh
+ BINUTILS=2.32
+ GCC=5.3.0
+ TARGET=powerpc64le-linux-gnu
+ SYSROOT=/usr/local/powerpc64le-linux-gnu/sysroot
+ mkdir -p /usr/local/powerpc64le-linux-gnu/sysroot
+ pushd /usr/local/powerpc64le-linux-gnu/sysroot
+ centos_base=http://vault.centos.org/altarch/7.3.1611/os/ppc64le/Packages/
+ glibc_v=2.17-157.el7
+ kernel_v=3.10.0-514.el7
+ for package in 'glibc{,-devel,-headers}-$glibc_v' 'kernel-headers-$kernel_v'
/usr/local/powerpc64le-linux-gnu/sysroot /tmp
+ curl http://vault.centos.org/altarch/7.3.1611/os/ppc64le/Packages//glibc-2.17-157.el7.ppc64le.rpm
+ rpm2cpio -
+ cpio -idm
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   276  100   276    0     0   1279      0 --:--:-- --:--:-- --:--:--  1277
argument is not an RPM package
cpio: premature end of archive
The command '/bin/sh -c ./build-powerpc64le-toolchain.sh' returned a non-zero code: 1
The command has failed after 5 attempts.
Error: Process completed with exit code 1.
