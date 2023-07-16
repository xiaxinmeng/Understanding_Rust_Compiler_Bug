plain

Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [rustdoc] rustdoc/rfc-2632-const-trait-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/rfc-2632-const-trait-impl" "/checkout/src/test/rustdoc/rfc-2632-const-trait-impl.rs"
stdout: none
--- stderr -------------------------------
14: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//pre[@class="rust trait"]/code/a[@class="trait"]' 'Drop'
17: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//pre[@class="rust trait"]/code/span[@class="where"]' 'Drop'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//pre[@class="rust trait"]/code/span[@class="where"]' ': Clone'
21: @!has check failed
 `XPATH PATTERN` did not match
     // @!has - '//div[@id="method.a"]/h4[@class="code-header"]/a[@class="trait"]' 'Drop'
24: @!has check failed
 `XPATH PATTERN` did not match
     // @!has - '//div[@id="method.a"]/h4[@class="code-header"]/span[@class="where fmt-newline"]' 'Drop'
25: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//div[@id="method.a"]/h4[@class="code-header"]/span[@class="where fmt-newline"]' ': Clone'
31: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//section[@id="impl-Tr%3CT%3E"]/h3[@class="code-header in-band"]/a[@class="trait"]' 'Drop'
34: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//section[@id="impl-Tr%3CT%3E"]/h3[@class="code-header in-band"]/span[@class="where fmt-newline"]' 'Drop'
35: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//section[@id="impl-Tr%3CT%3E"]/h3[@class="code-header in-band"]/span[@class="where fmt-newline"]' ': Clone'
41: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//pre[@class="rust fn"]/code/a[@class="trait"]' 'Drop'
44: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//pre[@class="rust fn"]/code/span[@class="where fmt-newline"]' 'Drop'
45: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//pre[@class="rust fn"]/code/span[@class="where fmt-newline"]' ': Clone'
52: @!has check failed
 `XPATH PATTERN` did not match
     // @!has - '//section[@id="method.foo"]/h4[@class="code-header"]/a[@class="trait"]' 'Drop'
55: @!has check failed
 `XPATH PATTERN` did not match
     // @!has - '//section[@id="method.foo"]/h4[@class="code-header"]/span[@class="where fmt-newline"]' 'Drop'
56: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//section[@id="method.foo"]/h4[@class="code-header"]/span[@class="where fmt-newline"]' ': Clone'
Encountered 15 errors
------------------------------------------


