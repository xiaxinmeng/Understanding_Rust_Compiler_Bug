plain
Attempting with retry: docker build --rm -t rust-ci -f /home/runner/work/rust/rust/src/ci/docker/host-x86_64/mingw-check/Dockerfile /home/runner/work/rust/rust/src/ci/docker
Sending build context to Docker daemon  423.9kB

Step 1/15 : FROM ubuntu:22.04
Head "https://registry-1.docker.io/v2/library/ubuntu/manifests/22.04": Get "https://auth.docker.io/token?account=githubactions&scope=repository%3Alibrary%2Fubuntu%3Apull&service=registry.docker.io": EOF
Sending build context to Docker daemon  423.9kB

Step 1/15 : FROM ubuntu:22.04
22.04: Pulling from library/ubuntu
---
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: unused import: `crate::middle::resolve_bound_vars as rbv`
  |
  |
1 | use crate::middle::resolve_bound_vars as rbv;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `DefKind`, `Res`
  |
  |
6 | use rustc_hir::def::{DefKind, Res};

    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: could not compile `rustc_middle` (lib) due to 2 previous errors
