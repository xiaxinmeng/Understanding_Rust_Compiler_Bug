
<anon>:26:8: 26:9 error: use of moved value: `v` [E0382]
<anon>:26     in v {42};
                 ^
<anon>:26:8: 26:9 help: see the detailed explanation for E0382
<anon>:25:8: 25:9 note: `v` moved here because it has type `MyVec<i32>`, which is non-copyable
<anon>:25     in v {42i32};
                 ^
