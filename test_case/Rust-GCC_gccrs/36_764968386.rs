
$ git log | head -1
commit 93b991e60438c7fb939a4cc8df737e6f7a142c07
$ cat hello.rs
fn main() {
    println!("Hello, world!");
}
$ TMPDIR=/tmp /opt/gccrs/bin/gccrs.exe -v hello.rs
Using built-in specs.
COLLECT_GCC=/opt/gccrs/bin/gccrs
COLLECT_LTO_WRAPPER=/opt/gccrs/libexec/gcc/x86_64-pc-cygwin/11.0.0/lto-wrapper.exe
Target: x86_64-pc-cygwin
Configured with: ./configure --prefix=/opt/gccrs --disable-bootstrap --enable-multilib --enable-languages=c,c++,rust
Thread model: single
Supported LTO compression algorithms: zlib zstd
gcc version 11.0.0 20201127 (experimental) (GCC)
COLLECT_GCC_OPTIONS='-v' '-shared-libgcc' '-mtune=generic' '-march=x86-64' '-dumpdir' 'a-'
 /opt/gccrs/libexec/gcc/x86_64-pc-cygwin/11.0.0/rust1.exe hello.rs -quiet -dumpdir a- -dumpbase hello.rs -dumpbase-ext .rs -mtune=generic -march=x86-64 -version -L/opt/gccrs/lib/gcc/x86_64-pc-cygwin/11.0.0 -L/opt/gccrs/lib/gcc/x86_64-pc-cygwin/11.0.0/../../../../lib -L/lib/../lib -L/usr/lib/../lib -L/opt/gccrs/lib/gcc/x86_64-pc-cygwin/11.0.0/../../.. -o /tmp/ccWBnGUF.s
GNU Rust (GCC) version 11.0.0 20201127 (experimental) (x86_64-pc-cygwin)
        compiled by GNU C version 10.2.0, GMP version 6.2.1, MPFR version 4.1.0, MPC version 1.2.1, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=30 --param ggc-min-heapsize=4096
GNU Rust (GCC) version 11.0.0 20201127 (experimental) (x86_64-pc-cygwin)
        compiled by GNU C version 10.2.0, GMP version 6.2.1, MPFR version 4.1.0, MPC version 1.2.1, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=30 --param ggc-min-heapsize=4096
Preparing to parse files.
Attempting to parse file: hello.rs
beginning null denotation identifier handling
current peek token when starting path pratt parse: '!'
current token (just about to return path to null denotation): '!'
finished null denotation identifier path parsing - next is branching
ast token created with str 'Hello, world!'
finished parsing new delim token tree - peeked token is now ';' while t is ')'
successfully parsed macro invocation (via partial)
SUCCESSFULLY PARSED CRATE
ran register_plugins (with no body)
SUCCESSFULLY REGISTERED PLUGINS
started injection
finished injection
SUCCESSFULLY FINISHED INJECTION
started expansion
finished expansion
SUCCESSFULLY FINISHED EXPANSION
started name resolution
finished name resolution
SUCCESSFULLY FINISHED RESOLUTION
started lowering AST
hello.rs:2:5: fatal error: Failed to lower expr: [MacroInvocation: println!("Hello, world!")]
    2 |     println!("Hello, world!");
      |     ^
compilation terminated.
$
