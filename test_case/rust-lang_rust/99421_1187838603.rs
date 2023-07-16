plain
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /checkout/src/tools/miri/Cargo.toml
workspace: /checkout/Cargo.toml
    Updating git repository `https://github.com/Bryanskiy/libc.git`
---
  Downloaded object v0.26.2
  Downloaded gimli v0.25.0
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
  Downloaded serde v1.0.125
   Compiling proc-macro2 v1.0.37
   Compiling syn v1.0.91
   Compiling unicode-xid v0.2.2
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling autocfg v1.1.0
   Compiling proc-macro-hack v0.5.19
   Compiling tinystr v0.3.4
   Compiling version_check v0.9.3
---
   Compiling syn v1.0.91
   Compiling unicode-xid v0.2.2
   Compiling memchr v2.4.1
   Compiling version_check v0.9.3
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling serde v1.0.125
    Checking cfg-if v1.0.0
   Compiling autocfg v1.1.0
    Checking lazy_static v1.4.0
---
  Downloaded proc-macro-error-attr v1.0.4
  Downloaded proc-macro-error v1.0.4
  Downloaded bytes v1.0.1
   Compiling syn v1.0.91
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling serde_json v1.0.59
    Checking matches v0.1.8
    Checking tinyvec v0.3.4
    Checking percent-encoding v2.1.0
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
package:   /checkout/src/tools/miri/Cargo.toml
workspace: /checkout/Cargo.toml
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
   Compiling rustc-hash v1.1.0
   Compiling parking_lot_core v0.8.5
   Compiling bitflags v1.2.1
    Checking scopeguard v1.1.0
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling self_cell v0.10.2
   Compiling annotate-snippets v0.8.0
   Compiling typenum v1.12.0
    Checking ppv-lite86 v0.2.8
---
    Checking matches v0.1.8
    Checking tinyvec v0.3.4
   Compiling pkg-config v0.3.18
    Checking percent-encoding v2.1.0
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling camino v1.0.5
   Compiling semver v1.0.3
   Compiling futures-task v0.3.19
   Compiling windows_i686_gnu v0.36.1
---
package:   /checkout/src/tools/miri/Cargo.toml
workspace: /checkout/Cargo.toml
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
  Downloaded colored v2.0.0
   Compiling proc-macro2 v1.0.37
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.91
   Compiling libc v0.2.126 (https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c)
   Compiling lazy_static v1.4.0
   Compiling memchr v2.4.1
   Compiling autocfg v1.1.0
   Compiling serde_derive v1.0.125
---
   Compiling regex v1.5.5
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 7.35s
tidy check
tidy error: invalid source: "git+https://github.com/Bryanskiy/libc.git?branch=android-crt-static#64d47e8c4bb23f20befe95b9da3a45769024487a"
Checking which error codes lack tests...
* 631 error codes
* highest error code: E0789
Found 506 error codes
