plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 543:
                         // The file was present and created by the same compiler version, but we
                         // couldn't load it for some reason.  Give a hard error instead of silently
                         // ignoring it, but only if we would have given an error anyway.
-                        self.crate_rejections.via_invalid.push(CrateMismatch { path: lib, got: err });
+                        self.crate_rejections
+                            .via_invalid
+                            .push(CrateMismatch { path: lib, got: err });
                     }
                     }
                     Err(err @ MetadataError::NotPresent(_)) => {
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 731:
         return Err(MetadataError::NotPresent(filename));
     }
     let raw_bytes: MetadataRef = match flavor {
-        CrateFlavor::Rlib => loader.get_rlib_metadata(target, filename).map_err(MetadataError::LoadFailure)?,
+        CrateFlavor::Rlib => {
+            loader.get_rlib_metadata(target, filename).map_err(MetadataError::LoadFailure)?
+        }
         CrateFlavor::Dylib => {
-            let buf = loader.get_dylib_metadata(target, filename).map_err(MetadataError::LoadFailure)?;
+            let buf =
+                loader.get_dylib_metadata(target, filename).map_err(MetadataError::LoadFailure)?;
             // The header is uncompressed
             let header_len = METADATA_HEADER.len();
             debug!("checking {} bytes of metadata-version stamp", header_len);
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 740:
             let header = &buf[..cmp::min(header_len, buf.len())];
             if header != METADATA_HEADER {
-                return Err(MetadataError::LoadFailure(format!("invalid metadata version found: {}", filename.display())));
+                return Err(MetadataError::LoadFailure(format!(
+                    "invalid metadata version found: {}",
+                    filename.display()
+                )));
 
 
             // Header is okay -> inflate the actual metadata
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 749:
             match FrameDecoder::new(compressed_bytes).read_to_end(&mut inflated) {
                 Ok(_) => rustc_erase_owner!(OwningRef::new(inflated).map_owner_box()),
                 Err(_) => {
-                    return Err(
-                        MetadataError::LoadFailure(format!("failed to decompress metadata: {}", filename.display()))
-                    );
+                    return Err(MetadataError::LoadFailure(format!(
+                        "failed to decompress metadata: {}",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_metadata/src/rmeta/mod.rs" "/checkout/compiler/rustc_metadata/src/dynamic_lib.rs" "/checkout/compiler/rustc_metadata/src/rmeta/decoder.rs" "/checkout/compiler/rustc_metadata/src/foreign_modules.rs" "/checkout/compiler/rustc_metadata/src/dependency_format.rs" "/checkout/compiler/rustc_metadata/src/creader.rs" "/checkout/compiler/rustc_metadata/src/locator.rs" "/checkout/compiler/rustc_metadata/src/rmeta/table.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                        filename.display()
+                    )));
             }
         }
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 758:
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 758:
         CrateFlavor::Rmeta => {
             // mmap the file, because only a small fraction of it is read.
-            let file = std::fs::File::open(filename)
-                .map_err(|_| MetadataError::LoadFailure(format!("failed to open rmeta metadata: '{}'", filename.display())))?;
+            let file = std::fs::File::open(filename).map_err(|_| {
+                MetadataError::LoadFailure(format!(
+                    "failed to open rmeta metadata: '{}'",
+                    filename.display()
+            })?;
+            })?;
             let mmap = unsafe { Mmap::map(file) };
-            let mmap = mmap
-                .map_err(|_| MetadataError::LoadFailure(format!("failed to mmap rmeta metadata: '{}'", filename.display())))?;
+            let mmap = mmap.map_err(|_| {
+                MetadataError::LoadFailure(format!(
+                    "failed to mmap rmeta metadata: '{}'",
+                    filename.display()
+            })?;
 
 
             rustc_erase_owner!(OwningRef::new(mmap).map_owner_box())
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 770:
Diff in /checkout/compiler/rustc_metadata/src/locator.rs at line 770:
     if blob.is_compatible() {
         Ok(blob)
     } else {
-        Err(MetadataError::LoadFailure(format!("invalid metadata version found: {}", filename.display())))
+        Err(MetadataError::LoadFailure(format!(
+            "invalid metadata version found: {}",
+            filename.display()
+        )))
 }
 
Build completed unsuccessfully in 0:00:12
