text
[00:58:38] 
[00:58:38] 
[00:58:38] warning: doc comment contains an invalid Rust code block
[00:58:38]     --> src/libcore/num/mod.rs:28:9
[00:58:38]      |
[00:58:38] 28   |           #[doc = $x]
[00:58:38]      | 
[00:58:38]     ::: src/libcore/sync/atomic.rs:1982:1
[00:58:38]      |
[00:58:38]      |
[00:58:38] 1982 | / atomic_int! {
[00:58:38] 1983 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 1984 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 1985 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] ...    |
[00:58:38] 1994 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:58:38]      | |_- in this macro invocation
[00:58:38]      |
[00:58:38]      = help: mark blocks that do not contain Rust code as text: 