ini
[00:49:17] ---- [codegen-units] codegen-units/item-collection/function-as-argument.rs stdout ----
[00:49:17] 	
[00:49:17] These items should have been contained but were not:
[00:49:17] 
[00:49:17] TRANS_ITEM fn core::ops[0]::FnOnce[0]::call_once[0]<fn(char, f64), (char, f64)>
[00:49:17] TRANS_ITEM fn core::ops[0]::FnOnce[0]::call_once[0]<fn(u32, &str), (u32, &str)>
[00:49:17] 
[00:49:17] 
[00:49:17] 
[00:49:17] These items were contained but should not have been:
[00:49:17] 
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(char, f64), (char, f64)> @@ function_as_argument.cgu-0[Internal]
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(u32, &str), (u32, &str)> @@ function_as_argument.cgu-0[Internal]
[00:49:17] 
[00:49:17] 
[00:49:17] thread '[codegen-units] codegen-units/item-collection/function-as-argument.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1979
[00:49:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:17] 
[00:49:17] ---- [codegen-units] codegen-units/item-collection/trait-method-as-argument.rs stdout ----
[00:49:17] 	
[00:49:17] These items should have been contained but were not:
[00:49:17] 
[00:49:17] TRANS_ITEM fn core::ops[0]::FnMut[0]::call_mut[0]<fn(char) -> char, (char)>
[00:49:17] TRANS_ITEM fn core::ops[0]::FnMut[0]::call_mut[0]<fn(u32) -> u32, (u32)>
[00:49:17] TRANS_ITEM fn core::ops[0]::FnOnce[0]::call_once[0]<fn(char) -> char, (char)>
[00:49:17] TRANS_ITEM fn core::ops[0]::FnOnce[0]::call_once[0]<fn(u32) -> u32, (u32)>
[00:49:17] TRANS_ITEM fn core::ops[0]::Fn[0]::call[0]<fn(char) -> char, (char)>
[00:49:17] TRANS_ITEM fn core::ops[0]::Fn[0]::call[0]<fn(u32) -> u32, (u32)>
[00:49:17] 
[00:49:17] 
[00:49:17] 
[00:49:17] These items were contained but should not have been:
[00:49:17] 
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::FnMut[0]::call_mut[0]<fn(char) -> char, (char)> @@ trait_method_as_argument.cgu-0[Internal]
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::FnMut[0]::call_mut[0]<fn(u32) -> u32, (u32)> @@ trait_method_as_argument.cgu-0[Internal]
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(char) -> char, (char)> @@ trait_method_as_argument.cgu-0[Internal]
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(u32) -> u32, (u32)> @@ trait_method_as_argument.cgu-0[Internal]
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::Fn[0]::call[0]<fn(char) -> char, (char)> @@ trait_method_as_argument.cgu-0[Internal]
[00:49:17] TRANS_ITEM fn core::ops[0]::function[0]::Fn[0]::call[0]<fn(u32) -> u32, (u32)> @@ trait_method_as_argument.cgu-0[Internal]
[00:49:17] 
[00:49:17] 
[00:49:17] thread '[codegen-units] codegen-units/item-collection/trait-method-as-argument.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1979
[00:49:17] 
[00:49:17] 
[00:49:17] failures:
[00:49:17]     [codegen-units] codegen-units/item-collection/function-as-argument.rs
[00:49:17]     [codegen-units] codegen-units/item-collection/trait-method-as-argument.rs
