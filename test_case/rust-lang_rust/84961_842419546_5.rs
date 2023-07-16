

Mismatch at tests/source/issue-3302.rs:3:
Mismatch at tests/source/issue-3302.rs:3:
 macro_rules! moo1 {
         bar! {
-        "
 "
+"
+"
         }
     };
 }

Mismatch at tests/source/issue-3302.rs:34:
 macro_rules! moo4 {
         bar! {
-        "
+"
     foo
     foo
         bar
 baz"

Mismatch at tests/source/cfg_if/detect/arch/mips.rs:6:
 #[allow_internal_unstable(stdsimd_internal, stdsimd)]
 macro_rules! is_mips_feature_detected {
     ("msa") => {
-        cfg!(target_feature = "msa") || $crate::detect::check_for($crate::detect::Feature::msa)
+        cfg!(target_feature = "msa") ||
+            $crate::detect::check_for($crate::detect::Feature::msa)
     };
     ($t:tt,) => {
         is_mips_feature_detected!($t);

Mismatch at tests/source/cfg_if/detect/arch/mips.rs:13:
     };
-    ($t:tt) => {
-        compile_error!(concat!("unknown mips target feature: ", $t))
-    };
+    ($t:tt) => { compile_error!(concat!("unknown mips target feature: ", $t)) };
 
 
 /// MIPS CPU Feature enum. Each variant denotes a position in a bitset for a

Mismatch at tests/source/cfg_if/detect/arch/arm.rs:6:
 #[allow_internal_unstable(stdsimd_internal, stdsimd)]
 macro_rules! is_arm_feature_detected {
     ("neon") => {
-        cfg!(target_feature = "neon") || $crate::detect::check_for($crate::detect::Feature::neon)
+        cfg!(target_feature = "neon") ||
+            $crate::detect::check_for($crate::detect::Feature::neon)
     };
     ("pmull") => {
-        cfg!(target_feature = "pmull") || $crate::detect::check_for($crate::detect::Feature::pmull)
+        cfg!(target_feature = "pmull") ||
+            $crate::detect::check_for($crate::detect::Feature::pmull)
     };
-    ("v7") => {
-        compile_error!("\"v7\" feature cannot be detected at run-time")
-    };
-    ("vfp2") => {
-        compile_error!("\"vfp2\" feature cannot be detected at run-time")
-    };
-    ("vfp3") => {
-        compile_error!("\"vfp3\" feature cannot be detected at run-time")
-    };
-    ("vfp4") => {
-        compile_error!("\"vfp4\" feature cannot be detected at run-time")
-    };
+    ("v7") => { compile_error!("\"v7\" feature cannot be detected at run-time") };
+    ("vfp2") => { compile_error!("\"vfp2\" feature cannot be detected at run-time") };
+    ("vfp3") => { compile_error!("\"vfp3\" feature cannot be detected at run-time") };
+    ("vfp4") => { compile_error!("\"vfp4\" feature cannot be detected at run-time") };
     ($t:tt,) => {
         is_arm_feature_detected!($t);


Mismatch at tests/source/cfg_if/detect/arch/arm.rs:29:
-    ($t:tt) => {
-        compile_error!(concat!("unknown arm target feature: ", $t))
-    };
+    ($t:tt) => { compile_error!(concat!("unknown arm target feature: ", $t)) };
 
 
 /// ARM CPU Feature enum. Each variant denotes a position in a bitset for a

Mismatch at tests/source/cfg_if/detect/arch/powerpc64.rs:6:
 #[allow_internal_unstable(stdsimd_internal, stdsimd)]
 macro_rules! is_powerpc64_feature_detected {
     ("altivec") => {
-        cfg!(target_feature = "altivec")
-            || $crate::detect::check_for($crate::detect::Feature::altivec)
+        cfg!(target_feature = "altivec") ||
+            $crate::detect::check_for($crate::detect::Feature::altivec)
     };
     ("vsx") => {
-        cfg!(target_feature = "vsx") || $crate::detect::check_for($crate::detect::Feature::vsx)
+        cfg!(target_feature = "vsx") ||
+            $crate::detect::check_for($crate::detect::Feature::vsx)
     };
     ("power8") => {
-        cfg!(target_feature = "power8")
-            || $crate::detect::check_for($crate::detect::Feature::power8)
+        cfg!(target_feature = "power8") ||
+            $crate::detect::check_for($crate::detect::Feature::power8)
     };
     ($t:tt,) => {
         is_powerpc64_feature_detected!($t);

Mismatch at tests/source/cfg_if/detect/arch/powerpc64.rs:21:
     };
-    ($t:tt) => {
-        compile_error!(concat!("unknown powerpc64 target feature: ", $t))
-    };
+    ($t:tt) => { compile_error!(concat!("unknown powerpc64 target feature: ", $t)) };
 
 
 /// PowerPC64 CPU Feature enum. Each variant denotes a position in a bitset

Mismatch at tests/source/cfg_if/detect/arch/aarch64.rs:7:
 macro_rules! is_aarch64_feature_detected {
     ("neon") => {
         // FIXME: this should be removed once we rename Aarch64 neon to asimd
-        cfg!(target_feature = "neon") || $crate::detect::check_for($crate::detect::Feature::asimd)
+        cfg!(target_feature = "neon") ||
+            $crate::detect::check_for($crate::detect::Feature::asimd)
     };
     ("asimd") => {
-        cfg!(target_feature = "neon") || $crate::detect::check_for($crate::detect::Feature::asimd)
+        cfg!(target_feature = "neon") ||
+            $crate::detect::check_for($crate::detect::Feature::asimd)
     };
     ("pmull") => {
-        cfg!(target_feature = "pmull") || $crate::detect::check_for($crate::detect::Feature::pmull)
+        cfg!(target_feature = "pmull") ||
+            $crate::detect::check_for($crate::detect::Feature::pmull)
     };
     ("fp") => {
-        cfg!(target_feature = "fp") || $crate::detect::check_for($crate::detect::Feature::fp)
+        cfg!(target_feature = "fp") ||
+            $crate::detect::check_for($crate::detect::Feature::fp)
     };
     ("fp16") => {
-        cfg!(target_feature = "fp16") || $crate::detect::check_for($crate::detect::Feature::fp16)
+        cfg!(target_feature = "fp16") ||
+            $crate::detect::check_for($crate::detect::Feature::fp16)
     };
     ("sve") => {
-        cfg!(target_feature = "sve") || $crate::detect::check_for($crate::detect::Feature::sve)
+        cfg!(target_feature = "sve") ||
+            $crate::detect::check_for($crate::detect::Feature::sve)
     };
     ("crc") => {
-        cfg!(target_feature = "crc") || $crate::detect::check_for($crate::detect::Feature::crc)
+        cfg!(target_feature = "crc") ||
+            $crate::detect::check_for($crate::detect::Feature::crc)
     };
     ("crypto") => {
-        cfg!(target_feature = "crypto")
-            || $crate::detect::check_for($crate::detect::Feature::crypto)
+        cfg!(target_feature = "crypto") ||
+            $crate::detect::check_for($crate::detect::Feature::crypto)
     };
     ("lse") => {
-        cfg!(target_feature = "lse") || $crate::detect::check_for($crate::detect::Feature::lse)
+        cfg!(target_feature = "lse") ||
+            $crate::detect::check_for($crate::detect::Feature::lse)
     };
     ("rdm") => {
-        cfg!(target_feature = "rdm") || $crate::detect::check_for($crate::detect::Feature::rdm)
+        cfg!(target_feature = "rdm") ||
+            $crate::detect::check_for($crate::detect::Feature::rdm)
     };
     ("rcpc") => {
-        cfg!(target_feature = "rcpc") || $crate::detect::check_for($crate::detect::Feature::rcpc)
+        cfg!(target_feature = "rcpc") ||
+            $crate::detect::check_for($crate::detect::Feature::rcpc)
     };
     ("dotprod") => {
-        cfg!(target_feature = "dotprod")
-            || $crate::detect::check_for($crate::detect::Feature::dotprod)
+        cfg!(target_feature = "dotprod") ||
+            $crate::detect::check_for($crate::detect::Feature::dotprod)
     };
     ("ras") => {
         compile_error!("\"ras\" feature cannot be detected at run-time")

Mismatch at tests/source/cfg_if/detect/arch/aarch64.rs:59:
     ($t:tt,) => {
         is_aarch64_feature_detected!($t);
     };
-    ($t:tt) => {
-        compile_error!(concat!("unknown aarch64 target feature: ", $t))
-    };
+    ($t:tt) => { compile_error!(concat!("unknown aarch64 target feature: ", $t)) };
 
 
 /// ARM Aarch64 CPU Feature enum. Each variant denotes a position in a bitset

Mismatch at tests/source/cfg_if/detect/arch/mips64.rs:6:
 #[allow_internal_unstable(stdsimd_internal, stdsimd)]
 macro_rules! is_mips64_feature_detected {
     ("msa") => {
-        cfg!(target_feature = "msa") || $crate::detect::check_for($crate::detect::Feature::msa)
+        cfg!(target_feature = "msa") ||
+            $crate::detect::check_for($crate::detect::Feature::msa)
     };
     ($t:tt,) => {
         is_mips64_feature_detected!($t);

Mismatch at tests/source/cfg_if/detect/arch/mips64.rs:13:
     };
-    ($t:tt) => {
-        compile_error!(concat!("unknown mips64 target feature: ", $t))
-    };
+    ($t:tt) => { compile_error!(concat!("unknown mips64 target feature: ", $t)) };
 
 
 /// MIPS64 CPU Feature enum. Each variant denotes a position in a bitset

Mismatch at tests/source/cfg_if/detect/arch/powerpc.rs:6:
 #[allow_internal_unstable(stdsimd_internal, stdsimd)]
 macro_rules! is_powerpc_feature_detected {
     ("altivec") => {
-        cfg!(target_feature = "altivec")
-            || $crate::detect::check_for($crate::detect::Feature::altivec)
+        cfg!(target_feature = "altivec") ||
+            $crate::detect::check_for($crate::detect::Feature::altivec)
     };
     ("vsx") => {
-        cfg!(target_feature = "vsx") || $crate::detect::check_for($crate::detect::Feature::vsx)
+        cfg!(target_feature = "vsx") ||
+            $crate::detect::check_for($crate::detect::Feature::vsx)
     };
     ("power8") => {
-        cfg!(target_feature = "power8")
-            || $crate::detect::check_for($crate::detect::Feature::power8)
+        cfg!(target_feature = "power8") ||
+            $crate::detect::check_for($crate::detect::Feature::power8)
     };
     ($t:tt,) => {
         is_powerpc_feature_detected!($t);

Mismatch at tests/source/cfg_if/detect/arch/powerpc.rs:21:
     };
-    ($t:tt) => {
-        compile_error!(concat!("unknown powerpc target feature: ", $t))
-    };
+    ($t:tt) => { compile_error!(concat!("unknown powerpc target feature: ", $t)) };
 
 
 /// PowerPC CPU Feature enum. Each variant denotes a position in a bitset

Mismatch at tests/source/cfg_if/detect/arch/x86.rs:84:
 #[allow_internal_unstable(stdsimd_internal, stdsimd)]
 macro_rules! is_x86_feature_detected {
     ("aes") => {
-        cfg!(target_feature = "aes") || $crate::detect::check_for($crate::detect::Feature::aes)
-    };
+        cfg!(target_feature = "aes") || $crate::detect::check_for(
+            $crate::detect::Feature::aes)  };
     ("pclmulqdq") => {
-        cfg!(target_feature = "pclmulqdq")
-            || $crate::detect::check_for($crate::detect::Feature::pclmulqdq)
-    };
+        cfg!(target_feature = "pclmulqdq") || $crate::detect::check_for(
+            $crate::detect::Feature::pclmulqdq)  };
     ("rdrand") => {
-        cfg!(target_feature = "rdrand")
-            || $crate::detect::check_for($crate::detect::Feature::rdrand)
-    };
+        cfg!(target_feature = "rdrand") || $crate::detect::check_for(
+            $crate::detect::Feature::rdrand)  };
     ("rdseed") => {
-        cfg!(target_feature = "rdseed")
-            || $crate::detect::check_for($crate::detect::Feature::rdseed)
-    };
+        cfg!(target_feature = "rdseed") || $crate::detect::check_for(
+            $crate::detect::Feature::rdseed)  };
     ("tsc") => {
-        cfg!(target_feature = "tsc") || $crate::detect::check_for($crate::detect::Feature::tsc)
-    };
+        cfg!(target_feature = "tsc") || $crate::detect::check_for(
+            $crate::detect::Feature::tsc)  };
     ("mmx") => {
-        cfg!(target_feature = "mmx") || $crate::detect::check_for($crate::detect::Feature::mmx)
-    };
+        cfg!(target_feature = "mmx") || $crate::detect::check_for(
+            $crate::detect::Feature::mmx)  };
     ("sse") => {
-        cfg!(target_feature = "sse") || $crate::detect::check_for($crate::detect::Feature::sse)
-    };
+        cfg!(target_feature = "sse") || $crate::detect::check_for(
+            $crate::detect::Feature::sse)  };
     ("sse2") => {
-        cfg!(target_feature = "sse2") || $crate::detect::check_for($crate::detect::Feature::sse2)
+        cfg!(target_feature = "sse2") || $crate::detect::check_for(
+            $crate::detect::Feature::sse2)
     };
     ("sse3") => {
-        cfg!(target_feature = "sse3") || $crate::detect::check_for($crate::detect::Feature::sse3)
+        cfg!(target_feature = "sse3") || $crate::detect::check_for(
+            $crate::detect::Feature::sse3)
     };
     ("ssse3") => {
-        cfg!(target_feature = "ssse3") || $crate::detect::check_for($crate::detect::Feature::ssse3)
+        cfg!(target_feature = "ssse3") || $crate::detect::check_for(
+            $crate::detect::Feature::ssse3)
     };
     ("sse4.1") => {
-        cfg!(target_feature = "sse4.1")
-            || $crate::detect::check_for($crate::detect::Feature::sse4_1)
+        cfg!(target_feature = "sse4.1") || $crate::detect::check_for(
+            $crate::detect::Feature::sse4_1)
     };
     ("sse4.2") => {
-        cfg!(target_feature = "sse4.2")
-            || $crate::detect::check_for($crate::detect::Feature::sse4_2)
+        cfg!(target_feature = "sse4.2") || $crate::detect::check_for(
+            $crate::detect::Feature::sse4_2)
     };
     ("sse4a") => {
-        cfg!(target_feature = "sse4a") || $crate::detect::check_for($crate::detect::Feature::sse4a)
+        cfg!(target_feature = "sse4a") || $crate::detect::check_for(
+            $crate::detect::Feature::sse4a)
     };
     ("sha") => {
-        cfg!(target_feature = "sha") || $crate::detect::check_for($crate::detect::Feature::sha)
+        cfg!(target_feature = "sha") || $crate::detect::check_for(
+            $crate::detect::Feature::sha)
     };
     ("avx") => {
-        cfg!(target_feature = "avx") || $crate::detect::check_for($crate::detect::Feature::avx)
+        cfg!(target_feature = "avx") || $crate::detect::check_for(
+            $crate::detect::Feature::avx)
     };
     ("avx2") => {
-        cfg!(target_feature = "avx2") || $crate::detect::check_for($crate::detect::Feature::avx2)
+        cfg!(target_feature = "avx2") || $crate::detect::check_for(
+            $crate::detect::Feature::avx2)
     };
     ("avx512f") => {
-        cfg!(target_feature = "avx512f")
-            || $crate::detect::check_for($crate::detect::Feature::avx512f)
+        cfg!(target_feature = "avx512f") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512f)
     };
     ("avx512cd") => {
-        cfg!(target_feature = "avx512cd")
-            || $crate::detect::check_for($crate::detect::Feature::avx512cd)
+        cfg!(target_feature = "avx512cd") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512cd)
     };
     ("avx512er") => {
-        cfg!(target_feature = "avx512er")
-            || $crate::detect::check_for($crate::detect::Feature::avx512er)
+        cfg!(target_feature = "avx512er") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512er)
     };
     ("avx512pf") => {
-        cfg!(target_feature = "avx512pf")
-            || $crate::detect::check_for($crate::detect::Feature::avx512pf)
+        cfg!(target_feature = "avx512pf") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512pf)
     };
     ("avx512bw") => {
-        cfg!(target_feature = "avx512bw")
-            || $crate::detect::check_for($crate::detect::Feature::avx512bw)
+        cfg!(target_feature = "avx512bw") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512bw)
     };
     ("avx512dq") => {
-        cfg!(target_feature = "avx512dq")
-            || $crate::detect::check_for($crate::detect::Feature::avx512dq)
+        cfg!(target_feature = "avx512dq") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512dq)
     };
     ("avx512vl") => {
-        cfg!(target_Feature = "avx512vl")
-            || $crate::detect::check_for($crate::detect::Feature::avx512vl)
+        cfg!(target_Feature = "avx512vl") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512vl)
     };
     ("avx512ifma") => {
-        cfg!(target_feature = "avx512ifma")
-            || $crate::detect::check_for($crate::detect::Feature::avx512_ifma)
+        cfg!(target_feature = "avx512ifma") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512_ifma)
     };
     ("avx512vbmi") => {
-        cfg!(target_feature = "avx512vbmi")
-            || $crate::detect::check_for($crate::detect::Feature::avx512_vbmi)
+        cfg!(target_feature = "avx512vbmi") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512_vbmi)
     };
     ("avx512vpopcntdq") => {
-        cfg!(target_feature = "avx512vpopcntdq")
-            || $crate::detect::check_for($crate::detect::Feature::avx512_vpopcntdq)
+        cfg!(target_feature = "avx512vpopcntdq") || $crate::detect::check_for(
+            $crate::detect::Feature::avx512_vpopcntdq)
     };
     ("f16c") => {
-        cfg!(target_feature = "f16c") || $crate::detect::check_for($crate::detect::Feature::f16c)
+        cfg!(target_feature = "f16c") || $crate::detect::check_for(
+            $crate::detect::Feature::f16c)
     };
     ("fma") => {
-        cfg!(target_feature = "fma") || $crate::detect::check_for($crate::detect::Feature::fma)
+        cfg!(target_feature = "fma") || $crate::detect::check_for(
+            $crate::detect::Feature::fma)
     };
     ("bmi1") => {
-        cfg!(target_feature = "bmi1") || $crate::detect::check_for($crate::detect::Feature::bmi)
+        cfg!(target_feature = "bmi1") || $crate::detect::check_for(
+            $crate::detect::Feature::bmi)
     };
     ("bmi2") => {
-        cfg!(target_feature = "bmi2") || $crate::detect::check_for($crate::detect::Feature::bmi2)
+        cfg!(target_feature = "bmi2") || $crate::detect::check_for(
+            $crate::detect::Feature::bmi2)
     };
     ("abm") => {
-        cfg!(target_feature = "abm") || $crate::detect::check_for($crate::detect::Feature::abm)
+        cfg!(target_feature = "abm") || $crate::detect::check_for(
+            $crate::detect::Feature::abm)
     };
     ("lzcnt") => {
-        cfg!(target_feature = "lzcnt") || $crate::detect::check_for($crate::detect::Feature::abm)
+        cfg!(target_feature = "lzcnt") || $crate::detect::check_for(
+            $crate::detect::Feature::abm)
     };
     ("tbm") => {
-        cfg!(target_feature = "tbm") || $crate::detect::check_for($crate::detect::Feature::tbm)
+        cfg!(target_feature = "tbm") || $crate::detect::check_for(
+            $crate::detect::Feature::tbm)
     };
     ("popcnt") => {
-        cfg!(target_feature = "popcnt")
-            || $crate::detect::check_for($crate::detect::Feature::popcnt)
+        cfg!(target_feature = "popcnt") || $crate::detect::check_for(
+            $crate::detect::Feature::popcnt)
     };
     ("fxsr") => {
-        cfg!(target_feature = "fxsr") || $crate::detect::check_for($crate::detect::Feature::fxsr)
+        cfg!(target_feature = "fxsr") || $crate::detect::check_for(
+            $crate::detect::Feature::fxsr)
     };
     ("xsave") => {
-        cfg!(target_feature = "xsave") || $crate::detect::check_for($crate::detect::Feature::xsave)
+        cfg!(target_feature = "xsave") || $crate::detect::check_for(
+            $crate::detect::Feature::xsave)
     };
     ("xsaveopt") => {
-        cfg!(target_feature = "xsaveopt")
-            || $crate::detect::check_for($crate::detect::Feature::xsaveopt)
+        cfg!(target_feature = "xsaveopt") || $crate::detect::check_for(
+            $crate::detect::Feature::xsaveopt)
     };
     ("xsaves") => {
-        cfg!(target_feature = "xsaves")
-            || $crate::detect::check_for($crate::detect::Feature::xsaves)
+        cfg!(target_feature = "xsaves") || $crate::detect::check_for(
+            $crate::detect::Feature::xsaves)
     };
     ("xsavec") => {
-        cfg!(target_feature = "xsavec")
-            || $crate::detect::check_for($crate::detect::Feature::xsavec)
+        cfg!(target_feature = "xsavec") || $crate::detect::check_for(
+            $crate::detect::Feature::xsavec)
     };
     ("cmpxchg16b") => {
-        cfg!(target_feature = "cmpxchg16b")
-            || $crate::detect::check_for($crate::detect::Feature::cmpxchg16b)
+        cfg!(target_feature = "cmpxchg16b") || $crate::detect::check_for(
+            $crate::detect::Feature::cmpxchg16b)
     };
     ("adx") => {
-        cfg!(target_feature = "adx") || $crate::detect::check_for($crate::detect::Feature::adx)
+        cfg!(target_feature = "adx") || $crate::detect::check_for(
+            $crate::detect::Feature::adx)
     };
     ("rtm") => {
-        cfg!(target_feature = "rtm") || $crate::detect::check_for($crate::detect::Feature::rtm)
+        cfg!(target_feature = "rtm") || $crate::detect::check_for(
+            $crate::detect::Feature::rtm)
     };
     ($t:tt,) => {
         is_x86_feature_detected!($t);
