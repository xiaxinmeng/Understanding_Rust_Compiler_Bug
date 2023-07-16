plain
 ---> 2afb3e7bef8f
Step 3/8 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   gcc-multilib   make   ninja-build   file   curl   ca-certificates   python2.7   python3.9   git   cmake   sudo   gdb   llvm-12-tools   llvm-12-dev   libedit-dev   libssl-dev   pkg-config   zlib1g-dev   xz-utils   nodejs
 ---> Using cache
 ---> 357fae1e02d2
Step 4/8 : RUN apt-get update &&     apt-get install -y apt-transport-https software-properties-common &&     curl -s "https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb" > packages-microsoft-prod.deb &&     dpkg -i packages-microsoft-prod.deb &&     apt-get update &&     apt-get install -y powershell
 ---> 080d1843107f
Step 5/8 : COPY scripts/sccache.sh /scripts/
 ---> Using cache
 ---> 4a6c76c56ba3
