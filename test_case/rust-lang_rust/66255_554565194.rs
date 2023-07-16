plain
2019-11-15T23:08:33.9685007Z 
2019-11-15T23:08:33.9685055Z 
2019-11-15T23:08:33.9685085Z 
2019-11-15T23:08:33.9685113Z 
2019-11-15T23:08:33.9685246Z - Add the override in cc_detect for riscv (or any "no-C" platform like wasm32 and nvptx)
2019-11-15T23:08:33.9685770Z - Install and use the appropriate c compiler. I tried this the `g++-riscv64-linux-gnu` package, but it failed missing some header file.
2019-11-15T23:08:33.9686054Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-15T23:08:33.9686139Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-15T23:08:33.9686244Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-15T23:08:33.9686316Z AGENT_ID=514
---
2019-11-15T23:08:33.9713345Z TMP=/tmp
2019-11-15T23:08:33.9713413Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-15T23:08:33.9713522Z TOOLSTATE_PUBLISH=1
2019-11-15T23:08:33.9713638Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-15T23:08:33.9713868Z The main thorn is that `cc` gained knowledge about RISC-V architectures (https://github.com/alexcrichton/cc-rs/pull/428, https://github.com/alexcrichton/cc-rs/pull/429, https://github.com/alexcrichton/cc-rs/pull/430), but the builders on CI do not have the riscv C compiler installed. This means that bootstraps' cc detection was finding a C compiler that isn't installed, and fails.
2019-11-15T23:08:33.9714158Z The solution here is to override the cc detection to `false`. The C compiler isn't actually used on riscv platforms. AFAIK, the only location would be compiler_builtins, and it currently forces C support off (https://github.com/rust-lang/compiler-builtins/blob/a533ae9c5aa325db209659679535fe1f186eae81/build.rs#L49-L55).
2019-11-15T23:08:33.9714413Z This updates the `cc` crate, bringing in better parallel building support. Also updates `git2` which enables the parallel feature. (Note: I don't expect it will have a significant impact on build time, but seems good to update anyways.)
2019-11-15T23:08:33.9724118Z USERDOMAIN=fv-az379
2019-11-15T23:08:33.9724197Z USERDOMAIN_ROAMINGPROFILE=fv-az379
2019-11-15T23:08:33.9724287Z USERNAME=VssAdministrator
2019-11-15T23:08:33.9724350Z USERPROFILE=C:\Users\VssAdministrator
2019-11-15T23:08:33.9724350Z USERPROFILE=C:\Users\VssAdministrator
2019-11-15T23:08:33.9724427Z Update cc, git2, num_cpus.
2019-11-15T23:08:33.9724582Z VS140COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\Tools\
2019-11-15T23:08:33.9724655Z VSTS_AGENT_PERFLOG=c:\vsts\perflog
2019-11-15T23:08:33.9724747Z VSTS_PROCESS_LOOKUP_ID=vsts_23d78c07-fce6-46fc-835f-3b1040ac804e
2019-11-15T23:08:33.9724831Z WINDIR=C:\windows
---
2019-11-15T23:11:45.2850045Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-15T23:11:45.2850505Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-15T23:11:45.2855201Z 
2019-11-15T23:11:45.2861134Z Failures
2019-11-15T23:11:45.2868479Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-15T23:11:45.7454841Z 
2019-11-15T23:11:45.7454841Z 
2019-11-15T23:11:45.7553379Z ##[error]Bash exited with code '1'.
2019-11-15T23:11:45.7740963Z ##[section]Starting: Checkout
2019-11-15T23:11:45.7867422Z ==============================================================================
2019-11-15T23:11:45.7867546Z Task         : Get sources
2019-11-15T23:11:45.7867643Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
