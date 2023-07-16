toml
# XXX changing this requires ./updcode && ./go to detect/apply changes; actually it requires a ./clean !

# Sample TOML configuration file for building Rust.
#
# To configure rustbuild, copy this file to the directory from which you will be
# running the build, and name it config.toml.
#
# All options are commented out by default in this file, and they're commented
# out with their default values. The build system by default looks for
# `config.toml` in the current directory of a build for build configuration, but
# a custom configuration file can also be specified with `--config` to the build
# system.

# =============================================================================
# Tweaking how LLVM is compiled
# =============================================================================
[llvm]

# Indicates whether rustc will support compilation with LLVM
# note: rustc does not compile without LLVM at the moment
#enabled = true

# Indicates whether the LLVM build is a Release or Debug build
#optimize = true
optimize = true

# Indicates whether LLVM should be built with ThinLTO. Note that this will
# only succeed if you use clang, lld, llvm-ar, and llvm-ranlib in your C/C++
# toolchain (see the `cc`, `cxx`, `linker`, `ar`, and `ranlib` options below).
# More info at: https://clang.llvm.org/docs/ThinLTO.html#clang-bootstrap
#thin-lto = false

# Indicates whether an LLVM Release build should include debug info
#release-debuginfo = false
release-debuginfo = false #faster

# Indicates whether the LLVM assertions are enabled or not
#assertions = false
assertions = false #faster

# Indicates whether ccache is used when building LLVM
#ccache = false
# or alternatively ...
#ccache = "/path/to/ccache"
ccache = "/usr/bin/ccache" #faster

# If an external LLVM root is specified, we automatically check the version by
# default to make sure it's within the range that we're expecting, but setting
# this flag will indicate that this version check should not be done.
#version-check = true

# Link libstdc++ statically into the librustc_llvm instead of relying on a
# dynamic version to be available.
#static-libstdcpp = false
static-libstdcpp = false

# Tell the LLVM build system to use Ninja instead of the platform default for
# the generated build system. This can sometimes be faster than make, for
# example.
#ninja = false
ninja = true

# LLVM targets to build support for.
# Note: this is NOT related to Rust compilation targets. However, as Rust is
# dependent on LLVM for code generation, turning targets off here WILL lead to
# the resulting rustc being unable to compile for the disabled architectures.
# Also worth pointing out is that, in case support for new targets are added to
# LLVM, enabling them here doesn't mean Rust is automatically gaining said
# support. You'll need to write a target specification at least, and most
# likely, teach rustc about the C ABI of the target. Get in touch with the
# Rust team and file an issue if you need assistance in porting!
#targets = "X86;ARM;AArch64;Mips;PowerPC;SystemZ;JSBackend;MSP430;Sparc;NVPTX;Hexagon"
targets = "X86"
#;ARM;AArch64" //;Mips"

# LLVM experimental targets to build support for. These targets are specified in
# the same format as above, but since these targets are experimental, they are
# not built by default and the experimental Rust compilation targets that depend
# on them will not work unless the user opts in to building them. By default the
# `WebAssembly` and `RISCV` targets are enabled when compiling LLVM from scratch.
#experimental-targets = "WebAssembly;RISCV"
experimental-targets = ""

# Cap the number of parallel linker invocations when compiling LLVM.
# This can be useful when building LLVM with debug info, which significantly
# increases the size of binaries and consequently the memory required by
# each linker process.
# If absent or 0, linker invocations are treated like any other job and
# controlled by rustbuild's -j parameter.
link-jobs = 0
#link-jobs = 4

# When invoking `llvm-config` this configures whether the `--shared` argument is
# passed to prefer linking to shared libraries.
#link-shared = false

# When building llvm, this configures what is being appended to the version.
# If absent, we let the version as-is.
#version-suffix = "-rust"



# On MSVC you can compile LLVM with clang-cl, but the test suite doesn't pass
# with clang-cl, so this is special in that it only compiles LLVM with clang-cl
#clang-cl = '/path/to/clang-cl.exe'

# Use libc++ when building LLVM instead of libstdc++. This is the default on
# platforms already use libc++ as the default C++ library, but this option
# allows you to use libc++ even on platforms when it's not. You need to ensure
# that your host compiler ships with libc++.
#use-libcxx = true

# =============================================================================
# General build configuration options
# =============================================================================
[build]

# Build triple for the original snapshot compiler. This must be a compiler that
# nightlies are already produced for. The current platform must be able to run
# binaries of this build triple and the nightly will be used to bootstrap the
# first compiler.
#build = "x86_64-unknown-linux-gnu"    # defaults to your host platform
build = "x86_64-unknown-linux-gnu"

# In addition to the build triple, other triples to produce full compiler
# toolchains for. Each of these triples will be bootstrapped from the build
# triple and then will continue to bootstrap themselves. This platform must
# currently be able to run all of the triples provided here.
#host = ["x86_64-unknown-linux-gnu"]   # defaults to just the build triple
host = ["x86_64-unknown-linux-gnu"]

# In addition to all host triples, other triples to produce the standard library
# for. Each host triple will be used to produce a copy of the standard library
# for each target triple.
target = ["x86_64-unknown-linux-gnu"] # defaults to just the build triple
#target = ["x86_64-unknown-linux-gnu", "i686-unknown-linux-gnu" ] #XXX: disabled i686 to avoid wasting too much time compiling! FIXME: re-add i686 later.
#note: added i686-unknown-linux-gnu to be able to compile ${HOME}/build/2nonpkgs/rust.stuff/rustlearnage/usize/go_32bit

# Instead of downloading the src/stage0.txt version of Cargo specified, use
# this Cargo binary instead to build all Rust code
#cargo = "/path/to/bin/cargo"
#cargo = "/home/xftroxgpx/.cargo/bin/cargo"
#^ brought by rustup !

# Instead of downloading the src/stage0.txt version of the compiler
# specified, use this rustc binary instead as the stage0 snapshot compiler.
#rustc = "/path/to/bin/rustc"
#rustc = "/home/xftroxgpx/.cargo/bin/rustc" #this fails as follows:
#^ brought by rustup !
#rustc = "/home/xftroxgpx/bin/rustc"
#^ will use the rustup version unless we already have a stage2,1 or 0 rustc; this fails as follows:
#XXX: don't set rustc here, or you'll hit this https://github.com/rust-lang/rust/pull/47006#issuecomment-360561167 (see comment above that)  and local-rebuild must not be true also!


# Flag to specify whether any documentation is built. If false, rustdoc and
# friends will still be compiled but they will not be used to generate any
# documentation.
#docs = true
docs = true
#docs = false #faster ./x.py install XXX: undo;  FIXME: NOTE: rustdoc wasn't compiled! only stage0 one was available! well, the message appears: Building rustdoc for stage2 (x86_64-unknown-linux-gnu) (when docs=true, at least) but there's still no rustdoc found!(other than stage0 one) unless it's this one(aka renamed): ./rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/rustdoc_tool_binary  YEP that's the one! and it eventually appears as ./rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc  https://github.com/rust-lang/rust/issues/57458  note: rustdoc is needed to compile cbindgen archlinux package! it fails to with stage0 one due to different compiler generated log crate

# Indicate whether the compiler should be documented in addition to the standard
# library and facade crates.
#compiler-docs = false
#compiler-docs = true
compiler-docs = false #faster ./x.py install XXX: undo

# Indicate whether submodules are managed and updated automatically.
#submodules = true
submodules = true #this and/or fast-submodules=true below, will overwrite changes presumably made by eg. git checkout d6525ef539 in the root rustc/ dir and thus I cannot properly roll back to that working rustc commit because submodules are in fact brought up to date automatically, so still hitting this issue: https://github.com/rust-lang/rust/issues/57596
#submodules = false

# Update submodules only when the checked out commit in the submodules differs
# from what is committed in the main rustc repo.
fast-submodules = true
#fast-submodules = true
#fast-submodules = false

# The path to (or name of) the GDB executable to use. This is only used for
# executing the debuginfo test suite.
#gdb = "gdb"

# The node.js executable to use. Note that this is only used for the emscripten
# target when running tests, otherwise this can be omitted.
#nodejs = "node"

# Python interpreter to use for various tasks throughout the build, notably
# rustdoc tests, the lldb python interpreter, and some dist bits and pieces.
# Note that Python 2 is currently required.
#python = "python2.7"

# Force Cargo to check that Cargo.lock describes the precise dependency
# set that all the Cargo.toml files create, instead of updating it.
#locked-deps = false
locked-deps = false

# Indicate whether the vendored sources are used for Rust dependencies or not
#vendor = false

# Typically# the build system will build the rust compiler twice. The second
# compiler, however, will simply use its own libraries to link against. If you
# would rather to perform a full bootstrap, compiling the compiler three times,
# then you can set this option to true. You shouldn't ever need to set this
# option to true.
#full-bootstrap = false
full-bootstrap = false

# Enable a build of the extended rust tool set which is not only the compiler
# but also tools such as Cargo. This will also produce "combined installers"
# which are used to install Rust and Cargo together. This is disabled by
# default.
#extended = false
extended = false

# Installs chosen set of extended tools if enabled. By default builds all.
# If chosen tool failed to build the installation fails.
#tools = ["cargo", "rls", "clippy", "rustfmt", "analysis", "src"]
tools = [ "src" ]

# Verbosity level: 0 == not verbose, 1 == verbose, 2 == very verbose
#verbose = 0
verbose = 2

# Build the sanitizer runtimes
#sanitizers = false
sanitizers = false

# Build the profiler runtime
#profiler = false
profiler = false

# Indicates whether the native libraries linked into Cargo will be statically
# linked or not.
#cargo-native-static = false


# Run the build with low priority, by setting the process group's "nice" value
# to +10 on Unix platforms, and by using a "low priority" job object on Windows.
#low-priority = false
low-priority = true

# Arguments passed to the `./configure` script, used during distcheck. You
# probably won't fill this in but rather it's filled in by the `./configure`
# script.
#configure-args = []

# Indicates that a local rebuild is occurring instead of a full bootstrap,
# essentially skipping stage0 as the local compiler is recompiling itself again.
local-rebuild = false
#local-rebuild = true #don't to true here or you'll hit this https://github.com/rust-lang/rust/pull/47006#issuecomment-360561167 (see comment above that) and [build] rustc must not be set also!

# Print out how long each rustbuild step took (mostly intended for CI and
# tracking over time)
#print-step-timings = false
print-step-timings = true

# =============================================================================
# General install configuration options
# =============================================================================
[install]

# Instead of installing to /usr/local, install to this path instead.
#prefix = "/usr/local"
#prefix = "/home/xftroxgpx/.cargo" #those binaries seem to be just wrappers to the /home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/* ones
prefix = "/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu"

# Where to install system configuration files
# If this is a relative path, it will get installed in `prefix` above
#sysconfdir = "/etc"

# Where to install documentation in `prefix` above
#docdir = "share/doc/rust"

# Where to install binaries in `prefix` above
#bindir = "bin"
bindir = "bin"

# Where to install libraries in `prefix` above
#libdir = "lib"

# Where to install man pages in `prefix` above
#mandir = "share/man"

# Where to install data in `prefix` above (currently unused)
#datadir = "share"

# Where to install additional info in `prefix` above (currently unused)
#infodir = "share/info"

# Where to install local state (currently unused)
# If this is a relative path, it will get installed in `prefix` above
#localstatedir = "/var/lib"

# =============================================================================
# Options for compiling Rust code itself
# =============================================================================
[rust]


# Whether or not to optimize the compiler and standard library.
#
# Note: the slowness of the non optimized compiler compiling itself usually
#       outweighs the time gains in not doing optimizations, therefore a
#       full bootstrap takes much more time with `optimize` set to false.
#optimize = true
optimize = true

# Indicates that the build should be configured for debugging Rust. A
# `debug`-enabled compiler and standard library will be somewhat
# slower (due to e.g. checking of debug assertions) but should remain
# usable.
#
# Note: If this value is set to `true`, it will affect a number of
#       configuration options below as well, if they have been left
#       unconfigured in this file.
#
# Note: changes to the `debug` setting do *not* affect `optimize`
#       above. In theory, a "maximally debuggable" environment would
#       set `optimize` to `false` above to assist the introspection
#       facilities of debuggers like lldb and gdb. To recreate such an
#       environment, explicitly set `optimize` to `false` and `debug`
#       to `true`. In practice, everyone leaves `optimize` set to
#       `true`, because an unoptimized rustc with debugging
#       enabled becomes *unusably slow* (e.g. rust-lang/rust#24840
#       reported a 25x slowdown) and bootstrapping the supposed
#       "maximally debuggable" environment (notably libstd) takes
#       hours to build.
#
#debug = false
#debug = false #faster
debug = true


# Number of codegen units to use for each compiler invocation. A value of 0
# means "the number of cores on this machine", and 1+ is passed through to the
# compiler.
#codegen-units = 1
codegen-units = 1 #We currently have the capability to do multiple codegen units in parallel. Unfortunately, one drawback of using this functionality is that using multiple codegen units loses optimization opportunities, like inlining, between the units. src: https://internals.rust-lang.org/t/towards-a-second-edition-of-the-compiler/5582
#codegen-units = 0 #faster XXX: doesn't seem faster! so useless

# Sets the number of codegen units to build the standard library with,
# regardless of what the codegen-unit setting for the rest of the compiler is.
#codegen-units-std = 1
codegen-units-std = 1
#codegen-units-std = 5 #faster XXX: doesn't seem faster!
#XXX error: Value for codegen units must be a positive nonzero integer


# Whether or not debug assertions are enabled for the compiler and standard
# library.
#debug-assertions = false #faster
debug-assertions = true  #it's fixed! old://that delay issue isn't fixed yet (rustc -vV with RUST_BACKTRACE=1) well, let's see now: https://github.com/ianlancetaylor/libbacktrace/commit/b0d690331db157f53ac4a8bd9d4bb7513a561d90.patch  XXX also firefox crashes on startup due to "attempt to create unaligned slice" see https://gist.github.com/xftroxgpx/74a354e94a55dcc16b4e1aa2f5829695
#to test if fixed, you have to cargo new --bin newproj make it 'panic!()' then:
#$ time RUST_BACKTRACE=0 ./target/debug/newproj
#real	0m0.003s
#user	0m0.000s
#sys	0m0.004s
#$ time RUST_BACKTRACE=1 ./target/debug/panic
#real	0m0.096s
#user	0m0.092s
#sys	0m0.003s
#^ yeah I call this fixed! :)


# Whether or not debuginfo is emitted
#debuginfo = false #faster
debuginfo = true #doesn't happen anymore!old:://nope//this might be why this happens: "LLVM ERROR: Broken module found, compilation aborted! (fragment covers entire variable)" https://github.com/rust-lang/rust/issues/48910 //this alone isn't it, but debuginfo-lines=true is it, unsure atm if debuginfo has to also be false! assuming yes

# Whether or not line number debug information is emitted
#debuginfo-lines = false #faster
debuginfo-lines = true #not anymore! old://ok this triggers "fragment covers entire variable", thanks to michaelwoerister for telling me!

# Whether or not to only build debuginfo for the standard library if enabled.
# If enabled, this will not compile the compiler with debuginfo, just the
# standard library.
debuginfo-only-std = false
#debuginfo-only-std = true

# Enable debuginfo for the extended tools: cargo, rls, rustfmt
# Adding debuginfo makes them several times larger.
debuginfo-tools = false #faster
#debuginfo-tools = true

# Whether or not `panic!`s generate backtraces (RUST_BACKTRACE)
#backtrace = true
backtrace = true

# Whether to always use incremental compilation when building rustc
#incremental = false
incremental = true #faster

# Build rustc with experimental parallelization
#experimental-parallel-queries = false
experimental-parallel-queries = true #fixed://rust-clippy fails to build: https://github.com/rust-lang/rust-clippy/issues/3613 until this is merged https://github.com/rust-lang/rust/pull/57308 got fixed!

# The default linker that will be hard-coded into the generated compiler for
# targets that don't specify linker explicitly in their target specifications.
# Note that this is not the linker used to link said compiler.
#default-linker = "cc"

# The "channel" for the Rust build to produce. The stable/beta channels only
# allow using stable features, whereas the nightly and dev channels allow using
# nightly features
#channel = "dev"
channel = "dev"

# By default the `rustc` executable is built with `-Wl,-rpath` flags on Unix
# platforms to ensure that the compiler is usable by default from the build
# directory (as it links to a number of dynamic libraries). This may not be
# desired in distributions, for example.
#rpath = true
rpath = true

# Emits extraneous output from tests to ensure that failures of the test
# harness are debuggable just from logfiles.
#verbose-tests = false
verbose-tests = false

# Flag indicating whether tests are compiled with optimizations (the -O flag) or
# with debuginfo (the -g flag)
optimize-tests = true
debuginfo-tests = true
#I'm pretty sure that setting false to optimize here is probably a bad idea as the tests will take longer to run:
#optimize-tests = false
#debuginfo-tests = false

# Flag indicating whether codegen tests will be run or not. If you get an error
# saying that the FileCheck executable is missing, you may want to disable this.
# Also see the target's llvm-filecheck option.
#codegen-tests = true
codegen-tests = false #for no good reason; just to be sure

# Flag indicating whether git info will be retrieved from .git automatically.
# Having the git information can cause a lot of rebuilds during development.
# Note: If this attribute is not explicitly set (e.g. if left commented out) it
# will default to true if channel = "dev", but will default to false otherwise.
#ignore-git = true #fast XXX: undo; well I dno if fast or not, but 34mins later I keep getting some error saying `cc` is newer and should be compiled with current rustc, so I've to git clean -dfx and maybe even remove ~/.cargo/registry/ dir and restart rustc compilation to avoid it. I'm not sure if this ignore-git=true setting is causing it! or if this too; or maybe something else entirely? maybe incremental? but it didn't so far. I don't even know. But to be sure I'll be defaulting ignore-git=false so that rustc -V also shows me the commit!
ignore-git = false #i don't get it! what git information and why will it cause rebuilds?

# When creating source tarballs whether or not to create a source tarball.
#dist-src = false

# Whether to also run the Miri tests suite when running tests.
# As a side-effect also generates MIR for all libraries.
#test-miri = false

# After building or testing extended tools (e.g. clippy and rustfmt), append the
# result (broken, compiling, testing) into this JSON file.
#save-toolstates = "/path/to/toolstates.json"

# This is an array of the codegen backends that will be compiled for the rustc
# that's being compiled. The default is to only build the LLVM codegen backend,
# but you can also optionally enable the "emscripten" backend for asm.js or
# make this an empty array (but that probably won't get too far in the
# bootstrap)
#codegen-backends = ["llvm"]

# This is the name of the directory in which codegen backends will get installed
#codegen-backends-dir = "codegen-backends"

# Flag indicating whether `libstd` calls an imported function to handle basic IO
# when targeting WebAssembly. Enable this to debug tests for the `wasm32-unknown-unknown`
# target, as without this option the test output will not be captured.
#wasm-syscall = false

# Indicates whether LLD will be compiled and made available in the sysroot for
# rustc to execute.
lld = false

# Indicates whether some LLVM tools, like llvm-objdump, will be made available in the
# sysroot.
#llvm-tools = false

# Indicates whether LLDB will be made available in the sysroot.
# This is only built if LLVM is also being built.
#lldb = false


# Whether to deny warnings in crates
#deny-warnings = true
deny-warnings = true

# Print backtrace on internal compiler errors during bootstrap
#backtrace-on-ice = false
backtrace-on-ice = true

# Whether to verify generated LLVM IR
#verify-llvm-ir = false
#verify-llvm-ir = true
#2 aug 2018: error: unknown debugging option: `verify-llvm-ir`

# Map all debuginfo paths for libstd and crates to `/rust/$sha/$crate/...`,
# generally only set for releases
#remap-debuginfo = false

# Link the compiler against `jemalloc`, where on Linux and OSX it should
# override the default allocator for rustc and LLVM.
#jemalloc = false

# Run tests in various test suites with the "nll compare mode" in addition to
# running the tests in normal mode. Largely only used on CI and during local
# development of NLL
#test-compare-mode = false



# =============================================================================
# Options for specific targets
#
# Each of the following options is scoped to the specific target triple in
# question and is used for determining how to compile each target.
# =============================================================================
[target.x86_64-unknown-linux-gnu]

# C compiler to be used to compiler C code. Note that the
# default value is platform specific, and if not specified it may also depend on
# what platform is crossing to what platform.
#cc = "cc"

# C++ compiler to be used to compiler C++ code (e.g. LLVM and our LLVM shims).
# This is only used for host targets.
#cxx = "c++"

# Archiver to be used to assemble static libraries compiled from C/C++ code.
# Note: an absolute path should be used, otherwise LLVM build will break.
#ar = "ar"

# Ranlib to be used to assemble static libraries compiled from C/C++ code.
# Note: an absolute path should be used, otherwise LLVM build will break.
#ranlib = "ranlib"


# Linker to be used to link Rust code. Note that the
# default value is platform specific, and if not specified it may also depend on
# what platform is crossing to what platform.
#linker = "cc"

# Path to the `llvm-config` binary of the installation of a custom LLVM to link
# against. Note that if this is specified we don't compile LLVM at all for this
# target.
#llvm-config = "../path/to/llvm/root/bin/llvm-config"

# Normally the build system can find LLVM's FileCheck utility, but if
# not, you can specify an explicit file name for it.
#llvm-filecheck = "/path/to/FileCheck"

# If this target is for Android, this option will be required to specify where
# the NDK for the target lives. This is used to find the C compiler to link and
# build native code.
#android-ndk = "/path/to/ndk"

# Force static or dynamic linkage of the standard library for this target. If
# this target is a host for rustc, this will also affect the linkage of the
# compiler itself. This is useful for building rustc on targets that normally
# only use static libraries. If unset, the target's default linkage is used.
#crt-static = false

# The root location of the MUSL installation directory. The library directory
# will also need to contain libunwind.a for an unwinding implementation. Note
# that this option only makes sense for MUSL targets that produce statically
# linked binaries
#musl-root = "..."

# Used in testing for configuring where the QEMU images are located, you
# probably don't want to use this.
#qemu-rootfs = "..."

# =============================================================================
# Distribution options
#
# These options are related to distribution, mostly for the Rust project itself.
# You probably won't need to concern yourself with any of these options
# =============================================================================
[dist]

# This is the folder of artifacts that the build system will sign. All files in
# this directory will be signed with the default gpg key using the system `gpg`
# binary. The `asc` and `sha256` files will all be output into the standard dist
# output folder (currently `build/dist`)
#
# This folder should be populated ahead of time before the build system is
# invoked.
#sign-folder = "path/to/folder/to/sign"

# This is a file which contains the password of the default gpg key. This will
# be passed to `gpg` down the road when signing all files in `sign-folder`
# above. This should be stored in plaintext.
#gpg-password-file = "path/to/gpg/password"

# The remote address that all artifacts will eventually be uploaded to. The
# build system generates manifests which will point to these urls, and for the
# manifests to be correct they'll have to have the right URLs encoded.
#
# Note that this address should not contain a trailing slash as file names will
# be appended to it.
#upload-addr = "https://example.com/folder"

# Whether to build a plain source tarball to upload
# We disable that on Windows not to override the one already uploaded on S3
# as the one built on Windows will contain backslashes in paths causing problems
# on linux
#src-tarball = true
src-tarball = false
#

# Whether to allow failures when building tools
#missing-tools = false
missing-tools = false

