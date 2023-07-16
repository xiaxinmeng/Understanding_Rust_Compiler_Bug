plain
travis_time:end:20857e0e:start=1545254431050877912,finish=1545254432291275494,duration=1240397582
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/fs.rs:589:5
[00:04:34]     |
[00:04:34] 589 | /     fn try_clone(&self) -> io::Result<File> {
[00:04:34] 590 | |         File::try_clone(self)
[00:04:34]     | |_____^
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/fs.rs:587:6
[00:04:34]     |
[00:04:34] 587 | impl TryClone for File {
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
---
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/net/tcp.rs:582:5
[00:04:34]     |
[00:04:34] 582 | /     fn try_clone(&self) -> io::Result<TcpStream> {
[00:04:34] 583 | |         TcpStream::try_clone(self)
[00:04:34]     | |_____^
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/net/tcp.rs:580:6
[00:04:34]     |
[00:04:34] 580 | impl TryClone for TcpStream {
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
---
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/net/tcp.rs:902:5
[00:04:34]     |
[00:04:34] 902 | /     fn try_clone(&self) -> io::Result<TcpListener> {
[00:04:34] 903 | |         TcpListener::try_clone(self)
[00:04:34]     | |_____^
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/net/tcp.rs:900:6
[00:04:34]     |
[00:04:34] 900 | impl TryClone for TcpListener {
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
---
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/net/udp.rs:804:5
[00:04:34]     |
[00:04:34] 804 | /     fn try_clone(&self) -> io::Result<UdpSocket> {
[00:04:34] 805 | |         UdpSocket::try_clone(self)
[00:04:34]     | |_____^
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
[00:04:34] 
[00:04:34] error[E0658]: use of unstable library feature 'try_clone'
[00:04:34]    --> src/libstd/net/udp.rs:802:6
[00:04:34]     |
[00:04:34] 802 | impl TryClone for UdpSocket {
[00:04:34]     |
[00:04:34]     = help: add #![feature(try_clone)] to the crate attributes to enable
[00:04:34] 
llvm/test/MC/X86
---
10956 ./src/llvm/test/MC/Disassembler/AMDGPU
10820 ./src/tools/lldb/unittests
10508 ./src/llvm/test/MC/AMDGPU
10332 ./src/tools/clang/include
10140 ./src/tools/lldb/packages/Python/lldbsuite/test/functionalities/postmortem
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:22629b40
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
---
travis_time:end:08fb57a0:start=1545254716931794425,finish=1545254716938512626,duration=6718201
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:28f7a010
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|ob
