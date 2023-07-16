plain
[00:51:48] ---- [ui] ui/borrowck/two-phase-method-receivers.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-method-receivers/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/borrowck/two-phase-method-receivers.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:48] 
[00:51:48] ---- [ui] ui/borrowck/two-phase-multiple-activations.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-multiple-activations/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/borrowck/two-phase-multiple-activations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/const-eval/const_transmute.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/const_transmute/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/const-eval/const_transmute.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/const-eval/enum_discr.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/enum_discr/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/const-eval/enum_discr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/const-eval/strlen.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/strlen/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/const-eval/strlen.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/deprecated-macro_escape-inner.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecated-macro_escape-inner/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/deprecated-macro_escape-inner.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/deprecated-macro_escape.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecated-macro_escape/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/deprecated-macro_escape.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/deriving-meta-empty-trait-list.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving-meta-empty-trait-list/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/deriving-meta-empty-trait-list.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/enum-size-variance.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-size-variance/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/enum-size-variance.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/feature-gate-trivial_bounds-lint.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-trivial_bounds-lint/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/feature-gate-trivial_bounds-lint.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant-closure/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/in-band-lifetimes/impl/ref-underscore.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/impl/ref-underscore/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/in-band-lifetimes/impl/ref-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/in-band-lifetimes/impl/path-underscore.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/impl/path-underscore/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/in-band-lifetimes/impl/path-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/in-band-lifetimes/impl/trait-underscore.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/impl/trait-underscore/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/in-band-lifetimes/impl/trait-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/inference_unstable.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference_unstable/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
[00:51:48]     at Module.load (module.js:560:32)
[00:51:48]     at tryModuleLoad (module.js:503:12)
[00:51:48]     at Function.Module._load (module.js:495:3)
[00:51:48]     at Function.Module.runMain (module.js:682:10)
[00:51:48]     at startup (bootstrap_node.js:191:16)
[00:51:48]     at bootstrap_node.js:613:3
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] thread '[ui] ui/inference_unstable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:48] 
[00:51:48] 
[00:51:48] ---- [ui] ui/issue-19100.rs stdout ----
[00:51:48] 
[00:51:48] error: test run failed!
[00:51:48] status: exit code: 1
[00:51:48] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-19100/a.wasm"
[00:51:48] ------------------------------------------
[00:51:48] 
[00:51:48] ------------------------------------------
[00:51:48] stderr:
[00:51:48] stderr:
[00:51:48] ------------------------------------------
[00:51:48] /checkout/src/etc/wasm32-shim.js:129
[00:51:48] let instance = new WebAssembly.Instance(m, imports);
[00:51:48] 
[00:51:48] 
[00:51:48] LinkError: WebAssembly Instantiation: Import #0 module="env" function="sinf" error: function import requires a callable
[00:51:48]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:129:16)
[00:51:48]     at Module._compile (module.js:641:30)
[00:51:48]     at Object.Module._extensions..js (module.js:652:10)
---
[00:51:48] test result: FAILED. 1483 passed; 44 failed; 7 ignored; 0 measured; 0 filtered out
[00:51:48] 
[00:51:48] 
[00:51:48] 
[00:51:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:48] 
[00:51:48] 
[00:51:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:51:48] Build completed unsuccessfully in 0:48:13
---
travis_time:end:047e6f3e:start=1530275994502363794,finish=1530275994510230705,duration=7866911
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10cd5a7c
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:011d0180
$ dmesg | grep -i kill
