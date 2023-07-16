text
[00:58:38] 
[00:58:38] 
[00:58:38] warning: doc comment contains an invalid Rust code block
[00:58:38]     --> src/libcore/num/mod.rs:28:9
[00:58:38]      |
[00:58:38] 28   |           #[doc = $x]
[00:58:38]      | 
[00:58:38]     ::: src/libcore/sync/atomic.rs:2012:1
[00:58:38]      |
[00:58:38]      |
[00:58:38] 2012 | / atomic_int! {
[00:58:38] 2013 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 2014 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 2015 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] ...    |
[00:58:38] 2024 | |     i64 AtomicI64 ATOMIC_I64_INIT
[00:58:38]      | |_- in this macro invocation
[00:58:38]      |
[00:58:38]      = help: mark blocks that do not contain Rust code as text: 