
error[E0308]: mismatched types

   --> /checkout/src/libstd/sys_common/net.rs:342:55

    |

342 |         cvt(unsafe { c::bind(*sock.as_inner(), addrp, len) })?;

    |                                                       ^^^ expected i32, found u32

error[E0308]: mismatched types

   --> /checkout/src/libstd/sys_common/net.rs:433:55

    |

433 |         cvt(unsafe { c::bind(*sock.as_inner(), addrp, len) })?;

    |                                                       ^^^ expected i32, found u32

error[E0308]: mismatched types

   --> /checkout/src/libstd/sys/unix/ext/net.rs:644:82

    |

644 |                 cvt(libc::bind(*inner.as_inner(), &addr as *const _ as *const _, len))?;

    |                                                                                  ^^^ expected i32, found u32

error[E0308]: mismatched types

   --> /checkout/src/libstd/sys/unix/ext/net.rs:923:85

    |

923 |                 cvt(libc::bind(*socket.0.as_inner(), &addr as *const _ as *const _, len))?;

    |                                                                                     ^^^ expected i32, found u32

error: aborting due to 4 previous errors

error: Could not compile `std`.

To learn more, run the command again with --verbose.
