plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1974:
                     .iter()
                     .filter_map(|it| {
                         let trait_ = it.inner_impl().trait_.as_ref()?;
-                        let encoded = id_map.derive(get_id_for_impl(&it.inner_impl().for_, Some(trait_), cx));
+                        let encoded =
+                            id_map.derive(get_id_for_impl(&it.inner_impl().for_, Some(trait_), cx));
 
                         let i_display = format!("{:#}", trait_.print(cx));
                         let out = Escape(&i_display);
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1982:
                             ty::ImplPolarity::Positive | ty::ImplPolarity::Reservation => "",
                             ty::ImplPolarity::Negative => "!",
-                        let generated =
-                        let generated =
-                            format!("<a href=\"#{}\">{}{}</a>", encoded, prefix, out);
+                        let generated = format!("<a href=\"#{}\">{}{}</a>", encoded, prefix, out);
                         if links.insert(generated.clone()) { Some(generated) } else { None }
                     .collect::<Vec<String>>();
                     .collect::<Vec<String>>();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/span_map.rs" "/checkout/src/librustdoc/html/render/search_index.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/toc.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/librustdoc/html/url_parts_builder/tests.rs" "/checkout/src/librustdoc/html/render/write_shared.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
