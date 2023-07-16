
Updating only changed submodules
Submodules updated in 0.12 seconds
    Finished dev [unoptimized] target(s) in 0.40s
Checking stage0 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.46s
Checking stage0 std test/bench/example targets (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.39s
Checking stage0 compiler artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] stale: changed env "CFG_LLVM_ROOT"
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]        Some("c:\\Users\\milo5\\Desktop\\GitHub\\rust\\build\\x86_64-pc-windows-msvc\\ci-llvm/bin\\llvm-config.exe") != Some("C:\\Users\\milo5\\Desktop\\GitHub\\rust\\build\\x86_64-pc-windows-msvc\\ci-llvm/bin\\llvm-config.exe")
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc-main v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc)/Check { test: false }/TargetInner { name: "rustc-main", doc: true, ..: with_path("C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc\\src\\main.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_driver/74659112559a43bc) != old (rustc_driver/97c06cec17f54cc2)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_driver v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_driver)/Check { test: false }/TargetInner { ..: lib_target("rustc_driver", ["dylib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_driver\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_interface/ab6ec758641a91f1) != old (rustc_interface/eb9f92409a57cd3e)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_interface v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_interface)/Check { test: false }/TargetInner { doctest: false, ..: lib_target("rustc_interface", ["lib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_interface\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_codegen_llvm/2a7c1c23e5335c9) != old (rustc_codegen_llvm/18aa118ed0f27aa3)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_codegen_llvm v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_codegen_llvm)/Check { test: false }/TargetInner { tested: false, doctest: false, ..: lib_target("rustc_codegen_llvm", ["lib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_codegen_llvm\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_llvm/547782de7de2eb20) != old (rustc_llvm/e0eead57ef186c39)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_llvm v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_llvm)/Check { test: false }/TargetInner { ..: lib_target("rustc_llvm", ["lib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_llvm\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (build_script_build/9d5278997f5d00cb) != old (build_script_build/166b70ac75843216)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_llvm v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_llvm)/RunCustomBuild/TargetInner { ..: custom_build_target("build-script-build", "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_llvm\\build.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: env var `LLVM_CONFIG` changed: previously Some("c:\\Users\\milo5\\Desktop\\GitHub\\rust\\build\\x86_64-pc-windows-msvc\\ci-llvm/bin\\llvm-config.exe"), now Some("C:\\Users\\milo5\\Desktop\\GitHub\\rust\\build\\x86_64-pc-windows-msvc\\ci-llvm/bin\\llvm-config.exe")
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc-main v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc)/Check { test: true }/TargetInner { name: "rustc-main", doc: true, ..: with_path("C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc\\src\\main.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_driver/74659112559a43bc) != old (rustc_driver/97c06cec17f54cc2)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] stale: changed env "CFG_LLVM_ROOT"
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]        Some("c:\\Users\\milo5\\Desktop\\GitHub\\rust\\build\\x86_64-pc-windows-msvc\\ci-llvm/bin\\llvm-config.exe") != Some("C:\\Users\\milo5\\Desktop\\GitHub\\rust\\build\\x86_64-pc-windows-msvc\\ci-llvm/bin\\llvm-config.exe")
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_codegen_llvm v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_codegen_llvm)/Check { test: true }/TargetInner { tested: false, doctest: false, ..: lib_target("rustc_codegen_llvm", ["lib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_codegen_llvm\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_llvm/547782de7de2eb20) != old (rustc_llvm/e0eead57ef186c39)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_driver v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_driver)/Check { test: true }/TargetInner { ..: lib_target("rustc_driver", ["dylib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_driver\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_interface/ab6ec758641a91f1) != old (rustc_interface/eb9f92409a57cd3e)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_interface v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_interface)/Check { test: true }/TargetInner { doctest: false, ..: lib_target("rustc_interface", ["lib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_interface\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (rustc_codegen_llvm/2a7c1c23e5335c9) != old (rustc_codegen_llvm/18aa118ed0f27aa3)
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc_llvm v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_llvm)/Check { test: true }/TargetInner { ..: lib_target("rustc_llvm", ["lib"], "C:\\Users\\milo5\\Desktop\\GitHub\\rust\\compiler\\rustc_llvm\\src\\lib.rs", Edition2021) }
[2021-12-29T03:28:37Z INFO  cargo::core::compiler::fingerprint]     err: unit dependency information changed

    Caused by:
        new (build_script_build/9d5278997f5d00cb) != old (build_script_build/166b70ac75843216)
   Compiling rustc_llvm v0.0.0 (C:\Users\milo5\Desktop\GitHub\rust\compiler\rustc_llvm)
