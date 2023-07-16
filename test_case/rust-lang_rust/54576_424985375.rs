
C:\projects\rust\src/test\codegen\issue-32364.rs:19:11: error: CHECK: expected string not found in input
// CHECK: define internal x86_stdcallcc void @{{.*}}foo{{.*}}()
          ^
C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\issue-32364\issue-32364.ll:1:1: note: scanning from here
; ModuleID = 'issue_32364.7rcbfp3g-cgu.0'
^
C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\issue-32364\issue-32364.ll:48:1: note: possible intended match here
define internal i32 @_ZN3std3sys7windows7process8ExitCode6as_i3217h8f0526ed8f66b332E(i32* noalias readonly dereferenceable(4) %self) unnamed_addr #1 {
^
