plain
2019-11-30T15:59:19.9749246Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T15:59:19.9749463Z 
2019-11-30T15:59:19.9749673Z   git checkout -b <new-branch-name>
2019-11-30T15:59:19.9749825Z 
2019-11-30T15:59:19.9750045Z HEAD is now at 79060be03 Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T15:59:20.0200649Z ##[section]Starting: Setup environment
2019-11-30T15:59:20.0305037Z ==============================================================================
2019-11-30T15:59:20.0305132Z Task         : Bash
2019-11-30T15:59:20.0305324Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T15:59:21.6881031Z 
2019-11-30T15:59:21.6881174Z 
2019-11-30T15:59:21.6881333Z 
2019-11-30T15:59:21.6881538Z  - #66612 (Initial implementation of or-pattern usefulness checking)
2019-11-30T15:59:21.6881744Z  - #66705 (Atomic as_mut_ptr)
2019-11-30T15:59:21.6882046Z  - #66759 (impl TrustedLen for vec::Drain)
2019-11-30T15:59:21.6882282Z  - #66858 (Use LLVMAddAnalysisPasses instead of Rust's wrapper)
2019-11-30T15:59:21.6882680Z  - #66870 (SimplifyArmIdentity only for locals with the same type)
2019-11-30T15:59:21.6882905Z  - #66883 (rustc_typeck: gate AnonConst's generics on feature(const_generics).)
2019-11-30T15:59:21.6883152Z  - #66889 (Make python-generated source files compatible with rustfmt)
2019-11-30T15:59:21.6883365Z  - #66894 (Remove unneeded prelude imports in libcore tests)
2019-11-30T15:59:21.6890953Z  - #66895 (Feature gating *declarations* => new crate `rustc_feature`)
2019-11-30T15:59:21.6891630Z AGENT_BUILDDIRECTORY=D:\a\1
2019-11-30T15:59:21.6891810Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T15:59:21.6892007Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T15:59:21.6892179Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
---
2019-11-30T15:59:21.6908915Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T15:59:21.6908972Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T15:59:21.6909056Z BUILD_SOURCEVERSION=79060be03560ef58578a750cf8a69666d89e3a9b
2019-11-30T15:59:21.6909135Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T15:59:21.6909397Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T15:59:21.6912384Z CI_JOB_NAME=i686-msvc-1
2019-11-30T15:59:21.6912475Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T15:59:21.6912558Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T15:59:21.6912664Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T16:00:59.9525097Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T16:00:59.9525296Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T16:00:59.9528880Z 
2019-11-30T16:00:59.9533578Z Failures
2019-11-30T16:00:59.9539965Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T16:00:59.9540228Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T16:01:00.4178608Z 
2019-11-30T16:01:00.4261819Z ##[error]Bash exited with code '1'.
2019-11-30T16:01:00.4408865Z ##[section]Starting: Checkout
2019-11-30T16:01:00.4522445Z ==============================================================================
2019-11-30T16:01:00.4522547Z Task         : Get sources
2019-11-30T16:01:00.4522649Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
