
@@ -102,9 +103,10 @@ fn verify(root: &Path, data: &Path, sig: &Path) -> bool {
     let path = root.push("gpg");
     let res = gpgv(~[~"--homedir", path.to_str(),
                   ~"--keyring", ~"pubring.gpg",
-                  ~"--verbose",
+                  ~"--verify",
                  sig.to_str(), data.to_str()]);
