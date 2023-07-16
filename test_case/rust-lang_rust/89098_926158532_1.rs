console
Traceback (most recent call last):
  File "/usr/lib/python3.9/xml/etree/ElementPath.py", line 354, in iterfind
    selector = _cache[cache_key]
KeyError: ('.//*[@id="implementors-list"]//span[@class="where fmt-newline"]/text()[1]',)

During handling of the above exception, another exception occurred:

Traceback (most recent call last):
  File "/home/imperio/rust/rust/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/home/imperio/rust/rust/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/home/imperio/rust/rust/src/etc/htmldocck.py", line 439, in check_command
    ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
  File "/home/imperio/rust/rust/src/etc/htmldocck.py", line 370, in check_tree_text
    for e in tree.findall(path):
  File "/usr/lib/python3.9/xml/etree/ElementPath.py", line 395, in findall
    return list(iterfind(elem, path, namespaces))
  File "/usr/lib/python3.9/xml/etree/ElementPath.py", line 368, in iterfind
    selector.append(ops[token[0]](next, token))
KeyError: '()'
