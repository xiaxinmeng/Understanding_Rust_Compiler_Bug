plain
    Checking cranelift-object v0.75.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#c71ad949)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
    Finished release [optimized] target(s) in 27.56s
Checking stage0 gcc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: current package believes it's in a workspace when it's not:
current:   /checkout/compiler/rustc_codegen_gcc/Cargo.toml


this may be fixable by adding `compiler/rustc_codegen_gcc` to the `workspace.members` array of the manifest located at: /checkout/Cargo.toml
Alternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest.
