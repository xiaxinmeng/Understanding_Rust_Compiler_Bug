
$ rustc
usage: rustc [options] <input>

options:

    -h --help          display this message
    -v --version       print version info and exit

    -o <filename>      write output to <filename>
    --out-dir <dir>    write output to compiler-chosen filename in <dir>
    --lib              compile a library crate
    --bin              compile an executable crate (default)
    --static           use or produce static libraries
    --no-core          omit the 'core' library (used and imported by default)
    --pretty [type]    pretty-print the input instead of compiling
    --ls               list the symbols defined by a crate file
    -L <path>          add a directory to the library search path
    --no-verify        suppress LLVM verification step (slight speedup)
    --parse-only       parse only; do not compile, assemble, or link
    --no-trans         run all passes except translation; no output
    -g                 produce debug info
    --opt-level <lvl>  optimize with possible levels 0-3
    -O                 equivalent to --opt-level=2
    -S                 compile only; do not assemble or link
    --no-asm-comments  do not add comments into the assembly source
    -c                 compile and assemble, but do not link
    --emit-llvm        produce an LLVM bitcode file
    --save-temps       write intermediate files in addition to normal output
    --stats            gather and report various compilation statistics
    --cfg <cfgspec>    configure the compilation environment
    --time-passes      time the individual phases of the compiler
    --time-llvm-passes time the individual phases of the LLVM backend
    --sysroot <path>   override the system root
    --target <triple>  target to compile for (default: host triple)
    --test             build test harness
    --gc               garbage collect shared data (experimental/temporary)
    --warn-unused-imports
                       warn about unnecessary imports
    --no-lint-ctypes   suppress lint-style ctypes usage check
