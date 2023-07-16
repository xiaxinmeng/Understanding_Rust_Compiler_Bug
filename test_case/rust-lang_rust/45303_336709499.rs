diff
diff --git a/src/bootstrap/bootstrap.py b/src/bootstrap/bootstrap.py
index 89d297760e..598b0c1587 100644
--- a/src/bootstrap/bootstrap.py
+++ b/src/bootstrap/bootstrap.py
@@ -180,7 +180,7 @@ class RustBuild(object):
                 (not os.path.exists(self.cargo()) or self.cargo_out_of_date()):
             self.print_what_it_means_to_bootstrap()
             filename = "cargo-nightly-{}.tar.gz".format(self.build)
-            url = "https://s3.amazonaws.com/rust-lang-ci/cargo-builds/" + self.stage0_cargo_rev()
+            url = "https://rust-lang-ci2.s3.amazonaws.com/cargo-builds/" + self.stage0_cargo_rev()
             tarball = os.path.join(cargo_cache, filename)
             if not os.path.exists(tarball):
                 get("{}/{}".format(url, filename), tarball, verbose=self.verbose)
