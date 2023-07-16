plain
2019-11-30T16:30:19.1936350Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T16:30:19.1936414Z 
2019-11-30T16:30:19.1936940Z   git checkout -b <new-branch-name>
2019-11-30T16:30:19.1937589Z 
2019-11-30T16:30:19.1939165Z HEAD is now at 495606319 Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T16:30:19.2316385Z ##[section]Starting: Setup environment
2019-11-30T16:30:19.2427257Z ==============================================================================
2019-11-30T16:30:19.2427373Z Task         : Bash
2019-11-30T16:30:19.2427451Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T16:30:20.9674343Z 
2019-11-30T16:30:20.9674379Z 
2019-11-30T16:30:20.9674492Z 
2019-11-30T16:30:20.9674604Z  - #66612 (Initial implementation of or-pattern usefulness checking)
2019-11-30T16:30:20.9674715Z  - #66705 (Atomic as_mut_ptr)
2019-11-30T16:30:20.9674803Z  - #66759 (impl TrustedLen for vec::Drain)
2019-11-30T16:30:20.9674941Z  - #66858 (Use LLVMAddAnalysisPasses instead of Rust's wrapper)
2019-11-30T16:30:20.9675063Z  - #66870 (SimplifyArmIdentity only for locals with the same type)
2019-11-30T16:30:20.9675176Z  - #66883 (rustc_typeck: gate AnonConst's generics on feature(const_generics).)
2019-11-30T16:30:20.9675314Z  - #66889 (Make python-generated source files compatible with rustfmt)
2019-11-30T16:30:20.9675425Z  - #66894 (Remove unneeded prelude imports in libcore tests)
2019-11-30T16:30:20.9675549Z  - #66895 (Feature gating *declarations* => new crate `rustc_feature`)
2019-11-30T16:30:20.9676363Z AGENT_BUILDDIRECTORY=D:\a\1
2019-11-30T16:30:20.9676470Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T16:30:20.9676567Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T16:30:20.9676666Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
---
2019-11-30T16:30:20.9692195Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T16:30:20.9692275Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T16:30:20.9692385Z BUILD_SOURCEVERSION=495606319f23056b6100929d1e3904a6ce028cdd
2019-11-30T16:30:20.9692490Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T16:30:20.9692667Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T16:30:20.9692887Z CI_JOB_NAME=i686-msvc-1
2019-11-30T16:30:20.9692980Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T16:30:20.9693070Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T16:30:20.9693177Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T16:31:52.7392638Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T16:31:52.7393090Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T16:31:52.7396848Z 
2019-11-30T16:31:52.7402196Z Failures
2019-11-30T16:31:52.7409237Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T16:31:52.7409784Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T16:31:52.7432394Z Enjoy using Chocolatey? Explore more amazing features to take your
2019-11-30T16:31:52.7432861Z experience to the next level at
2019-11-30T16:31:52.7433296Z  https://chocolatey.org/compare
2019-11-30T16:31:53.2384285Z 
2019-11-30T16:31:53.2384285Z 
2019-11-30T16:31:53.2481591Z ##[error]Bash exited with code '1'.
2019-11-30T16:31:53.2664257Z ##[section]Starting: Checkout
2019-11-30T16:31:53.2789351Z ==============================================================================
2019-11-30T16:31:53.2789486Z Task         : Get sources
2019-11-30T16:31:53.2789592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
