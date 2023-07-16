
$ rustc - --crate-type staticlib     # just a quick way to create an archive with valid object files
$ cat "some metadata" > lib.rmeta
$ ar q librust_out.a lib.rmeta       # add lib.rmeta to the librust_out.a archive
$ ./mold -m elf_x86_64 lib.rmeta
mold: fatal: lib.rmeta: unknown file type
