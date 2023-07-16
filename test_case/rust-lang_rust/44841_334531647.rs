
...
[00:46:00] ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
[00:46:00] 	
[00:46:00] The following items were assigned to wrong codegen units:
[00:46:00] 
[00:46:00] fn core::ptr[0]::drop_in_place[0]<u32>
[00:46:00]   expected: vtable_through_const[Internal] 
[00:46:00]   actual:   vtable_through_const.cgu-m.[Internal] 
[00:46:00] 
[00:46:00] fn vtable_through_const::main[0]
[00:46:00]   expected: vtable_through_const[Internal] 
[00:46:00]   actual:   vtable_through_const.cgu-m.[Internal] 
[00:46:00] 
[00:46:00] fn vtable_through_const::mod1[0]::Trait1[0]::do_something[0]<u32>
[00:46:00]   expected: vtable_through_const-mod1.volatile[External] 
[00:46:00]   actual:   vtable_through_const.cgu-m.-mod1.volatile[External] 
[00:46:00] 
[00:46:00] fn vtable_through_const::mod1[0]::Trait1[0]::do_something_else[0]<u32>
[00:46:00]   expected: vtable_through_const-mod1.volatile[External] 
[00:46:00]   actual:   vtable_through_const.cgu-m.-mod1.volatile[External] 
[00:46:00] 
[00:46:00] fn vtable_through_const::mod1[0]::id[0]<char>
[00:46:00]   expected: vtable_through_const-mod1.volatile[External] 
[00:46:00]   actual:   vtable_through_const.cgu-m.-mod1.volatile[External] 
[00:46:00] 
[00:46:00] fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something[0]<u8>
[00:46:00]   expected: vtable_through_const-mod1.volatile[External] 
[00:46:00]   actual:   vtable_through_const.cgu-m.-mod1.volatile[External] 
[00:46:00] 
[00:46:00] fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something_else[0]<u8>
[00:46:00]   expected: vtable_through_const-mod1.volatile[External] 
[00:46:00]   actual:   vtable_through_const.cgu-m.-mod1.volatile[External] 
[00:46:00] 
[00:46:00] thread '[codegen-units] codegen-units/partitioning/vtable-through-const.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1927:12
[00:46:00] 
[00:46:00] 
[00:46:00] failures:
[00:46:00]     [codegen-units] codegen-units/partitioning/extern-drop-glue.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/extern-generic.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/local-drop-glue.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/local-generic.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/local-inlining.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/regular-modules.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/statics.rs
[00:46:00]     [codegen-units] codegen-units/partitioning/vtable-through-const.rs
[00:46:00] 
[00:46:00] test result: FAILED. 22 passed; 10 failed; 3 ignored; 0 measured; 0 filtered out
[00:46:00] 
