
ubuntu@ip-10-222-153-90:~$ rustc foo.rs && ./foo
Illegal instruction
ubuntu@ip-10-222-153-90:~$ PATH=$HOME/root-2.20.1/bin:$PATH rustc foo.rs && ./foo
Illegal instruction
ubuntu@ip-10-222-153-90:~$ PATH=$HOME/root-2.21.1/bin:$PATH rustc foo.rs && ./foo
Illegal instruction
ubuntu@ip-10-222-153-90:~$ PATH=$HOME/root-2.22/bin:$PATH rustc foo.rs && ./foo
foo
ubuntu@ip-10-222-153-90:~$ PATH=$HOME/root-2.23.2/bin:$PATH rustc foo.rs && ./foo
foo
ubuntu@ip-10-222-153-90:~$ PATH=$HOME/root-2.24/bin:$PATH rustc foo.rs && ./foo
foo
ubuntu@ip-10-222-153-90:~$ PATH=$HOME/root-2.25/bin:$PATH rustc foo.rs && ./foo
foo
