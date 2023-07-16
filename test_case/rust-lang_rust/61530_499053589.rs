
 mateusz@ubuntu  /tmp  git clone https://github.com/dalek-cryptography/curve25519-dalek.git                                          
Cloning into 'curve25519-dalek'...
remote: Enumerating objects: 89, done.
remote: Counting objects: 100% (89/89), done.
remote: Compressing objects: 100% (55/55), done.
remote: Total 5149 (delta 51), reused 59 (delta 34), pack-reused 5060
Receiving objects: 100% (5149/5149), 2.26 MiB | 3.34 MiB/s, done.
Resolving deltas: 100% (3465/3465), done.
 mateusz@ubuntu  /tmp  curve25519-dalek/
 mateusz@ubuntu  /tmp/curve25519-dalek   master  git checkout use_upstream_intrinsics 
Branch 'use_upstream_intrinsics' set up to track remote branch 'use_upstream_intrinsics' from 'origin'.
Switched to a new branch 'use_upstream_intrinsics'
 mateusz@ubuntu  /tmp/curve25519-dalek   use_upstream_intrinsics  cargo test --features "simd_backend"

[long output]

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
