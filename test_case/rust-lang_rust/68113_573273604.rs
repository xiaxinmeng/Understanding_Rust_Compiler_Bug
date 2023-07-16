plain
2020-01-11T02:54:47.2909073Z 1a0f3a523f04: Pull complete
2020-01-11T02:54:47.2933443Z Digest: sha256:181800dada370557133a502977d0e3f7abda0c25b9bbb035f199f5eb6082a114
2020-01-11T02:54:47.2945512Z Status: Downloaded newer image for ubuntu:16.04
2020-01-11T02:54:47.2951761Z  ---> c6a43cd4801e
2020-01-11T02:54:47.2952799Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:54:52.7345082Z Get:1 http://security.ubuntu.com/ubuntu xenial-security InRelease [109 kB]
2020-01-11T02:54:52.8143845Z Get:2 http://archive.ubuntu.com/ubuntu xenial InRelease [247 kB]
2020-01-11T02:54:53.4085331Z Get:3 http://security.ubuntu.com/ubuntu xenial-security/main amd64 Packages [1031 kB]
2020-01-11T02:54:53.5753982Z Get:4 http://archive.ubuntu.com/ubuntu xenial-updates InRelease [109 kB]
---
2020-01-11T02:56:50.2545721Z Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
2020-01-11T02:56:50.2545826Z 
2020-01-11T02:56:50.2546199Z Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
2020-01-11T02:56:50.2546540Z  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
2020-01-11T02:56:50.9244566Z gpg: keyring `/tmp/tmp3tlwahyf/secring.gpg' created
2020-01-11T02:56:50.9245571Z gpg: keyring `/tmp/tmp3tlwahyf/pubring.gpg' created
2020-01-11T02:56:50.9246935Z gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
2020-01-11T02:56:51.4317139Z gpg: /tmp/tmp3tlwahyf/trustdb.gpg: trustdb created
2020-01-11T02:56:51.4336972Z gpg: no ultimately trusted keys found
2020-01-11T02:56:51.4340089Z gpg: Total number processed: 1
2020-01-11T02:56:51.4340462Z gpg:               imported: 1  (RSA: 1)
2020-01-11T02:56:51.5039134Z OK
---
2020-01-11T02:57:08.8893825Z Setting up gcc-arm-embedded (7-2018q2-1~xenial1) ...
2020-01-11T02:57:08.8924398Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2020-01-11T02:57:20.2205836Z Removing intermediate container 7a1517189bb2
2020-01-11T02:57:20.2207326Z  ---> 78bf3705ff95
2020-01-11T02:57:20.2208011Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:57:20.2249678Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:57:21.3579012Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:57:21.3579431Z 
2020-01-11T02:57:21.4029424Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:57:21.4032737Z  ---> c6a43cd4801e
2020-01-11T02:57:21.4032737Z  ---> c6a43cd4801e
2020-01-11T02:57:21.4036494Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:57:21.4041839Z  ---> 2f38e5c3cd3a
2020-01-11T02:57:21.4044577Z Step 3/65 : WORKDIR /build
2020-01-11T02:57:21.4047347Z  ---> Using cache
2020-01-11T02:57:21.4049878Z  ---> 79087e80d538
2020-01-11T02:57:21.4049878Z  ---> 79087e80d538
2020-01-11T02:57:21.4053177Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:57:21.4055737Z  ---> Using cache
2020-01-11T02:57:21.4058179Z  ---> 78bf3705ff95
2020-01-11T02:57:21.4061517Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:57:21.4071632Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:57:23.4917471Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:57:23.4918010Z 
2020-01-11T02:57:23.5204174Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:57:23.5216431Z  ---> c6a43cd4801e
2020-01-11T02:57:23.5216431Z  ---> c6a43cd4801e
2020-01-11T02:57:23.5217686Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:57:23.5218466Z  ---> 2f38e5c3cd3a
2020-01-11T02:57:23.5218565Z Step 3/65 : WORKDIR /build
2020-01-11T02:57:23.5218786Z  ---> Using cache
2020-01-11T02:57:23.5219027Z  ---> 79087e80d538
2020-01-11T02:57:23.5219027Z  ---> 79087e80d538
2020-01-11T02:57:23.5219417Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:57:23.5219694Z  ---> Using cache
2020-01-11T02:57:23.5219934Z  ---> 78bf3705ff95
2020-01-11T02:57:23.5220344Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:57:23.5220515Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:57:26.6168506Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:57:26.6169242Z 
2020-01-11T02:57:26.6401589Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:57:26.6405147Z  ---> c6a43cd4801e
2020-01-11T02:57:26.6405147Z  ---> c6a43cd4801e
2020-01-11T02:57:26.6408581Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:57:26.6412481Z  ---> 2f38e5c3cd3a
2020-01-11T02:57:26.6414432Z Step 3/65 : WORKDIR /build
2020-01-11T02:57:26.6416336Z  ---> Using cache
2020-01-11T02:57:26.6418104Z  ---> 79087e80d538
2020-01-11T02:57:26.6418104Z  ---> 79087e80d538
2020-01-11T02:57:26.6420627Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:57:26.6422417Z  ---> Using cache
2020-01-11T02:57:26.6424736Z  ---> 78bf3705ff95
2020-01-11T02:57:26.6427723Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:57:26.6450481Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:57:30.8129215Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:57:30.8130054Z 
2020-01-11T02:57:30.8325664Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:57:30.8333159Z  ---> c6a43cd4801e
2020-01-11T02:57:30.8333159Z  ---> c6a43cd4801e
2020-01-11T02:57:30.8335176Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:57:30.8342385Z  ---> 2f38e5c3cd3a
2020-01-11T02:57:30.8344940Z Step 3/65 : WORKDIR /build
2020-01-11T02:57:30.8348866Z  ---> Using cache
2020-01-11T02:57:30.8349356Z  ---> 79087e80d538
2020-01-11T02:57:30.8349356Z  ---> 79087e80d538
2020-01-11T02:57:30.8350176Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:57:30.8353513Z  ---> Using cache
2020-01-11T02:57:30.8353951Z  ---> 78bf3705ff95
2020-01-11T02:57:30.8354515Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:57:30.8398802Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:57:30.8490433Z 
2020-01-11T02:57:30.8490433Z 
2020-01-11T02:57:30.8553819Z ##[error]Bash exited with code '1'.
2020-01-11T02:57:30.8586512Z ##[section]Starting: Checkout
2020-01-11T02:57:30.8588381Z ==============================================================================
2020-01-11T02:57:30.8588494Z Task         : Get sources
2020-01-11T02:57:30.8588582Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
