console
Diff in /checkout/library/core/src/str/validations.rs at line 125:
         let old_offset = index;
         macro_rules! err {
             ($error_len: expr) => {
-                return Err(Utf8Error { valid_up_to: old_offset, error_len: $error_len })
+                return Err(Utf8Error { valid_up_to: old_offset, error_len: $error_len });
             };
         }

Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/str/validations.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
