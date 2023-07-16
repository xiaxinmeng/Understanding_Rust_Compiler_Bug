
failures:

---- [run-fail] run-fail/adjust_never.rs stdout ----


executing /Users/rreverser/oss/rust/build/x86_64-apple-darwin/stage2/bin/rustc /Users/rreverser/oss/rust/src/test/run-fail/adjust_never.rs -L /Users/rreverser/oss/rust/build/x86_64-apple-darwin/test/run-fail --target=asmjs-unknown-emscripten -L /Users/rreverser/oss/rust/build/x86_64-apple-darwin/test/run-fail/adjust_never.stage2-asmjs-unknown-emscripten.run-fail.libaux -C prefer-dynamic -o /Users/rreverser/oss/rust/build/x86_64-apple-darwin/test/run-fail/adjust_never.stage2-asmjs-unknown-emscripten.js -Crpath -O -Lnative=/Users/rreverser/oss/rust/build/asmjs-unknown-emscripten/native/rust-test-helpers
------stdout------------------------------

------stderr------------------------------
warning: unused variable: `x`
  --> /Users/rreverser/oss/rust/src/test/run-fail/adjust_never.rs:17:9
   |
17 |     let x: ! = panic!();
   |         ^
   |
   = note: #[warn(unused_variables)] on by default

warning: unreachable expression
  --> /Users/rreverser/oss/rust/src/test/run-fail/adjust_never.rs:17:16
   |
17 |     let x: ! = panic!();
   |                ^^^^^^^^
   |
   = note: #[warn(unreachable_code)] on by default
   = note: this error originates in a macro outside of the current crate

warning: unused variable: `y`
  --> /Users/rreverser/oss/rust/src/test/run-fail/adjust_never.rs:18:9
   |
18 |     let y: u32 = x;
   |         ^
   |
   = note: #[warn(unused_variables)] on by default

warning: unreachable expression
  --> /Users/rreverser/oss/rust/src/test/run-fail/adjust_never.rs:18:18
   |
18 |     let y: u32 = x;
   |                  ^
   |
   = note: #[warn(unreachable_code)] on by default


------------------------------------------
executing /usr/local/bin/node /Users/rreverser/oss/rust/build/x86_64-apple-darwin/test/run-fail/adjust_never.stage2-asmjs-unknown-emscripten.js
------stdout------------------------------

------stderr------------------------------
thread 'main' panicked at 'explicit panic', /Users/rreverser/oss/rust/src/test/run-fail/adjust_never.rs:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

/Users/rreverser/oss/rust/build/x86_64-apple-darwin/test/run-fail/adjust_never.stage2-asmjs-unknown-emscripten.js:1
(function (exports, require, module, __filename, __dirname) { var Module;if(!Module)Module=(typeof Module!=="undefined"?Module:null)||{};var moduleOverrides={};for(var key in Module){if(Module.hasOwnProperty(key)){moduleOverrides[key]=Module[key]}}var ENVIRONMENT_IS_WEB=false;var ENVIRONMENT_IS_WORKER=false;var ENVIRONMENT_IS_NODE=false;var ENVIRONMENT_IS_SHELL=false;if(Module["ENVIRONMENT"]){if(Module["ENVIRONMENT"]==="WEB"){ENVIRONMENT_IS_WEB=true}else if(Module["ENVIRONMENT"]==="WORKER"){ENVIRONMENT_IS_WORKER=true}else if(Module["ENVIRONMENT"]==="NODE"){ENVIRONMENT_IS_NODE=true}else if(Module["ENVIRONMENT"]==="SHELL"){ENVIRONMENT_IS_SHELL=true}else{throw new Error("The provided Module['ENVIRONMENT'] value is not valid. It must be one of: WEB|WORKER|NODE|SHELL.")}}else{ENVIRONMENT_IS_WEB=typeof window==="object";ENVIRONMENT_IS_WORKER=typeof importScripts==="function";ENVIRONMENT_IS_NODE=type
5265640 - Exception catching is disabled, this exception cannot be caught. Compile with -s DISABLE_EXCEPTION_CATCHING=0 or DISABLE_EXCEPTION_CATCHING=2 to catch.

------------------------------------------

error: failure produced the wrong error: exit code: 7
status: exit code: 7
command: /usr/local/bin/node /Users/rreverser/oss/rust/build/x86_64-apple-darwin/test/run-fail/adjust_never.stage2-asmjs-unknown-emscripten.js
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /Users/rreverser/oss/rust/src/test/run-fail/adjust_never.rs:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

/Users/rreverser/oss/rust/build/x86_64-apple-darwin/test/run-fail/adjust_never.stage2-asmjs-unknown-emscripten.js:1
(function (exports, require, module, __filename, __dirname) { var Module;if(!Module)Module=(typeof Module!=="undefined"?Module:null)||{};var moduleOverrides={};for(var key in Module){if(Module.hasOwnProperty(key)){moduleOverrides[key]=Module[key]}}var ENVIRONMENT_IS_WEB=false;var ENVIRONMENT_IS_WORKER=false;var ENVIRONMENT_IS_NODE=false;var ENVIRONMENT_IS_SHELL=false;if(Module["ENVIRONMENT"]){if(Module["ENVIRONMENT"]==="WEB"){ENVIRONMENT_IS_WEB=true}else if(Module["ENVIRONMENT"]==="WORKER"){ENVIRONMENT_IS_WORKER=true}else if(Module["ENVIRONMENT"]==="NODE"){ENVIRONMENT_IS_NODE=true}else if(Module["ENVIRONMENT"]==="SHELL"){ENVIRONMENT_IS_SHELL=true}else{throw new Error("The provided Module['ENVIRONMENT'] value is not valid. It must be one of: WEB|WORKER|NODE|SHELL.")}}else{ENVIRONMENT_IS_WEB=typeof window==="object";ENVIRONMENT_IS_WORKER=typeof importScripts==="function";ENVIRONMENT_IS_NODE=type
5265640 - Exception catching is disabled, this exception cannot be caught. Compile with -s DISABLE_EXCEPTION_CATCHING=0 or DISABLE_EXCEPTION_CATCHING=2 to catch.

------------------------------------------

thread '[run-fail] run-fail/adjust_never.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2555
note: Run with `RUST_BACKTRACE=1` for a backtrace.
