
toula::~/code/rust $ cat /etc/redhat-release
CentOS release 5.7 (Final)
toula::~/code/rust $ make
cfg: shell host triple i686-unknown-linux-gnu
cfg: host for i686-unknown-linux-gnu is i386
cfg: unix-y environment
cfg: using gcc
cfg: no pandoc found, omitting doc/rust.pdf
cfg: no llnextgen found, omitting grammar-verification
cfg: no pandoc found, omitting library doc build
compile: rt/i686-unknown-linux-gnu/rust.o
cc1plus: warnings being treated as errors
./src/rt/util/indexed_list.h: In instantiation of ‘indexed_list<rust_task>’:
./src/rt/rust_task_thread.h:50:   instantiated from here
./src/rt/util/indexed_list.h:29: warning: ‘class indexed_list<rust_task>’ has virtual functions but non-virtual destructor
make: *** [rt/i686-unknown-linux-gnu/rust.o] Error 1
zsh: 6475 exit 2     make
