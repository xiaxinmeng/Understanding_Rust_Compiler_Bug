plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-10
##[endgroup]
Found differing commits between master and beta.
diff --git a/src/ci/scripts/cherries/beta-master b/src/ci/scripts/cherries/beta-master
index b7ddc693504..a2b1487c845 100644
--- a/src/ci/scripts/cherries/beta-master
+++ b/src/ci/scripts/cherries/beta-master
@@ -1,3 +1,4 @@
++ 06f454932e31c05d58e299e4ee769ca21cb11701
 + 2b2bf6c88a0d9bf72534cc4ce1a22971af77b137
 + 56397e90e178fdc98b48031ceb79e90c61a48e3a
 - 87d2c5934d5d7c8331b3859d5ed9b5523b99bb0e
##[error]Process completed with exit code 1.
