plain
configure: rust.thin-lto-import-instr-limit := 10
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: llvm.ccache          := sccache
  File "/checkout/src/bootstrap/configure.py", line 465, in <module>
    configure_section(sections[section_key], section_config)
  File "/checkout/src/bootstrap/configure.py", line 453, in configure_section
    raise RuntimeError("failed to find config line for {}".format(key))
configure: build.submodules     := False
configure: build.locked-deps    := True
configure: build.cargo-native-static := True
configure: dist.compression-formats := ['xz']
