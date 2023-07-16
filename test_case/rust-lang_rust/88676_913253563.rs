plain
+ for lib in c++ c_nonshared compiler_rt execinfo gcc pthread rt ssp_nonshared
+ files_to_extract=("${files_to_extract[@]}" "./usr/lib/lib${lib}.*")
+ URL=https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ curl https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ tar xJf - -C /usr/local/x86_64-unknown-freebsd11 --wildcards ./usr/include './usr/lib/*crt*.o' './lib/libc.*' './usr/lib/libc.*' './lib/libcxxrt.*' './usr/lib/libcxxrt.*' './lib/libgcc_s.*' './usr/lib/libgcc_s.*' './lib/libm.*' './usr/lib/libm.*' './lib/libthr.*' './usr/lib/libthr.*' './lib/libutil.*' './usr/lib/libutil.*' './lib/libprocstat.*' './usr/lib/libprocstat.*' './usr/lib/libc++.*' './usr/lib/libc_nonshared.*' './usr/lib/libcompiler_rt.*' './usr/lib/libexecinfo.*' './usr/lib/libgcc.*' './usr/lib/libpthread.*' './usr/lib/librt.*' './usr/lib/libssp_nonshared.*'
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
---
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'

100  125M  100  125M    0     0  11.0M      0  0:00:11  0:00:11 --:--:-- 11.4M
tar: ./lib/libprocstat.*: Not found in archive
tar: Exiting with failure status due to previous errors
The command '/bin/sh -c /tmp/freebsd-toolchain.sh x86_64' returned a non-zero code: 2
Sending build context to Docker daemon  497.2kB

Step 1/12 : FROM ubuntu:18.04
 ---> 54919e10a95d
---
+ for lib in c++ c_nonshared compiler_rt execinfo gcc pthread rt ssp_nonshared
+ files_to_extract=("${files_to_extract[@]}" "./usr/lib/lib${lib}.*")
+ URL=https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ curl https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ tar xJf - -C /usr/local/x86_64-unknown-freebsd11 --wildcards ./usr/include './usr/lib/*crt*.o' './lib/libc.*' './usr/lib/libc.*' './lib/libcxxrt.*' './usr/lib/libcxxrt.*' './lib/libgcc_s.*' './usr/lib/libgcc_s.*' './lib/libm.*' './usr/lib/libm.*' './lib/libthr.*' './usr/lib/libthr.*' './lib/libutil.*' './usr/lib/libutil.*' './lib/libprocstat.*' './usr/lib/libprocstat.*' './usr/lib/libc++.*' './usr/lib/libc_nonshared.*' './usr/lib/libcompiler_rt.*' './usr/lib/libexecinfo.*' './usr/lib/libgcc.*' './usr/lib/libpthread.*' './usr/lib/librt.*' './usr/lib/libssp_nonshared.*'
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
---
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'

100  125M  100  125M    0     0  11.1M      0  0:00:11  0:00:11 --:--:-- 11.1M
tar: ./lib/libprocstat.*: Not found in archive
tar: Exiting with failure status due to previous errors
The command '/bin/sh -c /tmp/freebsd-toolchain.sh x86_64' returned a non-zero code: 2
Sending build context to Docker daemon  497.2kB

Step 1/12 : FROM ubuntu:18.04
 ---> 54919e10a95d
---
+ for lib in c++ c_nonshared compiler_rt execinfo gcc pthread rt ssp_nonshared
+ files_to_extract=("${files_to_extract[@]}" "./usr/lib/lib${lib}.*")
+ URL=https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ curl https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ tar xJf - -C /usr/local/x86_64-unknown-freebsd11 --wildcards ./usr/include './usr/lib/*crt*.o' './lib/libc.*' './usr/lib/libc.*' './lib/libcxxrt.*' './usr/lib/libcxxrt.*' './lib/libgcc_s.*' './usr/lib/libgcc_s.*' './lib/libm.*' './usr/lib/libm.*' './lib/libthr.*' './usr/lib/libthr.*' './lib/libutil.*' './usr/lib/libutil.*' './lib/libprocstat.*' './usr/lib/libprocstat.*' './usr/lib/libc++.*' './usr/lib/libc_nonshared.*' './usr/lib/libcompiler_rt.*' './usr/lib/libexecinfo.*' './usr/lib/libgcc.*' './usr/lib/libpthread.*' './usr/lib/librt.*' './usr/lib/libssp_nonshared.*'
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
---
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'

100  125M  100  125M    0     0  11.3M      0  0:00:11  0:00:11 --:--:-- 11.3M
tar: ./lib/libprocstat.*: Not found in archive
tar: Exiting with failure status due to previous errors
The command '/bin/sh -c /tmp/freebsd-toolchain.sh x86_64' returned a non-zero code: 2
Sending build context to Docker daemon  497.2kB

Step 1/12 : FROM ubuntu:18.04
 ---> 54919e10a95d
---
+ for lib in c++ c_nonshared compiler_rt execinfo gcc pthread rt ssp_nonshared
+ files_to_extract=("${files_to_extract[@]}" "./usr/lib/lib${lib}.*")
+ URL=https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ curl https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ tar xJf - -C /usr/local/x86_64-unknown-freebsd11 --wildcards ./usr/include './usr/lib/*crt*.o' './lib/libc.*' './usr/lib/libc.*' './lib/libcxxrt.*' './usr/lib/libcxxrt.*' './lib/libgcc_s.*' './usr/lib/libgcc_s.*' './lib/libm.*' './usr/lib/libm.*' './lib/libthr.*' './usr/lib/libthr.*' './lib/libutil.*' './usr/lib/libutil.*' './lib/libprocstat.*' './usr/lib/libprocstat.*' './usr/lib/libc++.*' './usr/lib/libc_nonshared.*' './usr/lib/libcompiler_rt.*' './usr/lib/libexecinfo.*' './usr/lib/libgcc.*' './usr/lib/libpthread.*' './usr/lib/librt.*' './usr/lib/libssp_nonshared.*'
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
---

 99  125M   99  124M    0     0  11.3M      0  0:00:11  0:00:11 --:--:-- 11.3Mtar: Ignoring unknown extended header keyword 'SCHILY.fflags'

100  125M  100  125M    0     0  11.3M      0  0:00:11  0:00:11 --:--:-- 11.3M
tar: ./lib/libprocstat.*: Not found in archive
tar: Exiting with failure status due to previous errors
The command '/bin/sh -c /tmp/freebsd-toolchain.sh x86_64' returned a non-zero code: 2
Sending build context to Docker daemon  497.2kB

Step 1/12 : FROM ubuntu:18.04
 ---> 54919e10a95d
---
+ for lib in c++ c_nonshared compiler_rt execinfo gcc pthread rt ssp_nonshared
+ files_to_extract=("${files_to_extract[@]}" "./usr/lib/lib${lib}.*")
+ URL=https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ curl https://ci-mirrors.rust-lang.org/rustc/2020-08-09-freebsd-amd64-11.4-RELEASE-base.txz
+ tar xJf - -C /usr/local/x86_64-unknown-freebsd11 --wildcards ./usr/include './usr/lib/*crt*.o' './lib/libc.*' './usr/lib/libc.*' './lib/libcxxrt.*' './usr/lib/libcxxrt.*' './lib/libgcc_s.*' './usr/lib/libgcc_s.*' './lib/libm.*' './usr/lib/libm.*' './lib/libthr.*' './usr/lib/libthr.*' './lib/libutil.*' './usr/lib/libutil.*' './lib/libprocstat.*' './usr/lib/libprocstat.*' './usr/lib/libc++.*' './usr/lib/libc_nonshared.*' './usr/lib/libcompiler_rt.*' './usr/lib/libexecinfo.*' './usr/lib/libgcc.*' './usr/lib/libpthread.*' './usr/lib/librt.*' './usr/lib/libssp_nonshared.*'
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
---
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'

100  125M  100  125M    0     0  11.1M      0  0:00:11  0:00:11 --:--:-- 11.2M
tar: ./lib/libprocstat.*: Not found in archive
tar: Exiting with failure status due to previous errors
The command '/bin/sh -c /tmp/freebsd-toolchain.sh x86_64' returned a non-zero code: 2
##[error]Process completed with exit code 1.
Post job cleanup.
