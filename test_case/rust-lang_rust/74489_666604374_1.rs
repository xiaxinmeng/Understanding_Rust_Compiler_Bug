
$ time rustdoc +stage1 /home/joshua/rustc/src/test/rustdoc/intra-link-associated-items.rs

real	0m0.757s
user	0m0.669s
sys	0m0.069s
$ diff /home/joshua/rustc/src/test/rustdoc/intra-link-associated-items.rs no-assoc-items.rs 
4,5c4,5
< /// [`std::collections::BTreeMap::into_iter`]
< /// [`String::from`] is ambiguous as to which `From` impl
---
> /// [`std::collections::BTreeMap`]
> /// [`String`] is ambiguous as to which `From` impl
10c10
< /// Link to [MyStruct], [link from struct][MyStruct::method], [MyStruct::clone], [MyStruct::Input]
---
> /// Link to [MyStruct], [link from struct][MyStruct], [MyStruct], [MyStruct]
31c31
<     /// [link from method][MyStruct::method] on method
---
>     /// [link from method][MyStruct] on method
46c46
< /// Link to [S::ambiguous_method]
---
> /// Link to [S]
