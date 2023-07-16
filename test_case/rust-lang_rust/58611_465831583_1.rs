text
[00:58:38] 
[00:58:38] 
[00:58:38] warning: doc comment contains an invalid Rust code block
[00:58:38]     --> src/libcore/num/mod.rs:28:9
[00:58:38]      |
[00:58:38] 28   |           #[doc = $x]
[00:58:38]      | 
[00:58:38]     ::: src/libcore/sync/atomic.rs:1952:1
[00:58:38]      |
[00:58:38]      |
[00:58:38] 1952 | / atomic_int! {
[00:58:38] 1953 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 1954 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 1955 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] ...    |
[00:58:38] 1964 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:58:38]      | |_- in this macro invocation
[00:58:38]      |
[00:58:38]      = help: mark blocks that do not contain Rust code as text: 