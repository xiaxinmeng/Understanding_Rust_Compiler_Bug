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
             }
         }
 
-        f.write_str(
-            "<h1 class=\"fqn\">List of all items</h1>",
-        );
+        f.write_str("<h1 class=\"fqn\">List of all items</h1>");
         // Note: print_entries does not escape the title, because we know the current set of titles
         // doesn't require escaping.
         print_entries(f, &self.structs, ItemSection::Structs);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/formats/item_type.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/formats/mod.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/formats/cache.rs" "/checkout/src/librustdoc/html/render/search_index.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/clean/types/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
