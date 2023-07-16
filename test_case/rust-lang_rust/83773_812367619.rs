
$ cat Makefile
all:
    echo $(RUSTC) $(RUSTDOC)
$ x.py test src/test/run-make/exit-code
echo /home/joshua/rustc3/build/x86_64-unknown-linux-gnu/stage1/bin/rustc /home/joshua/rustc3/build/x86_64-unknown-linux-gnu/stage1/bin/rustdoc
