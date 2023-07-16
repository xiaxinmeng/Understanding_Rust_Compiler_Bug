plain
##[section]Starting: Windows x86_64-mingw-1
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
Agent machine name: 'fv-az425'
Current agent version: '2.170.1'
##[group]Operating System
10.0.14393
Datacenter
Datacenter
##[endgroup]
##[group]Virtual Environment
Environment: windows-2016
Version: 20200531.1
Included Software: https://github.com/actions/virtual-environments/blob/win16/20200531.1/images/win/Windows2016-Readme.md
##[endgroup]
Agent running as: 'VssAdministrator'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = C:/hostedtoolcache/windows Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = c:\vsts\perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/d888ca66-b87d-45dc-a87e-f473ae90710e.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73188/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73188/merge:refs/remotes/pull/73188/merge
---
configure: 
configure: run `python D:\a\1\s/x.py --help`
configure: 
Attempting with retry: make prepare
      1 [main] make (3968) C:\msys64\usr\bin\make.exe: *** fatal error - cygheap base mismatch detected - 0x180317408/0x18033E408.
Search for cygwin1.dll using the Windows Start->Find/Search facility
and delete all but the most recent version.  The most recent version *should*
reside in x:\cygwin\bin, where 'x' is the drive on which you have
installed the cygwin distribution.  Rebooting is also suggested if you
installed the cygwin distribution.  Rebooting is also suggested if you
are unable to find another cygwin DLL.
Command failed. Attempt 2/5:
      0 [main] make (2332) C:\msys64\usr\bin\make.exe: *** fatal error - cygheap base mismatch detected - 0x180317408/0x18033E408.
Search for cygwin1.dll using the Windows Start->Find/Search facility
and delete all but the most recent version.  The most recent version *should*
reside in x:\cygwin\bin, where 'x' is the drive on which you have
installed the cygwin distribution.  Rebooting is also suggested if you
installed the cygwin distribution.  Rebooting is also suggested if you
are unable to find another cygwin DLL.
Command failed. Attempt 3/5:
      0 [main] make (3116) C:\msys64\usr\bin\make.exe: *** fatal error - cygheap base mismatch detected - 0x180317408/0x18033E408.
Search for cygwin1.dll using the Windows Start->Find/Search facility
and delete all but the most recent version.  The most recent version *should*
reside in x:\cygwin\bin, where 'x' is the drive on which you have
installed the cygwin distribution.  Rebooting is also suggested if you
installed the cygwin distribution.  Rebooting is also suggested if you
are unable to find another cygwin DLL.
Command failed. Attempt 4/5:
      0 [main] make (5508) C:\msys64\usr\bin\make.exe: *** fatal error - cygheap base mismatch detected - 0x180317408/0x18033E408.
Search for cygwin1.dll using the Windows Start->Find/Search facility
and delete all but the most recent version.  The most recent version *should*
reside in x:\cygwin\bin, where 'x' is the drive on which you have
installed the cygwin distribution.  Rebooting is also suggested if you
installed the cygwin distribution.  Rebooting is also suggested if you
are unable to find another cygwin DLL.
Command failed. Attempt 5/5:
      1 [main] make (4320) C:\msys64\usr\bin\make.exe: *** fatal error - cygheap base mismatch detected - 0x180317408/0x18033E408.
Search for cygwin1.dll using the Windows Start->Find/Search facility
and delete all but the most recent version.  The most recent version *should*
reside in x:\cygwin\bin, where 'x' is the drive on which you have
installed the cygwin distribution.  Rebooting is also suggested if you
installed the cygwin distribution.  Rebooting is also suggested if you
are unable to find another cygwin DLL.
The command has failed after 5 attempts.
== clock drift check ==
  local time: Tue Jun  9 23:05:47 CUT 2020
  network time: Tue, 09 Jun 2020 23:05:47 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73188/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73188/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3876) (python)
Terminate orphan process: pid (5988) (sccache)
