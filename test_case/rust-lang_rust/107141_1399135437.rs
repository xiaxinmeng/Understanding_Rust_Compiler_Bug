plain
Testing parser-returned.js ... OK
Testing parser-separators.js ... OK
Testing parser-weird-queries.js ... OK
Testing path-ordering.js ... FAILED
==> '{"path":"std::collections::hash_set::HashSet","name":"get_or_insert"}' was supposed to be before '{"crate":"std","ty":11,"name":"get_or_insert","path":"std::collections::hash_set","desc":"Inserts the given <code>value</code> into the set if it is not present, …","parent":{"ty":3,"name":"HashSet"},"type":{"inputs":[{"name":"hashset","ty":3,"generics":[]}],"output":[]},"id":44423,"normalizedName":"getorinsert","lev":7,"displayPath":"<span>std::</span><span>collections::</span><span>hash_set::</span><span>HashSet::</span>","fullPath":"<span>std::</span><span>collections::</span><span>hash_set::</span><span>HashSet::</span>get_or_insert|11","href":"../std/collections/hash_set/struct.HashSet.html#method.get_or_insert"}'
==> '{"path":"std::collections::hash_set::HashSet","name":"get_or_insert_with"}' was supposed to be before '{"crate":"std","ty":11,"name":"get_or_insert_with","path":"std::collections::hash_set","desc":"Inserts a value computed from <code>f</code> into the set if the given …","parent":{"ty":3,"name":"HashSet"},"type":{"inputs":[{"name":"hashset","ty":3,"generics":[]}],"output":[]},"id":44425,"normalizedName":"getorinsertwith","lev":12,"displayPath":"<span>std::</span><span>collections::</span><span>hash_set::</span><span>HashSet::</span>","fullPath":"<span>std::</span><span>collections::</span><span>hash_set::</span><span>HashSet::</span>get_or_insert_with|11","href":"../std/collections/hash_set/struct.HashSet.html#method.get_or_insert_with"}'
==> '{"path":"std::collections::hash_set::HashSet","name":"get_or_insert_owned"}' was supposed to be before '{"crate":"std","ty":11,"name":"get_or_insert_owned","path":"std::collections::hash_set","desc":"Inserts an owned copy of the given <code>value</code> into the set if …","parent":{"ty":3,"name":"HashSet"},"type":{"inputs":[{"name":"hashset","ty":3,"generics":[]}],"output":[]},"id":44424,"normalizedName":"getorinsertowned","lev":13,"displayPath":"<span>std::</span><span>collections::</span><span>hash_set::</span><span>HashSet::</span>","fullPath":"<span>std::</span><span>collections::</span><span>hash_set::</span><span>HashSet::</span>get_or_insert_owned|11","href":"../std/collections/hash_set/struct.HashSet.html#method.get_or_insert_owned"}'
==> '{"path":"std::collections::hash_map::HashMap","name":"insert"}' was supposed to be before '{"crate":"std","ty":11,"name":"insert","path":"std::collections::hash_map","desc":"Inserts a key-value pair into the map.","parent":{"ty":3,"name":"HashMap"},"type":{"inputs":[{"name":"hashmap","ty":3,"generics":[]}],"output":[{"name":"option","ty":4,"generics":[]}]},"id":44148,"normalizedName":"insert","lev":0,"displayPath":"<span>std::</span><span>collections::</span><span>hash_map::</span><span>HashMap::</span>","fullPath":"<span>std::</span><span>collections::</span><span>hash_map::</span><span>HashMap::</span>insert|11","href":"../std/collections/hash_map/struct.HashMap.html#method.insert"}'
Testing quoted.js ... OK
Testing return-specific-literal.js ... OK
Testing return-specific.js ... OK
Testing should-fail.js ... OK
Testing should-fail.js ... OK
Testing string-from_ut.js ... FAILED
==> '{"path":"std::string::String","name":"from_utf16_lossy"}' was supposed to be before '{"crate":"std","ty":11,"name":"from_utf16_lossy","path":"std::string","desc":"Decode a UTF-16–encoded slice <code>v</code> into a <code>String</code>, replacing …","parent":{"ty":3,"name":"String"},"type":{"inputs":[],"output":[{"name":"string","ty":3,"generics":[]}]},"id":56029,"normalizedName":"fromutf16lossy","lev":9,"displayPath":"<span>std::</span><span>string::</span><span>String::</span>","fullPath":"<span>std::</span><span>string::</span><span>String::</span>from_utf16_lossy|11","href":"../std/string/struct.String.html#method.from_utf16_lossy"}'
Testing typed-query.js ... FAILED
Testing typed-query.js ... FAILED
==> Expected exactly 5 results but found 4 in 'others'
==> Result not found in 'others': '{"path":"std::pin","name":"pin"}'
Diff of first error:
-     "path": "std::pin",
+     "path": "std",
+     "path": "std",
-     "name": "pin",
+     "name": "print",
Testing vec-new.js ... FAILED
Testing vec-new.js ... FAILED
==> Result not found in 'others': '{"path":"std::vec::Vec","name":"new"}'
Diff of first error:
-     "path": "std::vec::Vec",
+     "path": "alloc::alloc",
+     "path": "alloc::alloc",
     "name": "new",
 }
==> Result not found in 'others': '{"path":"std::vec::Vec","name":"new_in"}'
Diff of first error:
-     "path": "std::vec::Vec",
-     "path": "std::vec::Vec",
+     "path": "alloc::boxed",
-     "name": "new_in",
+     "name": "new",
Build completed unsuccessfully in 0:27:50
