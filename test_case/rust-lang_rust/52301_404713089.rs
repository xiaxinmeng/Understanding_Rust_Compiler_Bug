
  ld -m elf_x86_64 -z max-page-size=0x200000   --entry=init_module --gc-sections -r -o /home/ubuntu/t2/example-sysctl/examplesysctl.o /home/ubuntu/t2/example-sysctl/target/x86_64-linux-kernel-module/debug/libexample_sysctl.a
