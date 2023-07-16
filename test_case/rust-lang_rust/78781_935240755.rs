plain
   Compiling rand_core v0.5.1
   Compiling rand_chacha v0.3.0
   Compiling rand_xoshiro v0.6.0
   Compiling crossbeam-deque v0.7.4
   Compiling measureme v10.0.0-alpha (https://github.com/rust-lang/measureme#b4da5341)
   Compiling matchers v0.0.1
   Compiling rand_chacha v0.2.2
   Compiling rand v0.8.4
   Compiling digest v0.9.0
---
   Compiling regex v1.5.4
   Compiling crossbeam-deque v0.7.4
   Compiling rand_chacha v0.3.0
   Compiling rand_xoshiro v0.6.0
   Compiling measureme v10.0.0-alpha (https://github.com/rust-lang/measureme#b4da5341)
   Compiling block-buffer v0.9.0
   Compiling rand_chacha v0.2.2
   Compiling rand v0.8.4
   Compiling chrono v0.4.19
   Compiling chrono v0.4.19
   Compiling sha2 v0.9.1
   Compiling sha-1 v0.9.1
   Compiling md-5 v0.9.1
   Compiling rand v0.7.3
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:564:18
    |
564 |                 "mov {tmp_rbx:r}, rbx",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:3:5
    |
3   | mov %rdi, rbx

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:566:18
    |
566 |                 "mov rbx, {tmp_rbx:r}",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:5:10
    |
5   | mov rbx, %rdi

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:798:38
    |
798 | ...                   "lock xadd qword ptr [{atomic}], {tmp}",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:2:23
    |
2   |     lock xadd qword ptr [%rcx], %rax

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:564:18
    |
564 |                 "mov {tmp_rbx:r}, rbx",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:3:5
    |
3   | mov %rbp, rbx

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:566:18
    |
566 |                 "mov rbx, {tmp_rbx:r}",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:5:10
    |
5   | mov rbx, %rbp

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:564:18
    |
564 |                 "mov {tmp_rbx:r}, rbx",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:3:5
    |
3   | mov %rsi, rbx

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:566:18
    |
566 |                 "mov rbx, {tmp_rbx:r}",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:5:10
    |
5   | mov rbx, %rsi

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:564:18
    |
564 |                 "mov {tmp_rbx:r}, rbx",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:3:5
    |
3   | mov %rbx, rbx

error: unknown token in expression
error: unknown token in expression
   --> /cargo/git/checkouts/measureme-c8c0e3346d1404a3/b4da534/measureme/src/counters.rs:566:18
    |
566 |                 "mov rbx, {tmp_rbx:r}",
    |
note: instantiated into assembly here
note: instantiated into assembly here
   --> <inline asm>:5:10
    |
5   | mov rbx, %rbx

   Compiling matchers v0.0.1
error: could not compile `measureme` due to 9 previous errors
warning: build failed, waiting for other jobs to finish...
