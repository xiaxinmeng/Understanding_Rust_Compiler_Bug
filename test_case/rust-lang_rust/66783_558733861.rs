plain
2019-11-26T16:54:47.1796050Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-26T16:54:47.1796225Z 
2019-11-26T16:54:47.1796434Z   git checkout -b <new-branch-name>
2019-11-26T16:54:47.1797386Z 
2019-11-26T16:54:47.1797724Z HEAD is now at 74cddf52e Auto merge of #66783 - Mark-Simulacrum:par, r=<try>
2019-11-26T16:54:47.2141053Z ##[section]Starting: Setup environment
2019-11-26T16:54:47.2241017Z ==============================================================================
2019-11-26T16:54:47.2241115Z Task         : Bash
2019-11-26T16:54:47.2241318Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-26T16:54:49.6762304Z BUILD_SOURCEBRANCHNAME=try
2019-11-26T16:54:49.6762484Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-26T16:54:49.6762681Z BUILD_SOURCEVERSION=74cddf52e79357176c39070b4535ca7a42cac16e
2019-11-26T16:54:49.6762858Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-26T16:54:49.6763071Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66783 - Mark-Simulacrum:par, r=<try>
2019-11-26T16:54:49.6763441Z CI_JOB_NAME=dist-x86_64-msvc
2019-11-26T16:54:49.6763622Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-26T16:54:49.6763798Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-26T16:54:49.6763993Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-26T16:54:49.6797396Z VSTS_AGENT_PERFLOG=c:\vsts\perflog
2019-11-26T16:54:49.6797988Z VSTS_PROCESS_LOOKUP_ID=vsts_1068dfe5-8c16-430c-8541-a52bf8c4229d
2019-11-26T16:54:49.6798371Z WINDIR=C:\windows
2019-11-26T16:54:49.6798550Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-26T16:54:49.6798713Z [do not merge] posix semaphores over make jobserver
2019-11-26T16:54:49.6799033Z agent.jobstatus=Succeeded
2019-11-26T16:54:49.6799176Z r? @ghost
2019-11-26T16:54:49.6799302Z 
2019-11-26T16:54:49.6799437Z disk usage:
---
2019-11-26T17:04:10.2978001Z downloading https://static.rust-lang.org/dist/2019-11-06/rustc-beta-x86_64-pc-windows-msvc.tar.gz
2019-11-26T17:04:17.5683087Z extracting D:\a\1\s\build\cache\2019-11-06\rustc-beta-x86_64-pc-windows-msvc.tar.gz
2019-11-26T17:04:17.9423849Z downloading https://static.rust-lang.org/dist/2019-11-06/cargo-beta-x86_64-pc-windows-msvc.tar.gz
2019-11-26T17:04:19.3386838Z extracting D:\a\1\s\build\cache\2019-11-06\cargo-beta-x86_64-pc-windows-msvc.tar.gz
2019-11-26T17:04:19.7812772Z     Updating git repository `https://github.com/Mark-Simulacrum/jobserver-rs`
2019-11-26T17:04:20.3773491Z     Updating git repository `https://github.com/spastorino/rustc-rayon`
---
2019-11-26T17:23:06.7059134Z    Compiling byteorder v1.3.2
2019-11-26T17:23:07.1981672Z    Compiling nodrop v0.1.12
2019-11-26T17:23:07.2999608Z    Compiling log v0.4.8
2019-11-26T17:23:08.3065345Z    Compiling proc-macro2 v1.0.3
2019-11-26T17:23:09.2111215Z    Compiling rustc-rayon-core v0.3.0 (https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#dd85f978)
2019-11-26T17:23:10.2973544Z    Compiling bitflags v1.2.1
2019-11-26T17:23:10.5241475Z    Compiling unicode-xid v0.2.0
2019-11-26T17:23:10.9458797Z    Compiling either v1.5.0
2019-11-26T17:23:11.2172059Z    Compiling syn v1.0.5
2019-11-26T17:23:11.2172059Z    Compiling syn v1.0.5
2019-11-26T17:23:11.9988534Z    Compiling smallvec v0.6.10
2019-11-26T17:23:12.5424723Z    Compiling indexmap v1.0.2
2019-11-26T17:23:13.3749080Z    Compiling stable_deref_trait v1.1.0
2019-11-26T17:23:13.6330855Z    Compiling jobserver v0.1.17 (https://github.com/Mark-Simulacrum/jobserver-rs?branch=sem#3ec169d9)
2019-11-26T17:23:13.6917774Z error[E0463]: can't find crate for `getrandom`
2019-11-26T17:23:13.6959345Z    --> C:\Users\VssAdministrator\.cargo\git\checkouts\jobserver-rs-3952e9071bf95b82\3ec169d\src\lib.rs:689:5
2019-11-26T17:23:13.6959681Z 689 |     extern crate getrandom;
2019-11-26T17:23:13.6959895Z     |     ^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-26T17:23:13.6959969Z 
2019-11-26T17:23:13.6960103Z error: aborting due to previous error
---
2019-11-26T17:23:13.8461291Z   local time: Tue Nov 26 17:23:13 CUT 2019
2019-11-26T17:23:13.9663125Z   network time: Tue, 26 Nov 2019 17:23:13 GMT
2019-11-26T17:23:13.9684792Z == end clock drift check ==
2019-11-26T17:23:13.9712504Z 
2019-11-26T17:23:14.0534938Z ##[error]Bash exited with code '1'.
2019-11-26T17:23:14.0653199Z ##[section]Starting: Checkout
2019-11-26T17:23:14.0758869Z ==============================================================================
2019-11-26T17:23:14.0759288Z Task         : Get sources
2019-11-26T17:23:14.0759382Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
