
error[E0308]: mismatched types
    --> /home/oliver/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_errors-100.0.0/emitter.rs:1392:39
     |
1392 |     (b_start..b_end + extra).contains(a_start) ||
     |                                       ^^^^^^^
     |                                       |
     |                                       expected &usize, found usize
     |                                       help: consider borrowing here: `&a_start`
     |
     = note: expected type `&usize`
                found type `usize`
