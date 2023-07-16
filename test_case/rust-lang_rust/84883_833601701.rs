
---- [codegen] codegen/abi-efiapi.rs#arm stdout ----
  |  
  | error in revision `arm`: verification with 'FileCheck' failed
  | status: exit status: 2
  | command: "/var/lib/buildkite-agent/builds/rust-llvm-integrate/llvm-project/rust-llvm-integrate-prototype/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/var/lib/buildkite-agent/builds/rust-llvm-integrate/llvm-project/rust-llvm-integrate-prototype/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.arm/abi-efiapi.ll" "/var/lib/buildkite-agent/builds/rust-llvm-integrate/llvm-project/rust-llvm-integrate-prototype/src/test/codegen/abi-efiapi.rs" "--check-prefixes" "CHECK,arm"
  | stdout:
  | ------------------------------------------
  |  
  | ------------------------------------------
  | stderr:
  | ------------------------------------------
  | error: no check strings found with prefix 'CHECK:'
  |  
  | ------------------------------------------ 
