diff
--- config.example.toml	2023-03-16 13:10:23.060225944 +0100
+++ config.toml	2023-03-20 21:06:44.968264877 +0100
@@ -10,7 +10,7 @@
 # system.

 # Keeps track of the last version of `x.py` used.
-# If it does not match the version that is currently running,
+# If it does not match them version that is currently running,
 # `x.py` will prompt you to update it and read the changelog.
 # See `src/bootstrap/CHANGELOG.md` for more information.
 changelog-seen = 2
@@ -35,6 +35,9 @@
 # Unless you're developing for a target where Rust CI doesn't build a compiler
 # toolchain or changing LLVM locally, you probably want to set this to true.
 #
+# This is false by default so that distributions don't unexpectedly download
+# LLVM from the internet.
+#
 # All tier 1 targets are currently supported; set this to `"if-available"` if
 # you are not sure whether you're on a tier 1 target.
 #
@@ -42,9 +45,13 @@
 #
 # Note that many of the LLVM options are not currently supported for
 # downloading. Currently only the "assertions" option can be toggled.
-#
-# Defaults to "if-available" when `channel = "dev"` and "false" otherwise.
-#download-ci-llvm = "if-available"
+download-ci-llvm = false
+
+# Indicates whether LLVM rebuild should be skipped when running bootstrap. If
+# this is `false` then the compiler's LLVM will be rebuilt whenever the built
+# version doesn't have the correct hash. If it is `true` then LLVM will never
+# be rebuilt. The default value is `false`.
+#skip-rebuild = false

 # Indicates whether the LLVM build is a Release or Debug build
 #optimize = true
@@ -53,13 +60,13 @@
 # only succeed if you use clang, lld, llvm-ar, and llvm-ranlib in your C/C++
 # toolchain (see the `cc`, `cxx`, `linker`, `ar`, and `ranlib` options below).
 # More info at: https://clang.llvm.org/docs/ThinLTO.html#clang-bootstrap
-#thin-lto = false
+thin-lto = true

 # Indicates whether an LLVM Release build should include debug info
 #release-debuginfo = false

 # Indicates whether the LLVM assertions are enabled or not
-#assertions = false
+assertions = true

 # Indicates whether the LLVM testsuite is enabled in the build or not. Does
 # not execute the tests as part of the build as part of x.py build et al,
@@ -71,17 +78,21 @@
 #plugins = false

 # Indicates whether ccache is used when building LLVM
-#ccache = false
+ccache = "/home/matthias/.cargo/bin/sccache"
 # or alternatively ...
 #ccache = "/path/to/ccache"

-# When true, link libstdc++ statically into the rustc_llvm.
-# This is useful if you don't want to use the dynamic version of that
-# library provided by LLVM.
+# If an external LLVM root is specified, we automatically check the version by
+# default to make sure it's within the range that we're expecting, but setting
+# this flag will indicate that this version check should not be done.
+#version-check = true
+
+# Link libstdc++ statically into the rustc_llvm instead of relying on a
+# dynamic version to be available.
 #static-libstdcpp = false

 # Whether to use Ninja to build LLVM. This runs much faster than make.
-#ninja = true
+ninja = true

 # LLVM targets to build support for.
 # Note: this is NOT related to Rust compilation targets. However, as Rust is
@@ -92,7 +103,7 @@
 # support. You'll need to write a target specification at least, and most
 # likely, teach rustc about the C ABI of the target. Get in touch with the
 # Rust team and file an issue if you need assistance in porting!
-#targets = "AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86"
+targets = "X86"

 # LLVM experimental targets to build support for. These targets are specified in
 # the same format as above, but since these targets are experimental, they are
@@ -106,7 +117,7 @@
 # each linker process.
 # If absent or 0, linker invocations are treated like any other job and
 # controlled by rustbuild's -j parameter.
-#link-jobs = 0
+link-jobs = 2

 # When invoking `llvm-config` this configures whether the `--shared` argument is
 # passed to prefer linking to shared libraries.
@@ -124,9 +135,9 @@
 #clang-cl = cc

 # Pass extra compiler and linker flags to the LLVM CMake build.
-#cflags = ""
-#cxxflags = ""
-#ldflags = ""
+cflags = "-march=native" # -D_GLIBCXX_DEBUG
+cxxflags = "-march=native" # -D_GLIBCXX_DEBUG
+ldflags = "-march=native" # -D_GLIBCXX_DEBUG

 # Use libc++ when building LLVM instead of libstdc++. This is the default on
 # platforms already use libc++ as the default C++ library, but this option
@@ -141,22 +152,15 @@
 #allow-old-toolchain = false

 # Whether to include the Polly optimizer.
-#polly = false
+polly = true

 # Whether to build the clang compiler.
 #clang = false

-# Whether to enable llvm compilation warnings.
-#enable-warnings = false
-
-# Custom CMake defines to set when building LLVM.
-#build-config = {}
-
 # =============================================================================
 # General build configuration options
 # =============================================================================
 [build]
-
 # The default stage to use for the `check` subcommand
 #check-stage = 0

@@ -170,10 +174,10 @@
 #test-stage = 1

 # The default stage to use for the `dist` subcommand
-#dist-stage = 2
+dist-stage = 2

 # The default stage to use for the `install` subcommand
-#install-stage = 2
+install-stage = 2

 # The default stage to use for the `bench` subcommand
 #bench-stage = 2
@@ -226,9 +230,6 @@
 # and generated in already-minified form from the beginning.
 #docs-minification = true

-# Flag to specify whether private items should be included in the library docs.
-#library-docs-private-items = false
-
 # Indicate whether the compiler should be documented in addition to the standard
 # library and facade crates.
 #compiler-docs = false
@@ -236,6 +237,10 @@
 # Indicate whether git submodules are managed and updated automatically.
 #submodules = true

+# Update git submodules only when the checked out commit in the submodules differs
+# from what is committed in the main rustc repo.
+#fast-submodules = true
+
 # The path to (or name of) the GDB executable to use. This is only used for
 # executing the debuginfo test suite.
 #gdb = "gdb"
@@ -250,16 +255,6 @@
 # Defaults to the Python interpreter used to execute x.py
 #python = "python"

-# The path to the REUSE executable to use. Note that REUSE is not required in
-# most cases, as our tooling relies on a cached (and shrinked) copy of the
-# REUSE output present in the git repository and in our source tarballs.
-#
-# REUSE is only needed if your changes caused the overral licensing of the
-# repository to change, and the cached copy has to be regenerated.
-#
-# Defaults to the "reuse" command in the system path.
-#reuse = "reuse"
-
 # Force Cargo to check that Cargo.lock describes the precise dependency
 # set that all the Cargo.toml files create, instead of updating it.
 #locked-deps = false
@@ -279,26 +274,13 @@
 # which are used to install Rust and Cargo together. This is disabled by
 # default. The `tools` option (immediately below) specifies which tools should
 # be built if `extended = true`.
-#extended = false
+extended = true

-# Set of tools to be included in the installation.
-#
-# If `extended = false`, the only one of these built by default is rustdoc.
-#
-# If `extended = true`, they're all included, with the exception of
-# rust-demangler which additionally requires `profiler = true` to be set.
-#
-# If any enabled tool fails to build, the installation fails.
-#tools = [
-#    "cargo",
-#    "clippy",
-#    "rustdoc",
-#    "rustfmt",
-#    "rust-analyzer",
-#    "analysis",
-#    "src",
-#    "rust-demangler",  # if profiler = true
-#]
+# Installs chosen set of extended tools if `extended = true`. By default builds
+# all extended tools except `rust-demangler`, unless the target is also being
+# built with `profiler = true`. If chosen tool failed to build the installation
+# fails. If `extended = false`, this option is ignored.
+tools = ["cargo", "clippy", "rustfmt", "analysis", "src", "miri"] # + "rust-demangler" if `profiler`

 # Verbosity level: 0 == not verbose, 1 == verbose, 2 == very verbose
 #verbose = 0
@@ -316,7 +298,7 @@

 # Run the build with low priority, by setting the process group's "nice" value
 # to +10 on Unix platforms, and by using a "low priority" job object on Windows.
-#low-priority = false
+low-priority = true

 # Arguments passed to the `./configure` script, used during distcheck. You
 # probably won't fill this in but rather it's filled in by the `./configure`
@@ -343,23 +325,17 @@
 # a Nix toolchain on non-NixOS distributions.
 #patch-binaries-for-nix = false

-# Collect information and statistics about the current build and writes it to
-# disk. Enabling this or not has no impact on the resulting build output. The
-# schema of the file generated by the build metrics feature is unstable, and
-# this is not intended to be used during local development.
-#metrics = false
-
 # =============================================================================
 # General install configuration options
 # =============================================================================
 [install]

 # Instead of installing to /usr/local, install to this path instead.
-#prefix = "/usr/local"
+prefix = "/home/matthias/.rustup/toolchains/local-debug-assertions"

 # Where to install system configuration files
 # If this is a relative path, it will get installed in `prefix` above
-#sysconfdir = "/etc"
+sysconfdir = "etc"

 # Where to install documentation in `prefix` above
 #docdir = "share/doc/rust"
@@ -408,7 +384,7 @@
 #       "maximally debuggable" environment (notably libstd) takes
 #       hours to build.
 #
-#debug = false
+debug = true

 # Whether to download the stage 1 and 2 compilers from CI.
 # This is mostly useful for tools; if you have changes to `compiler/` they will be ignored.
@@ -435,13 +411,13 @@
 # binary, otherwise they are omitted.
 #
 # Defaults to rust.debug value
-#debug-assertions = rust.debug (boolean)
+debug-assertions = true

 # Whether or not debug assertions are enabled for the standard library.
 # Overrides the `debug-assertions` option, if defined.
 #
 # Defaults to rust.debug-assertions value
-#debug-assertions-std = rust.debug-assertions (boolean)
+debug-assertions-std = true

 # Whether or not to leave debug! and trace! calls in the rust binary.
 # Overrides the `debug-assertions` option, if defined.
@@ -457,13 +433,13 @@
 # library.
 #
 # Defaults to rust.debug value
-#overflow-checks = rust.debug (boolean)
+overflow-checks = true

 # Whether or not overflow checks are enabled for the standard library.
 # Overrides the `overflow-checks` option, if defined.
 #
 # Defaults to rust.overflow-checks value
-#overflow-checks-std = rust.overflow-checks (boolean)
+overflow-checks-std = true

 # Debuginfo level for most of Rust code, corresponds to the `-C debuginfo=N` option of `rustc`.
 # `0` - no debug info
@@ -479,7 +455,7 @@
 # and will slow down the linking process significantly.
 #
 # Defaults to 1 if debug is true
-#debuginfo-level = 0
+debuginfo-level = 1

 # Debuginfo level for the compiler.
 #debuginfo-level-rustc = debuginfo-level
@@ -494,23 +470,13 @@
 # FIXME(#61117): Some tests fail when this option is enabled.
 #debuginfo-level-tests = 0

-# Should rustc be build with split debuginfo? Default is platform dependent.
-# Valid values are the same as those accepted by `-C split-debuginfo`
-# (`off`/`unpacked`/`packed`).
-#
-# On Linux, split debuginfo is disabled by default.
-#
-# On Apple platforms, unpacked split debuginfo is used by default. Unpacked
-# debuginfo does not run `dsymutil`, which packages debuginfo from disparate
-# object files into a single `.dSYM` file. `dsymutil` adds time to builds for
-# no clear benefit, and also makes it more difficult for debuggers to find
-# debug info. The compiler currently defaults to running `dsymutil` to preserve
-# its historical default, but when compiling the compiler itself, we skip it by
-# default since we know it's safe to do so in that case.
-#
-# On Windows platforms, packed debuginfo is the only supported option,
-# producing a `.pdb` file.
-#split-debuginfo = if linux { off } else if windows { packed } else if apple { unpacked }
+# Whether to run `dsymutil` on Apple platforms to gather debug info into .dSYM
+# bundles. `dsymutil` adds time to builds for no clear benefit, and also makes
+# it more difficult for debuggers to find debug info. The compiler currently
+# defaults to running `dsymutil` to preserve its historical default, but when
+# compiling the compiler itself, we skip it by default since we know it's safe
+# to do so in that case.
+#run-dsymutil = false

 # Whether or not `panic!`s generate backtraces (RUST_BACKTRACE)
 #backtrace = true
@@ -540,12 +506,6 @@
 # A descriptive string to be appended to `rustc --version` output, which is
 # also used in places like debuginfo `DW_AT_producer`. This may be useful for
 # supplementary build information, like distro-specific package versions.
-#
-# The Rust compiler will differentiate between versions of itself, including
-# based on this string, which means that if you wish to be compatible with
-# upstream Rust you need to set this to "". However, note that if you are not
-# actually compatible -- for example if you've backported patches that change
-# behavior -- this may lead to miscompilations or other bugs.
 #description = <none> (string)

 # The root location of the musl installation directory. The library directory
@@ -591,7 +551,7 @@
 # and currently the only standard options supported are `"llvm"`, `"cranelift"`
 # and `"gcc"`. The first backend in this list will be used as default by rustc
 # when no explicit backend is specified.
-#codegen-backends = ["llvm"]
+codegen-backends = ["llvm", "cranelift"]

 # Indicates whether LLD will be compiled and made available in the sysroot for
 # rustc to execute.
@@ -614,10 +574,10 @@
 #deny-warnings = true

 # Print backtrace on internal compiler errors during bootstrap
-#backtrace-on-ice = false
+backtrace-on-ice = true

 # Whether to verify generated LLVM IR
-#verify-llvm-ir = false
+verify-llvm-ir = true

 # Compile the compiler with a non-default ThinLTO import limit. This import
 # limit controls the maximum size of functions imported by ThinLTO. Decreasing
@@ -636,9 +596,16 @@
 # development of NLL
 #test-compare-mode = false

-# Global default for llvm-libunwind for all targets. See the target-specific
-# documentation for llvm-libunwind below. Note that the target-specific
-# option will override this if set.
+# Use LLVM libunwind as the implementation for Rust's unwinder.
+# Accepted values are 'in-tree' (formerly true), 'system' or 'no' (formerly false).
+# This option only applies for Linux and Fuchsia targets.
+# On Linux target, if crt-static is not enabled, 'no' means dynamic link to
+# `libgcc_s.so`, 'in-tree' means static link to the in-tree build of llvm libunwind
+# and 'system' means dynamic link to `libunwind.so`. If crt-static is enabled,
+# the behavior is depend on the libc. On musl target, 'no' and 'in-tree' both
+# means static link to the in-tree build of llvm libunwind, and 'system' means
+# static link to `libunwind.a` provided by system. Due to the limitation of glibc,
+# it must link to `libgcc_eh.a` to get a working output, and this option have no effect.
 #llvm-libunwind = 'no'

 # Enable Windows Control Flow Guard checks in the standard library.
@@ -653,18 +620,8 @@
 # If an explicit setting is given, it will be used for all parts of the codebase.
 #new-symbol-mangling = true|false (see comment)

-# Select LTO mode that will be used for compiling rustc. By default, thin local LTO
-# (LTO within a single crate) is used (like for any Rust crate). You can also select
-# "thin" or "fat" to apply Thin/Fat LTO to the `rustc_driver` dylib, or "off" to disable
-# LTO entirely.
-#lto = "thin-local"
-
-# Build compiler with the optimization enabled and -Zvalidate-mir, currently only for `std`
-#validate-mir-opts = 3
-
-# Copy the linker, DLLs, and various libraries from MinGW into the rustc toolchain.
-# Only applies when the host or target is pc-windows-gnu.
-#include-mingw-linker = true
+# disabled until https://github.com/rust-lang/rust/issues/105637 is fixed
+lto = "thin"

 # =============================================================================
 # Options for specific targets
@@ -704,26 +661,10 @@
 # target.
 #llvm-config = <none> (path)

-# Override detection of whether this is a Rust-patched LLVM. This would be used
-# in conjunction with either an llvm-config or build.submodules = false.
-#llvm-has-rust-patches = if llvm-config { false } else { true }
-
 # Normally the build system can find LLVM's FileCheck utility, but if
 # not, you can specify an explicit file name for it.
 #llvm-filecheck = "/path/to/llvm-version/bin/FileCheck"

-# Use LLVM libunwind as the implementation for Rust's unwinder.
-# Accepted values are 'in-tree' (formerly true), 'system' or 'no' (formerly false).
-# This option only applies for Linux and Fuchsia targets.
-# On Linux target, if crt-static is not enabled, 'no' means dynamic link to
-# `libgcc_s.so`, 'in-tree' means static link to the in-tree build of llvm libunwind
-# and 'system' means dynamic link to `libunwind.so`. If crt-static is enabled,
-# the behavior is depend on the libc. On musl target, 'no' and 'in-tree' both
-# means static link to the in-tree build of llvm libunwind, and 'system' means
-# static link to `libunwind.a` provided by system. Due to the limitation of glibc,
-# it must link to `libgcc_eh.a` to get a working output, and this option have no effect.
-#llvm-libunwind = 'no' if Linux, 'in-tree' if Fuchsia
-
 # If this target is for Android, this option will be required to specify where
 # the NDK for the target lives. This is used to find the C compiler to link and
 # build native code.
@@ -763,10 +704,6 @@
 # probably don't want to use this.
 #qemu-rootfs = <none> (path)

-# Skip building the `std` library for this target. Enabled by default for
-# target triples containing `-none`, `nvptx`, `switch`, or `-uefi`.
-#no-std = <platform-specific> (bool)
-
 # =============================================================================
 # Distribution options
 #
@@ -805,4 +742,4 @@
 # formats is provided to rust-installer, which must support all of them.
 #
 # This list must be non-empty.
-#compression-formats = ["gz", "xz"]
+compression-formats = ["xz"]
