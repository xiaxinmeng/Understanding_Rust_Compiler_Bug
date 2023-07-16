plain
 ---> 2afb3e7bef8f
Step 3/8 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   gcc-multilib   make   ninja-build   file   curl   ca-certificates   python2.7   python3.9   git   cmake   sudo   gdb   llvm-12-tools   llvm-12-dev   libedit-dev   libssl-dev   pkg-config   zlib1g-dev   xz-utils   nodejs
 ---> Using cache
 ---> 357fae1e02d2
Step 4/8 : RUN apt-get update &&     apt-get install -y apt-transport-https software-properties-common &&     curl -s "https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb" > packages-microsoft-prod.deb &&     dpkg -i packages-microsoft-prod.deb &&     apt-get update &&     apt-get install -y powershell
 ---> 080d1843107f
Step 5/8 : COPY scripts/sccache.sh /scripts/
 ---> Using cache
 ---> 4a6c76c56ba3
---
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_privacy/src/lib.rs:927:27
    |
927 |             self.tcx.sess.span_err(span, access_level);
    |
note: the lint level is defined here
   --> compiler/rustc_privacy/src/lib.rs:9:34
    |
    |
9   | #![cfg_attr(not(bootstrap), deny(rustc::diagnostic_outside_of_impl))]

error: could not compile `rustc_privacy` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:07:56
