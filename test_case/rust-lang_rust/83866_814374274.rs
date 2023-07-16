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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/lib.rs at line 599:
     }
     println!("{}", options.usage(&format!("{} [options] <input>", argv0)));
     println!("    @path               Read newline separated options from `path`\n");
-    println!("More information available at https://doc.rust-lang.org/{}/rustdoc/what-is-rustdoc.html", doc_rust_lang_org_channel());
+    println!(
+        "More information available at https://doc.rust-lang.org/{}/rustdoc/what-is-rustdoc.html",
+        doc_rust_lang_org_channel()
+    );
 
 
 /// A result type used by several functions under `main()`.
Diff in /checkout/src/librustdoc/core.rs at line 498:
     let mut krate = tcx.sess.time("clean_crate", || clean::krate(&mut ctxt));
 
     if krate.module.doc_value().map(|d| d.is_empty()).unwrap_or(true) {
-        let help = format!("The following guide may be of use:\n\
+        let help = format!(
+            "The following guide may be of use:\n\
             https://doc.rust-lang.org/{}/rustdoc/how-to-write-documentation.html",
             crate::doc_rust_lang_org_channel(),
         );
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/theme.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/error.rs" "/checkout/src/librustdoc/doctest/tests.rs" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/librustdoc/passes/doc_test_lints.rs" "/checkout/src/librustdoc/theme/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
Diff in /checkout/src/librustdoc/clean/types.rs at line 882:
                                 }
                                 }
                                 Some(&(_, _, ExternalLocation::Remote(ref s))) => s.to_string(),
                                 Some(&(_, _, ExternalLocation::Unknown)) | None => format!(
-                                    "https://doc.rust-lang.org/{}", crate::doc_rust_lang_org_channel(),
-                                )
+                                    "https://doc.rust-lang.org/{}",
+                                    crate::doc_rust_lang_org_channel(),
                             };
                             };
                             // This is a primitive so the url is done "by hand".
                             let tail = fragment.find('#').unwrap_or_else(|| fragment.len());
