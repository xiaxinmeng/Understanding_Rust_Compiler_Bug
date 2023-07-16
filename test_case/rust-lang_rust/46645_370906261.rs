
OVERVIEW: LLVM Linker

USAGE: ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/bin/lld [options] <inputs>

OPTIONS:
  --emit-relocs          Generate relocations in output
  --entry <entry>        Name of entry point symbol
  --global-base=<value>  Where to start to place global data
  --import-memory        Import memory from the environment
  --initial-memory=<value>
                         Initial size of the linear memory
  --max-memory=<value>   Maximum size of the linear memory
  --no-entry             Do not output any entry point
  --relocatable          Create relocatable object file
