
[22:16:45] <reem> eibwen: I thought the decision was to keep types in Debug output
[22:17:19] <eibwen> oh, was it?
[22:17:24] <eibwen> I'll dig up the RFC
[22:17:32] <eibwen> I thought it was the other way around
[22:18:15] <eibwen> reem: from the RFC: Focus on the *runtime* aspects of a type; repeating information such as suffixes for integer literals is not generally useful since that data is readily available from the type definition.
[22:19:03] <eibwen> reem: further down: Containers print using *some* notation that makes their type and contents clear. (Since we lack literals for all container types, this will be ad hoc).
[22:19:17] <eibwen> this is indicated by using [] for sequences and {} for sets/maps
[22:19:30] <reem> eibwen: [] or {} is not enough to tell me the tyep
[22:19:37] <reem> if I want to know the type I need a name too
[22:20:15] <eibwen> I might well have misunderstood the RFC, but I interpret it as "focus on the semantics of the type, not its implementation"
[22:20:49] <eibwen> which is: "sequence" for Vec, DList, etc. (the actual type can be seen in source code, the semantic type is there)
[22:21:00] <eibwen> or "map" for HashMap, TreeMap, etc.
[22:21:55] <bluss_> I like it. I think it composes better with derived Debug impls of custom types? (But those still use struct names(?))
[22:22:25] <reem> I still think the types should stay in the output
[22:22:47] <eibwen> I think the terser the output, the better
[22:22:52] <eibwen> as long as it's readable
[22:23:02] <eibwen> because more things fit on the screen
[22:23:25] <eibwen> (this has been an actual nuisance for me, if you nest a few structs, the debug output explodes)
