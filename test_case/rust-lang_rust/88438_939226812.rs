shell
#!/bin/sh
rustc +dev -C opt-level=3 --crate-type rlib a.rs -Zshare-generics
rustc +dev -C opt-level=3 --crate-type rlib --extern a=liba.rlib b.rs -Zshare-generics
rustc +dev -C opt-level=3 --crate-type rlib --extern b=libb.rlib -L dependency=. \
  -Zsymbol-mangling-version=v0 -Zshare-generics --emit=llvm-ir -Cno-prepopulate-passes -Cpasses=name-anon-globals \
  c.rs
