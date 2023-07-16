plain
failures:

---- [rustdoc] src/test/rustdoc/deprecated-future-staged-api.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-future-staged-api" "/checkout/src/test/rustdoc/deprecated-future-staged-api.rs"
stdout: none
--- stderr -------------------------------
4: @has check failed
 File does not exist 'deprecated-future-staged-api/index.html'
 // @has deprecated-future-staged-api/index.html '//*[@class="stab deprecated"]' 'Deprecation planned'
6: @has check failed
 File does not exist 'deprecated-future-staged-api/struct.S1.html'
 // @has deprecated-future-staged-api/struct.S1.html '//*[@class="stab deprecated"]' 'Deprecating in 99.99.99: effectively never'
12: @has check failed
 File does not exist 'deprecated-future-staged-api/index.html'
 // @has deprecated-future-staged-api/index.html '//*[@class="stab deprecated"]' 'Deprecation planned'
14: @has check failed
 File does not exist 'deprecated-future-staged-api/struct.S2.html'
 // @has deprecated-future-staged-api/struct.S2.html '//*[@class="stab deprecated"]' 'Deprecating in a future Rust version: literally never'
Encountered 4 errors
------------------------------------------


