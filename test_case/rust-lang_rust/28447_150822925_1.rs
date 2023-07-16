
we@we-pc MINGW64 ~
$ rustc --version --verbose
rustc 1.4.0-beta.3 (20eba406f 2015-10-16)
binary: rustc
commit-hash: 20eba406fa504192cbca531a18117290559d0e34
commit-date: 2015-10-16
host: x86_64-pc-windows-gnu
release: 1.4.0-beta.3

we@we-pc MINGW64 ~
$ RUST_BACKTRACE=1 rustc test.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/m                                                                                                    aster/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'No def'n found for DefId { krate: 0, node: 11 => Z }                                                                                                     in tcx.item_variance_map', ../src/librustc\middle\ty/mod.rs:1901

stack backtrace:
   0:         0x6c6414fe - <unknown>
   1:         0x6c63deb9 - <unknown>
   2:         0x6c60571e - <unknown>
   3:         0x6c60608b - <unknown>
   4:         0x6e003906 - <unknown>
   5:         0x6868191b - <unknown>
   6:         0x6867b772 - <unknown>
   7:         0x6868601f - <unknown>
   8:         0x6868565a - <unknown>
   9:         0x686bbc18 - <unknown>
  10:         0x6877045a - <unknown>
  11:         0x670087c2 - <unknown>
  12:         0x66feab77 - <unknown>
  13:         0x66fe64d8 - <unknown>
  14:         0x66fc2cf8 - <unknown>
  15:         0x67131c07 - <unknown>
  16:         0x6712f3f1 - <unknown>
  17:         0x6712ede8 - <unknown>
  18:         0x6c631643 - <unknown>
  19:         0x6712ef83 - <unknown>
  20:         0x6c64721d - <unknown>
  21:     0x7ffa1fa313d1 - <unknown>
