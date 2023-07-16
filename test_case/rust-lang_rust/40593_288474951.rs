
; ModuleID = '<stdin>'
source_filename = "rustc.cgu-0.rs"
target datalayout = "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "armv7-unknown-linux-gnueabihf"

define fastcc i64 @test_function(i64, i1 zeroext, i1 zeroext) {
entry-block:
  %cleared = and i64 %0, -3
  %maybe_cleared = select i1 %1, i64 %cleared, i64 %0

  %set = or i64 %maybe_cleared, 2
  %result = select i1 %2, i64 %set, i64 %maybe_cleared
  ret i64 %result
}
