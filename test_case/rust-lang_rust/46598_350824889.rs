
[00:59:30] ---- [compile-fail] compile-fail/borrowck/borrowck-local-borrow-outlives-fn.rs stdout ----
[00:59:30] 	
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/borrowck/borrowck-local-borrow-outlives-fn.rs:15: unexpected error: '15:5: 15:7: `x` does not live long enough [E0597]'
[00:59:30] 
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/borrowck/borrowck-local-borrow-outlives-fn.rs:16: expected error not found: borrowed value does not live long enough
[00:59:30] 
[00:59:30] error in revision `mir`: 1 unexpected errors found, 1 expected errors not found
...
[00:59:30] ---- [compile-fail] compile-fail/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs stdout ----
[00:59:30] 	
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs:16: unexpected error: '16:15: 16:23: `z.1` does not live long enough [E0597]'
[00:59:30] 
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs:18: expected error not found: [E0597]
[00:59:30] 
[00:59:30] error in revision `mir`: 1 unexpected errors found, 1 expected errors not found
...
[00:59:30] ---- [compile-fail] compile-fail/region-borrow-params-issue-29793-big.rs stdout ----
[00:59:30] 	
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/region-borrow-params-issue-29793-big.rs:84: unexpected error: '84:5: 84:6: `x` does not live long enough [E0597]'
[00:59:30] 
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/region-borrow-params-issue-29793-big.rs:84: unexpected error: '84:5: 84:6: `y` does not live long enough [E0597]'
[00:59:30] 
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/region-borrow-params-issue-29793-big.rs:84: expected error not found: borrowed value does not live long enough
[00:59:30] 
[00:59:30] error in revision `mir`: /checkout/src/test/compile-fail/region-borrow-params-issue-29793-big.rs:84: expected error not found: borrowed value does not live long enough
[00:59:30] 
[00:59:30] error in revision `mir`: 2 unexpected errors found, 2 expected errors not found
