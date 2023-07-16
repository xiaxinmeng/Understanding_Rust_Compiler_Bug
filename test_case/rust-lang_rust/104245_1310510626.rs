plain
== end clock drift check ==
sccache: Starting the server...
Traceback (most recent call last):
configure: processing command line
  File "/checkout/src/bootstrap/configure.py", line 469, in <module>
    configure_section(sections[section_key], section_config)
  File "/checkout/src/bootstrap/configure.py", line 457, in configure_section
    raise RuntimeError("failed to find config line for {}".format(key))
configure: build.build          := x86_64-unknown-linux-gnu
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-13/bin/ll ...
configure: llvm.link-shared     := True
configure: rust.thin-lto-import-instr-limit := 10
