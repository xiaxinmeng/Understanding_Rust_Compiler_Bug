plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/tools/collect-license-metadata/src/path_tree.rs at line 186:
                 }
                 childs.retain(|child| !matches!(child, Node::Empty));
-            Node::File { .. } => {},
-            Node::Empty => {},
+            Node::File { .. } => {}
+            Node::Empty => {}
+            Node::Empty => {}
         }
     }
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tier-check/src/main.rs" "/checkout/src/tools/rustbook/src/main.rs" "/checkout/src/bootstrap/setup.rs" "/checkout/src/bootstrap/install.rs" "/checkout/src/tools/error_index_generator/main.rs" "/checkout/src/tools/collect-license-metadata/src/path_tree.rs" "/checkout/src/tools/collect-license-metadata/src/licenses.rs" "/checkout/src/bootstrap/metrics.rs"` failed.
Build completed unsuccessfully in 0:00:10
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
