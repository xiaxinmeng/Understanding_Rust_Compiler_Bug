
diff --git a/ci/reprotest.sh b/ci/reprotest.sh
index 5bdd378..ecc48b7 100755
--- a/ci/reprotest.sh
+++ b/ci/reprotest.sh
@@ -8,8 +8,10 @@ set -xue
 export TMPDIR="$HOME/tmp/repro-test"
 mkdir -p "$TMPDIR"
 
+rustup install "nightly-2017-12-$1"
+
 reprotest -vv --vary=-time,-domain_host --source-pattern 'Cargo.* src/' '
-    RUSTC_BOOTSTRAP=1 CARGO_HOME="$PWD/.cargo" RUSTUP_HOME='"$HOME/.rustup"' \
+    RUSTC_BOOTSTRAP=1 CARGO_HOME="'$HOME'/.cargo" RUSTUP_HOME='"$HOME/.rustup"' \
         RUSTFLAGS="-Zremap-path-prefix-from=$HOME -Zremap-path-prefix-to=/remap-home -Zremap-path-prefix-from=$PWD -Zremap-path-prefix-to=/remap-pwd" \
-        cargo build --release --verbose' \
+        rustup run nightly-2017-12-'$1' cargo build --release --verbose' \
     target/release/sniffglue
