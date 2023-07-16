plain
100  128M    0  128M    0     0  5959k      0 --:--:--  0:00:22 --:--:-- 6968k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Sun Aug 22 16:40:10 UTC 2021 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
Step 25/37 : ENV CC=clang CXX=clang++
 ---> Running in 09499401164e
Removing intermediate container 09499401164e
 ---> 60b22c8735dd
Step 26/37 : ENV PERF_COMMIT 1e19fc4c6168d2f7596e512f42f358f245d8f09d
Removing intermediate container ecc6d7149d83
 ---> d25757e680b5
 ---> d25757e680b5
Step 27/37 : RUN curl -LS -o perf.zip https://github.com/rust-lang/rustc-perf/archive/$PERF_COMMIT.zip &&     unzip perf.zip &&     mv rustc-perf-$PERF_COMMIT rustc-perf &&     rm perf.zip
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 21.4M    0 21.4M    0     0  6891k      0 --:--:--  0:00:03 --:--:-- 7314k
100 33.5M    0 33.5M    0     0  8936k      0 --:--:--  0:00:03 --:--:-- 9390k
Archive:  perf.zip
1e19fc4c6168d2f7596e512f42f358f245d8f09d
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.dockerignore  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.github/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.github/workflows/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.github/workflows/ci.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/Dockerfile  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/README.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/check-benchmarks.sh  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/check-profiling.sh  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/check-triage-script.sh  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/
 extracting: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/LICENSE  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/README.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/LICENSE.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/README.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/Cargo.toml  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/src/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/src/lib.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/.travis.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/0-println.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/ARCHITECTURE.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/CONTRIBUTING.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/LICENSE-APACHE  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/LICENSE-MIT  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/LICENSE-THIRD-PARTY  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/README.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/appveyor.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/perf-config.json  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/cli.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/command_prelude.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/bench.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/build.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/check.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/clean.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/doc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/fetch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/generate_lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/git_checkout.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/init.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/locate_project.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/login.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/metadata.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/new.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/owner.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/pkgid.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/read_manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/run.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/rustc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/rustdoc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/search.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/uninstall.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/update.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/verify_project.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/yank.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/main.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_config.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_context/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_context/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_context/target_info.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_plan.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/compilation.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/compilation_files.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/unit_dependencies.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/custom_build.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/fingerprint.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/job.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/job_queue.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/layout.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/output_depinfo.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/dependency.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/features.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/interning.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/package_id.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/package_id_spec.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/profiles.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/registry.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/conflict_cache.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/context.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/encode.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/resolve.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/types.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/shell.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/source/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/source/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/source/source_id.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/summary.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/workspace.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/lib.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_clean.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_compile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_doc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_fetch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_generate_lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_new.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_output_metadata.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_pkgid.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_read_manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_run.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/resolve.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/directory.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/source.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/utils.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/path.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/index.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/local.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/remote.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/replaced.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/cfg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/dependency_queue.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/errors.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/flock.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/graph.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/hex.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/important_paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/job.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/lev_distance.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/machine_message.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/network.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/process_builder.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/profile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/progress.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/read2.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/rustc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/sha256.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/to_semver.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/to_url.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/toml/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/toml/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/toml/targets.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/vcs.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/Cargo.toml  
    linking: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/LICENSE-APACHE  -> ../../LICENSE-APACHE 
    linking: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/LICENSE-MIT  -> ../../LICENSE-MIT 
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/lib.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/README.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/book.toml  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/SUMMARY.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/faq.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/first-steps.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/index.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/installation.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/build-cache.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/cargo-toml-vs-cargo-lock.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/continuous-integration.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/creating-a-new-project.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/dependencies.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/index.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/project-layout.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/tests.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/why-cargo-exists.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/working-on-an-existing-project.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/
 extracting: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/Cargo-Logo-Small.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/auth-level-acl.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/org-level-acl.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/index.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/build-scripts.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/config.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/environment-variables.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/external-tools.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/index.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/manifest.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/pkgid-spec.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/publishing.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/source-replacement.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/specifying-dependencies.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/unstable.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/theme/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/theme/favicon.png  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/_cargo  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/cargo.bashcomp.sh  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-bench.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-build.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-check.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-clean.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-doc.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-fetch.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-generate-lockfile.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-init.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-install.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-login.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-metadata.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-new.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-owner.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-package.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-pkgid.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-publish.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-run.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-rustc.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-rustdoc.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-search.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-test.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-uninstall.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-update.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-version.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-yank.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo.1  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/alt_registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/bad_config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/bad_manifest_path.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/bench.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_auth.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_lib.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_plan.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_script.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_script_env.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargo_alias_config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargo_command.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargo_features.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/mod.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/cross_compile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/git.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cfg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/check-style.sh  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/check.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/clean.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/concurrent.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/corrupt_git.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cross_compile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cross_publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/custom_target.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/death.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/dep_info.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/directory.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/doc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/features.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/fetch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/freshness.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/generate_lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/git.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/hamcrest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/init.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/jobserver.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/local_registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/lockfile_compat.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/login.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/main.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/metadata.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/net_config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/new.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/out_dir.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/overrides.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/patch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/path.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/plugins.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/proc_macro.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/profile_overrides.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/profile_targets.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/profiles.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/read_manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rename_deps.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/required_features.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/resolve.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/run.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustc_info_cache.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustdoc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustdocflags.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustflags.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/search.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/small_fd_limits.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/tool_paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/update.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/verify_project.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/warn_on_failure.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/workspaces.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.appveyor.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.clog.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.mention-bot  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.travis.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/0-println.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/CHANGELOG.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/CONTRIBUTORS.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/LICENSE-MIT  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/README.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/SPONSORS.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/01_default.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/02_simple.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/03_complex.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/04_new_help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/05_ripgrep.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/06_rustup.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/clap_perf.dat  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/clap_perf.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/plot_perf.gp  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap_dep_graph.dot  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap_dep_graph.png  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/01a_quick_example.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/01b_quick_example.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/01c_quick_example.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/02_apps.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/03_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/04_using_matches.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/05_flag_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/06_positional_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/07_option_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/08_subcommands.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/09_auto_version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/10_default_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/11_only_specific_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/12_typed_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/13a_enum_values_automatic.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/13b_enum_values_manual.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/14_groups.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/15_custom_validator.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/16_app_settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/17_yaml.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/17_yaml.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/18_builder_macro.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/19_auto_authors.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/20_subcommands.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/21_aliases.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/22_stop_parsing_with_--.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/index.html  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/justfile  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/rustfmt.toml  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/meta.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/parser.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/usage.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/validator.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/any_arg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/base.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/flag.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/option.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/positional.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/switched.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/valued.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_matcher.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_matches.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/group.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/matched_arg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/subcommand.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/bash.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/fish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/powershell.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/shell.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/zsh.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/errors.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/fmt.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/lib.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/map.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/osstringext.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/strext.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/suggestions.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/usage_parser.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/app.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/app_settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/arg_aliases.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/borrowed.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/completions.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/conflicts.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/default_vals.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/delimiters.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/derive_order.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/env.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/example1_tmpl_full.txt  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/example1_tmpl_simple.txt  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/flags.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/global_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/groups.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/hidden_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/multiple_occurrences.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/multiple_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/opts.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/positionals.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/posix_compatible.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/possible_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/propagate_globals.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/require.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/subcommands.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/template_help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/tests.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/unique_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/utf8.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/version-numbers.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/yaml.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/0-println.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/1-add-static-arr-item.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/README  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/src/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/src/main.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/.rustfmt.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/.travis.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/CODE_OF_CONDUCT.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/CONTRIBUTING.md  
---
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] StdLink { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.001
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 65.889
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_BUILD_INSTRUMENTED=IR" "-DLLVM_BUILD_RUNTIME=No" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.56.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_ASM_COMPILER=clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm -static-libstdc++" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 12.0.1
-- The ASM compiler identification is Clang
-- Found assembler: /rustroot/bin/clang
-- Check for working C compiler: /rustroot/bin/clang
---
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
Build completed successfully in 0:32:16
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib ../library/core/src/lib.rs
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib -Copt-level=3 ../library/core/src/lib.rs
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
+ cp -r /tmp/rustc-perf ./
++ whoami
+ chown -R user: ./rustc-perf
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build -p collector
    Updating git repository `https://github.com/rust-lang/measureme`
    Updating git repository `https://github.com/rust-lang/team`
---
   Compiling ahash v0.3.5
   Compiling regex-syntax v0.6.18
   Compiling anyhow v1.0.31
   Compiling snap v1.0.1
   Compiling bumpalo v3.4.0
   Compiling fallible-streaming-iterator v0.1.9
   Compiling termcolor v1.1.0
   Compiling unicode-width v0.1.7
   Compiling semver-parser v0.7.0
   Compiling ansi_term v0.11.0
---
   Compiling lock_api v0.3.4
   Compiling humantime v1.3.0
   Compiling phf_shared v0.8.0
   Compiling unicase v2.6.0
   Compiling lru-cache v0.1.2
   Compiling semver v0.9.0
   Compiling phf v0.8.0
   Compiling openssl-sys v0.9.58
   Compiling libsqlite3-sys v0.18.0
---
   Compiling futures v0.3.7
   Compiling hyper v0.13.10
   Compiling serde_json v1.0.53
   Compiling chrono v0.4.11
   Compiling serde_urlencoded v0.6.1
   Compiling intern v0.1.0 (/checkout/obj/rustc-perf/intern)
   Compiling postgres-types v0.1.1
   Compiling tokio-postgres v0.5.4
   Compiling hyper-tls v0.4.1
   Compiling reqwest v0.10.6
   Compiling postgres-native-tls v0.3.0
   Compiling rustc-artifacts v0.2.2
   Compiling database v0.1.0 (/checkout/obj/rustc-perf/database)
   Compiling collector v0.1.0 (/checkout/obj/rustc-perf/collector)
    Finished dev [unoptimized + debuginfo] target(s) in 39.03s
+ RUST_LOG=collector=debug
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc Test --builds Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --runs All --include externs,ctfe-stress-4,inflate,cargo,token-stream-stress,match-stress-enum
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc Test --builds Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --runs All --include externs,ctfe-stress-4,inflate,cargo,token-stream-stress,match-stress-enum`
[2021-08-22T17:35:34Z DEBUG collector] benchmark rust-mozjs - ignored
[2021-08-22T17:35:34Z DEBUG collector] benchmark native-tls-0.1.5 - ignored
[2021-08-22T17:35:34Z DEBUG collector] benchmark README.md - ignored
[2021-08-22T17:35:34Z DEBUG collector] benchmark LICENSE.md - ignored
[2021-08-22T17:35:34Z DEBUG collector] benchmark deeply-nested - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark stm32f4 - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark hyper-2 - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark webrender-wrench - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark coercions - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark webrender - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark serde - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark `cargo`- registered
[2021-08-22T17:35:34Z DEBUG collector] benchmark deep-vector - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark unused-warnings - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark derive - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark syn - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark `externs`- registered
[2021-08-22T17:35:34Z DEBUG collector] benchmark encoding - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark `match-stress-enum`- registered
[2021-08-22T17:35:34Z DEBUG collector] benchmark regression-31157 - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark tokio-webpush-simple - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark style-servo - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark deeply-nested-async - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark ripgrep - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark `inflate`- registered
[2021-08-22T17:35:34Z DEBUG collector] benchmark await-call-tree - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark keccak - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark many-assoc-items - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark html5ever - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark packed-simd - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark piston-image - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark unify-linearly - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark regex - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark diesel - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark match-stress-exhaustive_patterns - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark wg-grammar - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark wf-projection-stress-65510 - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark issue-46449 - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark deeply-nested-closures - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark helloworld - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark clap-rs - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark cranelift-codegen - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark `token-stream-stress`- registered
[2021-08-22T17:35:34Z DEBUG collector] benchmark ucd - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark `ctfe-stress-4`- registered
[2021-08-22T17:35:34Z DEBUG collector] benchmark tuple-stress - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark issue-58319 - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark unicode_normalization - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark futures - doesn't match --include argument, skipping
[2021-08-22T17:35:34Z DEBUG collector] benchmark rustc - doesn't match --include argument, skipping
Profiling with Eprintln
6 benchmarks remaining
Preparing cargo
[2021-08-22T17:35:34Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T17:35:34Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T17:35:34Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T17:35:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaIDRcd#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2021-08-22T17:35:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjnqW24#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2021-08-22T17:35:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKTkTPD#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
Running cargo: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:37:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:37:35Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T17:37:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFECSG3#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:37:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:37:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFECSG3#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFECSG3/incremental-state"
[2021-08-22T17:38:11Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:38:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFECSG3#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFECSG3/incremental-state"
[2021-08-22T17:38:15Z DEBUG collector::execute] applying patch println
[2021-08-22T17:38:15Z DEBUG collector::execute] applying println to "/tmp/.tmpFECSG3"
[2021-08-22T17:38:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T17:38:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFECSG3#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFECSG3/incremental-state"
Running cargo: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:38:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:38:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T17:38:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7eEcLm#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:39:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:39:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7eEcLm#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7eEcLm/incremental-state"
[2021-08-22T17:39:51Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:39:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7eEcLm#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7eEcLm/incremental-state"
[2021-08-22T17:39:58Z DEBUG collector::execute] applying patch println
[2021-08-22T17:39:58Z DEBUG collector::execute] applying println to "/tmp/.tmp7eEcLm"
[2021-08-22T17:39:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T17:39:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7eEcLm#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7eEcLm/incremental-state"
Running cargo: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:40:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:40:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T17:40:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7n4QW#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:41:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:41:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7n4QW#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpb7n4QW/incremental-state"
[2021-08-22T17:43:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:43:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7n4QW#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpb7n4QW/incremental-state"
[2021-08-22T17:43:21Z DEBUG collector::execute] applying patch println
[2021-08-22T17:43:21Z DEBUG collector::execute] applying println to "/tmp/.tmpb7n4QW"
[2021-08-22T17:43:21Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T17:43:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb7n4QW#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpb7n4QW/incremental-state"
5 benchmarks remaining
Preparing ctfe-stress-4
[2021-08-22T17:44:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T17:44:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T17:44:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T17:44:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQjHlkJ#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-08-22T17:44:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcrqmsQ#ctfe-stress-4:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-08-22T17:44:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDKp20O#ctfe-stress-4:0.1.0" "--" "--skip-this-rustc"
Running ctfe-stress-4: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:44:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:44:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T17:44:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoRv7RW#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:44:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:44:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoRv7RW#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpoRv7RW/incremental-state"
[2021-08-22T17:45:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:45:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoRv7RW#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpoRv7RW/incremental-state"
Running ctfe-stress-4: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:45:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:45:16Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T17:45:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCA5iXR#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:45:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:45:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCA5iXR#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCA5iXR/incremental-state"
[2021-08-22T17:46:20Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:46:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCA5iXR#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCA5iXR/incremental-state"
Running ctfe-stress-4: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:46:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:46:21Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T17:46:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnCOg5N#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:46:47Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:46:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnCOg5N#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpnCOg5N/incremental-state"
[2021-08-22T17:47:25Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:47:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnCOg5N#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpnCOg5N/incremental-state"
4 benchmarks remaining
Preparing externs
[2021-08-22T17:47:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T17:47:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T17:47:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T17:47:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphGdP7h#externs:0.1.0" "--" "--skip-this-rustc"
[2021-08-22T17:47:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZ3op1Z#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-08-22T17:47:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoNGRVX#externs:0.1.0" "--release" "--" "--skip-this-rustc"
Running externs: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:47:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:47:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T17:47:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR54gYN#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:47:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:47:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR54gYN#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpR54gYN/incremental-state"
[2021-08-22T17:47:29Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:47:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR54gYN#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpR54gYN/incremental-state"
Running externs: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:47:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:47:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T17:47:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8RcQcd#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:47:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:47:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8RcQcd#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8RcQcd/incremental-state"
[2021-08-22T17:47:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:47:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8RcQcd#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8RcQcd/incremental-state"
Running externs: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:47:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:47:34Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T17:47:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4OwT0i#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:47:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:47:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4OwT0i#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4OwT0i/incremental-state"
[2021-08-22T17:47:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:47:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4OwT0i#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4OwT0i/incremental-state"
3 benchmarks remaining
Preparing inflate
[2021-08-22T17:47:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T17:47:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T17:47:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T17:47:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCeD0sD#inflate:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-08-22T17:47:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJTdyEv#inflate:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-08-22T17:47:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkm794y#inflate:0.1.0" "--" "--skip-this-rustc"
Running inflate: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:47:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:47:39Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T17:47:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqSk6mS#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:47:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:47:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqSk6mS#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpqSk6mS/incremental-state"
[2021-08-22T17:47:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:47:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqSk6mS#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpqSk6mS/incremental-state"
[2021-08-22T17:47:45Z DEBUG collector::execute] applying patch println
[2021-08-22T17:47:45Z DEBUG collector::execute] applying println to "/tmp/.tmpqSk6mS"
[2021-08-22T17:47:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T17:47:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqSk6mS#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpqSk6mS/incremental-state"
Running inflate: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:47:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:47:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T17:47:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGmDHRh#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:47:51Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:47:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGmDHRh#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpGmDHRh/incremental-state"
[2021-08-22T17:47:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:47:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGmDHRh#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpGmDHRh/incremental-state"
[2021-08-22T17:47:56Z DEBUG collector::execute] applying patch println
[2021-08-22T17:47:56Z DEBUG collector::execute] applying println to "/tmp/.tmpGmDHRh"
[2021-08-22T17:47:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T17:47:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGmDHRh#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpGmDHRh/incremental-state"
Running inflate: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:48:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:48:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T17:48:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBnL4OO#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:48:12Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:48:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBnL4OO#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBnL4OO/incremental-state"
[2021-08-22T17:48:25Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:48:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBnL4OO#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBnL4OO/incremental-state"
[2021-08-22T17:48:25Z DEBUG collector::execute] applying patch println
[2021-08-22T17:48:25Z DEBUG collector::execute] applying println to "/tmp/.tmpBnL4OO"
[2021-08-22T17:48:25Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T17:48:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBnL4OO#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBnL4OO/incremental-state"
2 benchmarks remaining
Preparing match-stress-enum
[2021-08-22T17:48:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T17:48:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T17:48:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T17:48:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuKXGsm#match-stress-enum:0.1.0" "--" "--skip-this-rustc"
[2021-08-22T17:48:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoU4w12#match-stress-enum:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-08-22T17:48:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphffzh1#match-stress-enum:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
Running match-stress-enum: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:48:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:48:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T17:48:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnNqjbb#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:48:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:48:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnNqjbb#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpnNqjbb/incremental-state"
[2021-08-22T17:48:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:48:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnNqjbb#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpnNqjbb/incremental-state"
Running match-stress-enum: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:48:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:48:44Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T17:48:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbaHNcT#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:48:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:48:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbaHNcT#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpbaHNcT/incremental-state"
[2021-08-22T17:48:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:48:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbaHNcT#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpbaHNcT/incremental-state"
Running match-stress-enum: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:48:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:48:50Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T17:48:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJqb5td#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:48:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:48:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJqb5td#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJqb5td/incremental-state"
[2021-08-22T17:48:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:48:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJqb5td#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJqb5td/incremental-state"
1 benchmark remaining
Preparing token-stream-stress
[2021-08-22T17:48:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T17:48:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T17:48:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T17:48:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUvhW7P#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-08-22T17:48:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgLwzCV#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-08-22T17:48:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpITMOz8#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
Running token-stream-stress: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:48:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:48:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T17:48:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJ9cyLp#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:48:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:48:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJ9cyLp#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJ9cyLp/incremental-state"
[2021-08-22T17:48:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:48:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJ9cyLp#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJ9cyLp/incremental-state"
Running token-stream-stress: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:48:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:48:58Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T17:48:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXPUA29#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:48:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:48:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXPUA29#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpXPUA29/incremental-state"
[2021-08-22T17:48:59Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:48:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXPUA29#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpXPUA29/incremental-state"
Running token-stream-stress: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T17:48:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T17:48:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T17:48:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1vZSSL#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T17:48:59Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T17:48:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1vZSSL#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp1vZSSL/incremental-state"
[2021-08-22T17:49:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T17:49:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1vZSSL#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp1vZSSL/incremental-state"
+ cd /checkout/obj
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
Generating unstable book md files (x86_64-unknown-linux-gnu)
[TIMING] Assemble { target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
[TIMING] Sysroot { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
[TIMING] Libdir { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
---
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeBeadsGen.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeEmitterGen.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenDAGPatterns.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenHwModes.cpp.o
warning: /checkout/src/llvm-project/llvm/utils/count/count.c: Function control flow change detected (hash mismatch) main Hash = 650973722546133261 [-Wbackend-plugin]
[  1%] Linking C executable ../../bin/count
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  1%] Built target count
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenInstruction.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenInstruction.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/PerfectShuffle/PerfectShuffle.cpp: Function control flow change detected (hash mismatch) main Hash = 899207464865258103 [-Wbackend-plugin]
[  1%] Linking CXX executable ../../bin/llvm-PerfectShuffle
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenMapTable.cpp.o
[  1%] Built target llvm-PerfectShuffle
---
[  8%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
[  8%] Built target LLVMWindowsManifest
[  8%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/not/not.cpp: Function control flow change detected (hash mismatch) main Hash = 862289824575759136 [-Wbackend-plugin]
[  8%] Linking CXX executable ../../bin/not
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  8%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
---
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  8%] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/Option.cpp.o
[  8%] Built target LLVMBitstreamReader
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/yaml-bench/YAMLBench.cpp: Function control flow change detected (hash mismatch) main Hash = 909850505536080120 [-Wbackend-plugin]
[  8%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackend.cpp.o
[  8%] Linking CXX executable ../../bin/yaml-bench
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  8%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackendSkeleton.cpp.o
---
[ 10%] Linking CXX static library ../libLLVMTableGen.a
[ 10%] Built target LLVMTableGen
[ 10%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/ArchitectureSet.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/FileCheck/FileCheck.cpp: Function control flow change detected (hash mismatch) main Hash = 346383081048862348 [-Wbackend-plugin]
[ 10%] Linking CXX executable ../../bin/FileCheck
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndexDiscovery.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
---
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 12%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MasmParser.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 12%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/WasmAsmParser.cpp.o
LLVM ERROR: Function Import: link error: linking module flags 'ProfileSummary': IDs have conflicting values in 'CMakeFiles/obj.llvm-tblgen.dir/X86ModRMFilters.cpp.o' and 'CMakeFiles/obj.llvm-tblgen.dir/X86DisassemblerTables.cpp.o'
PLEASE submit a bug report to https://bugs.llvm.org/ and include the crash backtrace.
[ 12%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Instruction.cpp.o
[ 12%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Instruction.cpp.o
LLVM ERROR: Failed to rename temporary file /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-4118b0.tmp.o to /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/llvmcache-2255FA1F43E5E4C75E7AF4A280A25DE7C642257B: No such file or directory
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 12%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.o
[ 12%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.o
clang-12: error: unable to execute command: Aborted (core dumped)
clang-12: error: linker command failed due to signal (use -v to see invocation)
make[2]: *** [bin/llvm-tblgen] Error 254
make[1]: *** [utils/TableGen/CMakeFiles/llvm-tblgen.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 12%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/XCOFFAsmParser.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 12%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/EntryStage.cpp.o
---
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 14%] Linking CXX static library ../../libLLVMMCParser.a
[ 14%] Built target LLVMMCParser
make: *** [all] Error 2
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 finished in 79.785 seconds
Build completed unsuccessfully in 0:02:14
