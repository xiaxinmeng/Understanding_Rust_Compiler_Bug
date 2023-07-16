
<anon>:6:11: 6:17 error: no method named `insert` found for type `fn() -> std::collections::BTreeMap<_, _> {<std::collections::BTreeMap<K, V>><_, _>::new}` in the current scope
<anon>:6         q.insert(i, [0u8; 65536]);
                   ^~~~~~
<anon>:6:9: 6:10 note: q is a function, perhaps you wish to call it
<anon>:6:9: 6:10 help: try calling the base function:
<anon>:          q().insert(i, [0u8; 65536]);
error: aborting due to previous error
