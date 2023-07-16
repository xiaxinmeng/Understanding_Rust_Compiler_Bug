diff
diff --git a/Cargo.lock b/Cargo.lock
index c071a2e11d..9b7be64c88 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -2053,18 +2053,6 @@ dependencies = [
  "winapi 0.3.6 (registry+https://github.com/rust-lang/crates.io-index)",
 ]
 
-[[package]]
-name = "rand"
-version = "0.5.5"
-source = "registry+https://github.com/rust-lang/crates.io-index"
-dependencies = [
- "cloudabi 0.0.3 (registry+https://github.com/rust-lang/crates.io-index)",
- "fuchsia-zircon 0.3.3 (registry+https://github.com/rust-lang/crates.io-index)",
- "libc 0.2.51 (registry+https://github.com/rust-lang/crates.io-index)",
- "rand_core 0.2.2 (registry+https://github.com/rust-lang/crates.io-index)",
- "winapi 0.3.6 (registry+https://github.com/rust-lang/crates.io-index)",
-]
-
 [[package]]
 name = "rand"
 version = "0.6.1"
@@ -2092,14 +2080,6 @@ dependencies = [
  "rustc_version 0.2.3 (registry+https://github.com/rust-lang/crates.io-index)",
 ]
 
-[[package]]
-name = "rand_core"
-version = "0.2.2"
-source = "registry+https://github.com/rust-lang/crates.io-index"
-dependencies = [
- "rand_core 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)",
-]
-
 [[package]]
 name = "rand_core"
 version = "0.3.0"
@@ -2559,7 +2539,7 @@ version = "1.0.0"
 dependencies = [
  "byteorder 1.2.7 (registry+https://github.com/rust-lang/crates.io-index)",
  "parking_lot 0.7.1 (registry+https://github.com/rust-lang/crates.io-index)",
- "rand 0.5.5 (registry+https://github.com/rust-lang/crates.io-index)",
+ "rand 0.6.1 (registry+https://github.com/rust-lang/crates.io-index)",
  "scopeguard 0.3.3 (registry+https://github.com/rust-lang/crates.io-index)",
  "serde 1.0.82 (registry+https://github.com/rust-lang/crates.io-index)",
  "serde_json 1.0.33 (registry+https://github.com/rust-lang/crates.io-index)",
@@ -4204,10 +4184,8 @@ source = "registry+https://github.com/rust-lang/crates.io-index"
 "checksum quote 0.6.10 (registry+https://github.com/rust-lang/crates.io-index)" = "53fa22a1994bd0f9372d7a816207d8a2677ad0325b073f5c5332760f0fb62b5c"
 "checksum racer 2.1.21 (registry+https://github.com/rust-lang/crates.io-index)" = "37c88638777cc178684cf648ca0e1dad56646ce105b8593dfe665c436300adc3"
 "checksum rand 0.4.3 (registry+https://github.com/rust-lang/crates.io-index)" = "8356f47b32624fef5b3301c1be97e5944ecdd595409cc5da11d05f211db6cfbd"
-"checksum rand 0.5.5 (registry+https://github.com/rust-lang/crates.io-index)" = "e464cd887e869cddcae8792a4ee31d23c7edd516700695608f5b98c67ee0131c"
 "checksum rand 0.6.1 (registry+https://github.com/rust-lang/crates.io-index)" = "ae9d223d52ae411a33cf7e54ec6034ec165df296ccd23533d671a28252b6f66a"
 "checksum rand_chacha 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)" = "771b009e3a508cb67e8823dda454aaa5368c7bc1c16829fb77d3e980440dd34a"
-"checksum rand_core 0.2.2 (registry+https://github.com/rust-lang/crates.io-index)" = "1961a422c4d189dfb50ffa9320bf1f2a9bd54ecb92792fb9477f99a1045f3372"
 "checksum rand_core 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)" = "0905b6b7079ec73b314d4c748701f6931eb79fd97c668caa3f1899b22b32c6db"
 "checksum rand_hc 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)" = "7b40677c7be09ae76218dc623efbf7b18e34bced3f38883af07bb75630a21bc4"
 "checksum rand_isaac 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)" = "ded997c9d5f13925be2a6fd7e66bf1872597f759fd9dd93513dd7e92e5a5ee08"
