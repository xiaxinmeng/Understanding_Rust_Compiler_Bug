
* Add a configure option --enable-relro to decide whether -z relro should
  be enabled in ELF linker by default.  Default to yes for all Linux
  targets except FRV, HPPA, IA64 and MIPS.  Note that -z relro can increase
  disk and memory size.
