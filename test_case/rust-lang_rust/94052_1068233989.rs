plain

error[E0405]: cannot find trait `Copy` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1018:6
     |
1018 | impl Copy for siginfo_cldval {}
     |
help: consider importing this trait
     |
3227 | use Copy;
3227 | use Copy;
     |

error[E0405]: cannot find trait `Clone` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1019:6
     |
1019 | impl Clone for siginfo_cldval {
     |
help: consider importing this trait
     |
3227 | use Clone;
3227 | use Clone;
     |

error[E0405]: cannot find trait `Copy` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1032:6
     |
1032 | impl Copy for siginfo_killval {}
     |
help: consider importing this trait
     |
3227 | use Copy;
3227 | use Copy;
     |

error[E0405]: cannot find trait `Clone` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1033:6
     |
1033 | impl Clone for siginfo_killval {
     |
help: consider importing this trait
     |
3227 | use Clone;
3227 | use Clone;
     |

error[E0405]: cannot find trait `Copy` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1046:6
     |
1046 | impl Copy for siginfo_sigcld {}
     |
help: consider importing this trait
     |
3227 | use Copy;
3227 | use Copy;
     |

error[E0405]: cannot find trait `Clone` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1047:6
     |
1047 | impl Clone for siginfo_sigcld {
     |
help: consider importing this trait
     |
3227 | use Clone;
3227 | use Clone;
     |

error[E0405]: cannot find trait `Copy` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1060:6
     |
1060 | impl Copy for siginfo_kill {}
     |
help: consider importing this trait
     |
3227 | use Copy;
3227 | use Copy;
     |

error[E0405]: cannot find trait `Clone` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1061:6
     |
1061 | impl Clone for siginfo_kill {
     |
help: consider importing this trait
     |
3227 | use Clone;
3227 | use Clone;
     |

error[E0405]: cannot find trait `Copy` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.120/src/unix/solarish/mod.rs:1068:25
     |
1068 |     unsafe fn sidata<T: Copy>(&self) -> T {
     |
help: consider importing this trait
     |
3227 | use Copy;
