plain
== end clock drift check ==
Starting sccache server...
configure: processing command line
Traceback (most recent call last):
  File "/checkout/src/bootstrap/configure.py", line 452, in <module>
configure: build.build          := x86_64-unknown-linux-gnu
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-9/bin/llv ...
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-9/bin/llv ...
    configure_section(targets[target], section_config[target])
  File "/checkout/src/bootstrap/configure.py", line 442, in configure_section
    raise RuntimeError("failed to find config line for {}".format(key))
RuntimeError: failed to find config line for llvm-config
configure: rust.thin-lto-import-instr-limit := 10
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: llvm.ccache          := sccache
