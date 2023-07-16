
$ rustc hello.rs && objdump -Wf hello | grep ZERO
$ rustc hello.rs -C link-args=-fuse-ld=lld && objdump -Wf hello | grep ZERO
00004d50 ZERO terminator
