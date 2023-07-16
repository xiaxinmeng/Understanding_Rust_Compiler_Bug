text
[00:58:38] 
[00:58:38] 
[00:58:38] warning: doc comment contains an invalid Rust code block
[00:58:38]     --> src/libcore/num/mod.rs:28:9
[00:58:38]      |
[00:58:38] 28   |           #[doc = $x]
[00:58:38]      | 
[00:58:38]     ::: src/libcore/sync/atomic.rs:2084:1
[00:58:38]      |
[00:58:38]      |
[00:58:38] 2084 | / atomic_int!{
[00:58:38] 2085 | |     stable(feature = "rust1", since = "1.0.0"),
[00:58:38] 2086 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:58:38] 2087 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:58:38] ...    |
[00:58:38] 2096 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:58:38]      | |_- in this macro invocation
[00:58:38]      |
[00:58:38]      = help: mark blocks that do not contain Rust code as text: 