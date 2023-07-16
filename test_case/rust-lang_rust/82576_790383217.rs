plain
Step 5/10 : RUN npm install es-check -g
 ---> Running in 4097243468c4
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.3
added 95 packages from 44 contributors in 3.7s
Removing intermediate container 4097243468c4
 ---> 266eb9051b1e
---
Successfully built eebd9615baba
Successfully tagged rust-ci:latest
Built container sha256:eebd9615babaedc00ff687b689e606d8df6fff355a988f0bc8b94abf14b80f8d
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
Diff in /checkout/library/alloc/src/string.rs at line 2225:
     }
 }
 
-#[stable(feature = "u8_to_string_specialization", since="1.999.0")]
+#[stable(feature = "u8_to_string_specialization", since = "1.999.0")]
 impl ToString for u8 {
     #[inline]
     fn to_string(&self) -> String {
Diff in /checkout/library/alloc/src/string.rs at line 2251:
       8081828384858687888990919293949596979899";
 
 
-#[stable(feature = "i8_to_string_specialization", since="1.999.0")]
+#[stable(feature = "i8_to_string_specialization", since = "1.999.0")]
 impl ToString for i8 {
     #[inline]
     fn to_string(&self) -> String {
Diff in /checkout/library/alloc/src/string.rs at line 2258:
-        let mut vec : Vec<u8> = if *self < 0 {
+        let mut vec: Vec<u8> = if *self < 0 {
             let mut v = Vec::with_capacity(4);
             v.push(b'-');
Diff in /checkout/library/alloc/src/string.rs at line 2271:
Diff in /checkout/library/alloc/src/string.rs at line 2271:
             let nn = n * 2;
             vec.extend_from_slice(&DEC_DIGITS_LUT[nn as usize..=nn as usize + 1]);
         } else {
-            vec.push(b'0' + (*self as u8) );
+            vec.push(b'0' + (*self as u8));
-        unsafe {
-            String::from_utf8_unchecked(vec)
-        }
-        }
+        unsafe { String::from_utf8_unchecked(vec) }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/benches/vec_deque.rs" "/checkout/library/alloc/benches/lib.rs" "/checkout/library/alloc/benches/btree/map.rs" "/checkout/library/alloc/tests/fmt.rs" "/checkout/library/alloc/benches/btree/set.rs" "/checkout/library/alloc/tests/string.rs" "/checkout/library/alloc/src/string.rs" "/checkout/library/alloc/benches/str.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
