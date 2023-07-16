plain
2020-01-11T02:45:02.3006930Z 1a0f3a523f04: Pull complete
2020-01-11T02:45:02.3031937Z Digest: sha256:181800dada370557133a502977d0e3f7abda0c25b9bbb035f199f5eb6082a114
2020-01-11T02:45:02.3049319Z Status: Downloaded newer image for ubuntu:16.04
2020-01-11T02:45:02.3054956Z  ---> c6a43cd4801e
2020-01-11T02:45:02.3056153Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:45:05.8587908Z Get:1 http://security.ubuntu.com/ubuntu xenial-security InRelease [109 kB]
2020-01-11T02:45:05.9243720Z Get:2 http://archive.ubuntu.com/ubuntu xenial InRelease [247 kB]
2020-01-11T02:45:06.0379521Z Get:3 http://security.ubuntu.com/ubuntu xenial-security/main amd64 Packages [1031 kB]
2020-01-11T02:45:06.1712706Z Get:4 http://security.ubuntu.com/ubuntu xenial-security/restricted amd64 Packages [12.7 kB]
---
2020-01-11T02:46:40.6931435Z Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
2020-01-11T02:46:40.6931498Z 
2020-01-11T02:46:40.6931763Z Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
2020-01-11T02:46:40.6932023Z  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
2020-01-11T02:46:41.9497936Z gpg: keyring `/tmp/tmphkjkxue4/secring.gpg' created
2020-01-11T02:46:41.9505083Z gpg: keyring `/tmp/tmphkjkxue4/pubring.gpg' created
2020-01-11T02:46:41.9505391Z gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
2020-01-11T02:46:41.9505911Z gpg: /tmp/tmphkjkxue4/trustdb.gpg: trustdb created
2020-01-11T02:46:41.9506811Z gpg: no ultimately trusted keys found
2020-01-11T02:46:41.9507214Z gpg: Total number processed: 1
2020-01-11T02:46:41.9507406Z gpg:               imported: 1  (RSA: 1)
2020-01-11T02:46:41.9507897Z OK
---
2020-01-11T02:46:55.9559168Z Setting up gcc-arm-embedded (7-2018q2-1~xenial1) ...
2020-01-11T02:46:55.9590462Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2020-01-11T02:47:06.8664354Z Removing intermediate container 6076b9adabe7
2020-01-11T02:47:06.8665686Z  ---> bf64037fca0e
2020-01-11T02:47:06.8666318Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:47:06.8755495Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:47:07.9452792Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:47:08.4574145Z 
2020-01-11T02:47:08.4574904Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:47:08.4575683Z  ---> c6a43cd4801e
2020-01-11T02:47:08.4575683Z  ---> c6a43cd4801e
2020-01-11T02:47:08.4576653Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:47:08.4577774Z  ---> 9519007b7960
2020-01-11T02:47:08.4578014Z Step 3/65 : WORKDIR /build
2020-01-11T02:47:08.4578346Z  ---> Using cache
2020-01-11T02:47:08.4578685Z  ---> 8efacfae47b8
2020-01-11T02:47:08.4578685Z  ---> 8efacfae47b8
2020-01-11T02:47:08.4579136Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:47:08.4579507Z  ---> Using cache
2020-01-11T02:47:08.4579940Z  ---> bf64037fca0e
2020-01-11T02:47:08.4580412Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:47:08.4580659Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:47:10.0439937Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:47:10.0440389Z 
2020-01-11T02:47:10.0633589Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:47:10.0634452Z  ---> c6a43cd4801e
2020-01-11T02:47:10.0634452Z  ---> c6a43cd4801e
2020-01-11T02:47:10.0635728Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:47:10.0636947Z  ---> 9519007b7960
2020-01-11T02:47:10.0637168Z Step 3/65 : WORKDIR /build
2020-01-11T02:47:10.0637880Z  ---> Using cache
2020-01-11T02:47:10.0638096Z  ---> 8efacfae47b8
2020-01-11T02:47:10.0638096Z  ---> 8efacfae47b8
2020-01-11T02:47:10.0638419Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:47:10.0638690Z  ---> Using cache
2020-01-11T02:47:10.0638859Z  ---> bf64037fca0e
2020-01-11T02:47:10.0639186Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:47:10.0641998Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:47:13.1331582Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:47:13.1331757Z 
2020-01-11T02:47:13.1523856Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:47:13.1527336Z  ---> c6a43cd4801e
2020-01-11T02:47:13.1527336Z  ---> c6a43cd4801e
2020-01-11T02:47:13.1528501Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:47:13.1531586Z  ---> 9519007b7960
2020-01-11T02:47:13.1542031Z Step 3/65 : WORKDIR /build
2020-01-11T02:47:13.1592399Z  ---> Using cache
2020-01-11T02:47:13.1592778Z  ---> 8efacfae47b8
2020-01-11T02:47:13.1592778Z  ---> 8efacfae47b8
2020-01-11T02:47:13.1593500Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:47:13.1593904Z  ---> Using cache
2020-01-11T02:47:13.1594105Z  ---> bf64037fca0e
2020-01-11T02:47:13.1594631Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:47:13.1687292Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:47:17.2809740Z Sending build context to Docker daemon  545.8kB
2020-01-11T02:47:17.2815811Z 
2020-01-11T02:47:17.2816352Z Step 1/65 : FROM ubuntu:16.04
2020-01-11T02:47:17.2817120Z  ---> c6a43cd4801e
2020-01-11T02:47:17.2817120Z  ---> c6a43cd4801e
2020-01-11T02:47:17.2818111Z Step 2/65 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   automake   bison   bzip2   flex   help2man   libtool-bin   texinfo   unzip   wget   xz-utils   libncurses-dev   gawk   make   file   curl   ca-certificates   python2.7   python3   git   cmake   sudo   xz-utils   zlib1g-dev   g++-arm-linux-gnueabi   g++-arm-linux-gnueabihf   g++-aarch64-linux-gnu   g++-mips64-linux-gnuabi64   g++-mips64el-linux-gnuabi64   gcc-sparc64-linux-gnu   libc6-dev-sparc64-cross   bzip2   patch   libssl-dev   pkg-config   libnewlib-arm-none-eabi   qemu-system-arm   software-properties-common
2020-01-11T02:47:17.2819263Z  ---> 9519007b7960
2020-01-11T02:47:17.2819463Z Step 3/65 : WORKDIR /build
2020-01-11T02:47:17.2820143Z  ---> Using cache
2020-01-11T02:47:17.2820649Z  ---> 8efacfae47b8
2020-01-11T02:47:17.2820649Z  ---> 8efacfae47b8
2020-01-11T02:47:17.2821286Z Step 4/65 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
2020-01-11T02:47:17.2821728Z  ---> Using cache
2020-01-11T02:47:17.2822093Z  ---> bf64037fca0e
2020-01-11T02:47:17.2822615Z Step 5/65 : COPY scripts/rustbuild-setup.sh dist-various-1/build-riscv-toolchain.sh dist-various-1/riscv64-unknown-linux-gnu.config dist-various-1/crosstool-ng.sh /build
2020-01-11T02:47:17.2822887Z When using COPY with more than one source file, the destination must be a directory and end with a /
2020-01-11T02:47:17.2823222Z 
2020-01-11T02:47:17.2823222Z 
2020-01-11T02:47:17.2872676Z ##[error]Bash exited with code '1'.
2020-01-11T02:47:17.2898049Z ##[section]Starting: Checkout
2020-01-11T02:47:17.2899615Z ==============================================================================
2020-01-11T02:47:17.2899708Z Task         : Get sources
2020-01-11T02:47:17.2899775Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
