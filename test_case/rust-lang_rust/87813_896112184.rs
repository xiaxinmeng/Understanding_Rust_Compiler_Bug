
❯ i=1; cargo clean; while ! grep -q 'LLVM ERROR' <(cargo build 2>&1 | tee log); do cargo clean; echo "Pass $i"; ((i++)); done; cat log
Pass 1
   Compiling llvm_erro v0.1.0 (/tmp/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/llvm_erro)
LLVM ERROR: Invalid LLVMRustVisibility value!

❯ i=1; cargo clean; while ! grep -q 'LLVM ERROR' <(cargo build 2>&1 | tee log); do cargo clean; echo "Pass $i"; ((i++)); done; cat log
Pass 1
Pass 2
Pass 3
Pass 4
Pass 5
Pass 6
Pass 7
   Compiling llvm_erro v0.1.0 (/tmp/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/llvm_erro)
LLVM ERROR: Invalid LLVMRustVisibility value!

❯ i=1; cargo clean; while ! grep -q 'LLVM ERROR' <(cargo build 2>&1 | tee log); do cargo clean; echo "Pass $i"; ((i++)); done; cat log
Pass 1
Pass 2
Pass 3
Pass 4
Pass 5
Pass 6
   Compiling llvm_erro v0.1.0 (/tmp/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/llvm_erro)
LLVM ERROR: Invalid LLVMRustVisibility value!

❯ i=1; cargo clean; while ! grep -q 'LLVM ERROR' <(cargo build 2>&1 | tee log); do cargo clean; echo "Pass $i"; ((i++)); done; cat log
Pass 1
   Compiling llvm_erro v0.1.0 (/tmp/sfdgfdgdfgfdg/dfghfdgfd/fdfdgfdg/llvm_erro)
LLVM ERROR: Invalid LLVMRustVisibility value!
