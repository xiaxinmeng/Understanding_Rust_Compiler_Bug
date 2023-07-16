plain
sccache: Starting the server...
Traceback (most recent call last):
configure: processing command line
configure: 
  File "/checkout/src/bootstrap/configure.py", line 469, in <module>
    configure_section(sections[section_key], section_config)
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-13/bin/ll ...
configure: llvm.link-shared     := True
configure: llvm.link-shared     := True
  File "/checkout/src/bootstrap/configure.py", line 457, in configure_section
configure: rust.codegen-units-std := 1
configure: build.static-stdcpp  := 1
configure: rust.verify-llvm-ir  := True
configure: llvm.ccache          := sccache
---
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--build=x86_64-unknown-linux-gnu', '--llvm-r ...
    raise RuntimeError("failed to find config line for {}".format(key))
