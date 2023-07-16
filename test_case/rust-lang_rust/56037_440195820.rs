plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0b8843a4
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_time:end:2206cb8a:start=1542693872892792235,finish=1542693872986267751,duration=93475516
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_time:start:070e4b30
$ sudo mkdir -p  /checkout/obj &&
sudo chmod 0777 /checkout/obj &&
sudo tcpdump -vv -nn -i docker0 udp port 53 > /checkout/obj/udpdump.txt &
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
tcpdump: listening on docker0, link-type EN10MB (Ethernet), capture size 262144 bytes
travis_time:start:0d8c175b
rm 'src/llvm'
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm.tar.gz &&         curl -sSL -o download-src-llvm.tar.gz https://github.com/rust-lang/llvm/archive/7051ead40a5f825878b59bf08d4e768be9e99a4a.tar.gz
[00:00:00] rm 'src/doc/book'
---
[00:48:11] -- Check for working CXX compiler: /usr/local/bin/sccache -- works
[00:48:11] -- Detecting CXX compiler ABI info
[00:48:21] -- Detecting CXX compiler ABI info - done
[00:48:21] -- Detecting CXX compile features
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
