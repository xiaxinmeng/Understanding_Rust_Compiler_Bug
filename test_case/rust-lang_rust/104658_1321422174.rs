plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rand_xorshift v0.3.0
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0271]: type mismatch resolving `<SmallRng as SeedableRng>::Seed == [u8; 32]`
   |
   |
16 |     rand::SeedableRng::from_seed(seed)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 16 elements, found one with 32 elements
For more information about this error, try `rustc --explain E0271`.
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0271]: type mismatch resolving `<SmallRng as SeedableRng>::Seed == [{integer}; 32]`
  --> library/alloc/benches/lib.rs:25:5
   |
25 | /     rand::SeedableRng::from_seed([
26 | |         0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
27 | |         25, 26, 27, 28, 29, 30, 31,
28 | |     ])
   | |______^ expected an array with a fixed size of 16 elements, found one with 32 elements
error: could not compile `alloc` due to previous error
error: could not compile `alloc` due to previous error
error[E0271]: type mismatch resolving `<SmallRng as SeedableRng>::Seed == [{integer}; 32]`
   --> library/alloc/src/collections/binary_heap/tests.rs:443:41
    |
443 |       let mut rng: rand::rngs::SmallRng = rand::SeedableRng::from_seed([
444 | |         0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
445 | |         25, 26, 27, 28, 29, 30, 31,
446 | |     ]);
446 | |     ]);
    | |______^ expected an array with a fixed size of 16 elements, found one with 32 elements

error[E0271]: type mismatch resolving `<SmallRng as SeedableRng>::Seed == [{integer}; 32]`
   --> library/alloc/src/collections/linked_list/tests.rs:523:41
    |
523 |       let mut rng: rand::rngs::SmallRng = rand::SeedableRng::from_seed([
524 | |         0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
525 | |         25, 26, 27, 28, 29, 30, 31,
526 | |     ]);
526 | |     ]);
    | |______^ expected an array with a fixed size of 16 elements, found one with 32 elements

error[E0271]: type mismatch resolving `<SmallRng as SeedableRng>::Seed == [{integer}; 32]`
   |
   |
96 | /     rand::SeedableRng::from_seed([
97 | |         0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
98 | |         25, 26, 27, 28, 29, 30, 31,
99 | |     ])
   | |______^ expected an array with a fixed size of 16 elements, found one with 32 elements
error: could not compile `alloc` due to previous error
error: could not compile `alloc` due to 2 previous errors
error: could not compile `alloc` due to 2 previous errors
error[E0271]: type mismatch resolving `<SmallRng as SeedableRng>::Seed == [u8; 32]`
    |
    |
166 |     rand::SeedableRng::from_seed(seed)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 16 elements, found one with 32 elements
error: could not compile `core` due to previous error
error: could not compile `core` due to previous error
error[E0271]: type mismatch resolving `<SmallRng as SeedableRng>::Seed == [u8; 32]`
    |
    |
675 |         rand::SeedableRng::from_seed(seed)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 16 elements, found one with 32 elements
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:37
