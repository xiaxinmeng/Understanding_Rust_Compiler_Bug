
error[E0461]: couldn't find crate `dependency` with expected target triple x86_64-unknown-linux-gnu
  --> /home/joshua/rustc4/src/test/ui/cfg-dependent.rs:8:2
   |
LL |     dependency::is_64();
   |     ^^^^^^^^^^
   |
   = note: the following crate versions were found:
           crate `dependency`, target triple i686-unknown-linux-gnu: /home/joshua/rustc4/build/x86_64-unknown-linux-gnu/test/ui/cfg-dependent/auxiliary/libdependency.so
