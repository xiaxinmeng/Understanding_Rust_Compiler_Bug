plain
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] src/test/rustdoc/infinite-redirection.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/infinite-redirection" "/checkout/src/test/rustdoc/infinite-redirection.rs"
stdout: none
--- stderr -------------------------------
7: @has check failed
 File does not exist 'foo/builders/struct.ActionRowBuilder.html'
 // @has 'foo/builders/struct.ActionRowBuilder.html'
8: @has check failed
 File does not exist 'foo/builders/struct.ActionRowBuilder.html'
 // @has - '//*[@id="synthetic-implementations"]' 'Auto Trait Implementations'
11: @has check failed
 File does not exist 'foo/builders/index.html'
 // @has 'foo/builders/index.html'
12: @has check failed
 File does not exist 'foo/builders/index.html'
 // @has - '//a[@href="struct.ActionRowBuilder.html"]' 'ActionRowBuilder'
Encountered 4 errors
------------------------------------------


