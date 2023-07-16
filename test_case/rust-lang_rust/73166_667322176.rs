
[INFO] [stdout] error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
[INFO] [stdout]     --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/vektor-0.2.1/src/x86/avx2.rs:2452:5
[INFO] [stdout]      |
[INFO] [stdout] 2452 |     crate::mem::transmute(constify_imm8!(imm8, call))
[INFO] [stdout]      |     ^^^^^^^^^^^^^^^^^^^^^
[INFO] [stdout]      |
[INFO] [stdout]      = note: source type: `i32` (32 bits)
[INFO] [stdout]      = note: target type: `i8` (8 bits)
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stdout] error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
[INFO] [stdout]     --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/vektor-0.2.1/src/x86/avx2.rs:2471:5
[INFO] [stdout]      |
[INFO] [stdout] 2471 |     crate::mem::transmute(constify_imm8!(imm8, call))
[INFO] [stdout]      |     ^^^^^^^^^^^^^^^^^^^^^
[INFO] [stdout]      |
[INFO] [stdout]      = note: source type: `i32` (32 bits)
[INFO] [stdout]      = note: target type: `i16`
