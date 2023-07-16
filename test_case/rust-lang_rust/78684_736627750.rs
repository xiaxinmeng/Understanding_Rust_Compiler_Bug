
failures:
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [assembly] assembly/asm/wasm-types.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/wasm-types/wasm-types.s" "/checkout/src/test/assembly/asm/wasm-types.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/src/test/assembly/asm/wasm-types.rs:145:11: error: CHECK: expected string not found in input
// CHECK: .set i32_ptr, i32_i32
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/wasm-types/wasm-types.s:447:24: note: scanning from here
 .section .text.i32_ptr,"",@
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/wasm-types/wasm-types.s:451:7: note: possible intended match here
 .functype i32_ptr (i32) -> (i32)
      ^

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/wasm-types/wasm-types.s
Check file: /checkout/src/test/assembly/asm/wasm-types.rs
