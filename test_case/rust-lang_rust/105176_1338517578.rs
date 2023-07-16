
[WARNING]: Empty continuation line found in:
    RUN apt-get update && apt-get install -y --no-install-recommends   g++   gcc-multilib   make   ninja-build   file   curl   ca-certificates   python2.7   python3.9   git   cmake   sudo   gdb   llvm-13-tools   llvm-13-dev   libedit-dev   libssl-dev   pkg-config   zlib1g-dev   xz-utils   nodejs     apt-transport-https software-properties-common &&     curl -s "https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb" > packages-microsoft-prod.deb &&     dpkg -i packages-microsoft-prod.deb &&     apt-get update &&     apt-get install -y powershell     && rm -rf /var/lib/apt/lists/*
Warning: : Empty continuation lines will become errors in a future release.
