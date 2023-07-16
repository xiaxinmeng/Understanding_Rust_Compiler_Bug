
[00:54:41] ---- [rustdoc] rustdoc/doc-spotlight.rs stdout ----
[00:54:41] 	
[00:54:41] error: htmldocck failed!
[00:54:41] status: exit code: 1
[00:54:41] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-spotlight.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/doc-spotlight.rs"
[00:54:41] stdout:
[00:54:41] ------------------------------------------
[00:54:41] 
[00:54:41] ------------------------------------------
[00:54:41] stderr:
[00:54:41] ------------------------------------------
[00:54:41] Traceback (most recent call last):
[00:54:41]   File "/checkout/src/etc/htmldocck.py", line 455, in <module>
[00:54:41]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:54:41]   File "/checkout/src/etc/htmldocck.py", line 448, in check
[00:54:41]     check_command(c, cache)
[00:54:41]   File "/checkout/src/etc/htmldocck.py", line 397, in check_command
[00:54:41]     tree = cache.get_tree(c.args[0])
[00:54:41]   File "/checkout/src/etc/htmldocck.py", line 309, in get_tree
[00:54:41]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:54:41] RuntimeError: Cannot parse an HTML file 'doc_spotlight/trait.SomeTrait.html': pop from empty stack
[00:54:41] 
[00:54:41] ------------------------------------------
