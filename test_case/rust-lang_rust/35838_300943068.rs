
malloc(14)                                                                                                                                         = 0x560144bb11a0
memcpy(0x560144bb11a0, "RUST_BACKTRACE", 14)                                                                                                       = 0x560144bb11a0
memchr("RUST_BACKTRACE", '\0', 14)                                                                                                                 = 0
realloc(0x560144bb11a0, 15)                                                                                                                        = 0x560144bb11a0
pthread_mutex_lock(0x5601442862f0, 0x7ffd065c16d8, 15, 0x7ffd065c1608)                                                                             = 0
getenv("RUST_BACKTRACE")                                                                                                                           = nil
pthread_mutex_unlock(0x5601442862f0, 15, 0xc000, 416)                                                                                              = 0
free(0x560144bb11a0)                                                                                                                               = <void>
