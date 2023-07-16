plain
Step 7/9 : ENV NO_DOWNLOAD_CI_LLVM 1
 ---> Running in 803cf303ebdc
Removing intermediate container 803cf303ebdc
 ---> ebdbf5dc5b6d
Step 8/9 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-13       --enable-llvm-link-shared       --set rust.randomize-layout true       --set rust.thin-lto-import-instr-limit=10
Removing intermediate container 49c203623301
 ---> a83a9ff69656
Step 9/9 : ENV SCRIPT ../x.py --stage 2 test --exclude src/tools/tidy &&            ../x --stage 2 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            ../x.ps1 --stage 2 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 2 test src/tools/tidy
 ---> Running in 8f10e691a134
 ---> Running in 8f10e691a134
Removing intermediate container 8f10e691a134
 ---> 8259d5aae12e
Successfully built 8259d5aae12e
Successfully tagged rust-ci:latest
Built container sha256:8259d5aae12e4d843d136045d41d2c5807e9ebdcbd8f8fca3627a562b609c393
Uploading finished image to https://ci-caches.rust-lang.org/docker/3551f82503324308eca1af34630649e10d873929011e79a2b22414311d5b4808174173e111fabe8e8a431a468ffdc0f7217625af3b6e0de427f220f613e41569
upload failed: - to s3://rust-lang-ci-sccache2/docker/3551f82503324308eca1af34630649e10d873929011e79a2b22414311d5b4808174173e111fabe8e8a431a468ffdc0f7217625af3b6e0de427f220f613e41569 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
