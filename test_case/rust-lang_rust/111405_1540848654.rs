
different answers in an absolute sense
[src/main.rs:6] test(x, y, ieee_remainder) = 0.0
[src/main.rs:7] test(x, y, mod_trunc) = 0.0
[src/main.rs:8] test(x, y, rust_std_mod) = 1.5707963
[src/main.rs:9] test(x, y, libm_fmodf) = 1.5707963
[src/main.rs:10] test(x, y, libm_remainderf) = -1.1920929e-7

the errors are within reason though not exact to the ieee definition
[src/main.rs:13] error(test(x, y, libm_remainderf), 0.0, y) = 1.1920929e-7
[src/main.rs:14] error(test(x, y, libm_fmodf), 0.0, y) = 1.1920929e-7

the identity given for rem_euclid fails for this case
[src/main.rs:17] test(x, y, rem_euclid_identity) = 6.283185
[src/main.rs:17] x = 4.712389
