
error: function pointers are not allowed in const fn                   ] 17/29: core
    --> src/libcore/fmt/mod.rs:1042:17
     |
1042 |                 (arg.formatter)(arg.value, &mut formatter)?;
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: function pointers are not allowed in const fn
    --> src/libcore/fmt/mod.rs:1104:9
     |
1104 |         (value.formatter)(value.value, self)
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
