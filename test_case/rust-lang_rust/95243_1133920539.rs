plain
   Compiling rls-data v0.19.1
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/armv7k_apple_watchos.rs:7:22
  |
7 |         llvm_target: "armv7k-apple-watchos".to_string(),
  |
  = note: expected enum `Cow<'static, str>`
           found struct `std::string::String`


error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/armv7k_apple_watchos.rs:9:22
  |
9 |         data_layout: "e-m:o-p:32:32-Fi8-i64:64-a:0:32-n32-S128".to_string(),
  |
  = note: expected enum `Cow<'static, str>`
           found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/armv7k_apple_watchos.rs:10:15
   |
10 |         arch: "arm".to_string(),
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/armv7k_apple_watchos.rs:12:23
   |
12 |             features: "+v7,+vfp4,+neon".to_string(),
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/armv7k_apple_watchos.rs:17:35
   |
17 |               bitcode_llvm_cmdline: "-triple\0\
   |  ___________________________________^
18 | |                 armv7k-apple-watchos3.0.0\0\
19 | |                 -emit-obj\0\
20 | |                 -disable-llvm-passes\0\
23 | |                 -Os\0"
24 | |                 .to_string(),
   | |____________________________^ expected enum `Cow`, found struct `std::string::String`
   |
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`

error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/arm64_32_apple_watchos.rs:7:22
  |
7 |         llvm_target: "arm64_32-apple-watchos".to_string(),
  |
  = note: expected enum `Cow<'static, str>`
           found struct `std::string::String`


error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/arm64_32_apple_watchos.rs:9:22
  |
9 |         data_layout: "e-m:o-p:32:32-i64:64-i128:128-n32:64-S128".to_string(),
  |
  = note: expected enum `Cow<'static, str>`
           found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/arm64_32_apple_watchos.rs:10:15
   |
10 |         arch: "aarch64".to_string(),
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/arm64_32_apple_watchos.rs:12:23
   |
12 |             features: "+neon,+fp-armv8,+apple-a7".to_string(),
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/arm64_32_apple_watchos.rs:17:35
   |
17 |               bitcode_llvm_cmdline: "-triple\0\
   |  ___________________________________^
18 | |                 arm64_32-apple-watchos5.0.0\0\
19 | |                 -emit-obj\0\
20 | |                 -disable-llvm-passes\0\
23 | |                 -Os\0"
24 | |                 .to_string(),
   | |____________________________^ expected enum `Cow`, found struct `std::string::String`
   |
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`

error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:11:9
11 |         llvm_target,
   |         ^^^^^^^^^^^ expected enum `Cow`, found struct `std::string::String`
   |
   = note: expected enum `Cow<'static, str>`
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`

error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:13:22
   |
13 |           data_layout: "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
14 | |             .to_string(),
   | |________________________^ expected enum `Cow`, found struct `std::string::String`
   |
   = note: expected enum `Cow<'static, str>`
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`

error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:15:15
   |
15 |         arch: "x86_64".to_string(),
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:24:35
   |
24 |               bitcode_llvm_cmdline: "-triple\0\
   |  ___________________________________^
25 | |                 x86_64-apple-watchos5.0-simulator\0\
26 | |                 -emit-obj\0\
27 | |                 -disable-llvm-passes\0\
30 | |                 -Os\0"
31 | |                 .to_string(),
   | |____________________________^ expected enum `Cow`, found struct `std::string::String`
   |
