
$ /usr/lib/ld-linux-x86-64.so.2 --help
Usage: /usr/lib/ld-linux-x86-64.so.2 [OPTION]... EXECUTABLE-FILE [ARGS-FOR-PROGRAM...]
You have invoked 'ld.so', the program interpreter for dynamically-linked
ELF programs.  Usually, the program interpreter is invoked automatically
when a dynamically-linked executable is started.

You may invoke the program interpreter program directly from the command
line to load and run an ELF executable file; this is like executing that
file itself, but always uses the program interpreter you invoked,
instead of the program interpreter specified in the executable file you
run.  Invoking the program interpreter directly provides access to
additional diagnostics, and changing the dynamic linker behavior without
setting environment variables (which would be inherited by subprocesses).

  --list                list all dependencies and how they are resolved
  --verify              verify that given object really is a dynamically linked
                        object we can handle
  --inhibit-cache       Do not use /etc/ld.so.cache
  --library-path PATH   use given PATH instead of content of the environment
                        variable LD_LIBRARY_PATH
  --glibc-hwcaps-prepend LIST
                        search glibc-hwcaps subdirectories in LIST
  --glibc-hwcaps-mask LIST
                        only search built-in subdirectories if in LIST
  --inhibit-rpath LIST  ignore RUNPATH and RPATH information in object names
                        in LIST
  --audit LIST          use objects named in LIST as auditors
  --preload LIST        preload objects named in LIST
  --argv0 STRING        set argv[0] to STRING before running
  --list-tunables       list all tunables with minimum and maximum values
  --list-diagnostics    list diagnostics information
  --help                display this help and exit
  --version             output version information and exit

This program interpreter self-identifies as: /usr/lib/ld-linux-x86-64.so.2

Shared library search path:
  (libraries located via /etc/ld.so.cache)
  /usr/lib (system search path)

Subdirectories of glibc-hwcaps directories, in priority order:
  x86-64-v4
  x86-64-v3
  x86-64-v2 (supported, searched)

Legacy HWCAP subdirectories under library search path directories:
  x86_64 (AT_PLATFORM; supported, searched)
  tls (supported, searched)
  avx512_1
  x86_64 (supported, searched)
