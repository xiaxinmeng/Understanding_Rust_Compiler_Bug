
[01:34:34] 
[01:34:34] failures:
[01:34:34] 
[01:34:34] ---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----
[01:34:34] 	
[01:34:34] error: htmldocck failed!
[01:34:34] status: exit code: 1
[01:34:34] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/i686-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex.stage2-i686-unknown-linux-gnu" "/checkout/src/test/rustdoc/synthetic_auto/complex.rs"
[01:34:34] stdout:
[01:34:34] ------------------------------------------
[01:34:34] 
[01:34:34] ------------------------------------------
[01:34:34] stderr:
[01:34:34] ------------------------------------------
[01:34:34] 33: @has check failed
[01:34:34] 	`XPATH PATTERN` did not match
[01:34:34] 	// @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]/*/code' "impl<'a, T, K: ?Sized> Send for NotOuter<'a, T, K> where 'a: 'static, K: for<'b> Fn((&'b bool, &'a u8)) -> &'b i8, <T as MyTrait<'a>>::MyItem: Copy,  T: MyTrait<'a>"
[01:34:34] 
[01:34:34] Encountered 1 errors
[01:34:34] 
[01:34:34] ------------------------------------------
[01:34:34] 
[01:34:34] thread '[rustdoc] rustdoc/synthetic_auto/complex.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2891:9
[01:34:34] 
[01:34:34] 
[01:34:34] failures:
[01:34:34]     [rustdoc] rustdoc/synthetic_auto/complex.rs
