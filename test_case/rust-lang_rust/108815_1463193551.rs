plain
[TIMING] compile::Sysroot { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu } } -- 0.000
[TIMING] builder::Builder::sysroot_libdir::Libdir { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
Building stage0 library artifacts (x86_64-unknown-linux-gnu) 
    Updating crates.io index
    Updating git repository `https://github.com/the8472/ena.git`
---
[RUSTC-TIMING] build_script_build test:false 0.135
   Compiling remove_dir_all v0.5.3
[RUSTC-TIMING] build_script_build test:false 0.182
[RUSTC-TIMING] build_script_build test:false 0.154
   Compiling ena v0.14.1 (https://github.com/the8472/ena.git?branch=get-val-ref#0fb3bedf)
   Compiling elsa v1.8.0
[RUSTC-TIMING] log test:false 0.355
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] remove_dir_all test:false 0.035
---
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: associated function `type_variables_probe` is never used
    |
    |
198 |     fn type_variables_probe(&mut self, vid: ty::TyVid) -> type_variable::TypeVariableValue<'tcx> {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] rustc_symbol_mangling test:false 3.615
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
[RUSTC-TIMING] rustc_infer test:false 3.723
error: could not compile `rustc_infer` due to previous error
---
Total duration:                           7m 43s
------------------------------------------------
root INFO: Free disk space: 596.00 GiB out of total 666.61 GiB (10.59% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
