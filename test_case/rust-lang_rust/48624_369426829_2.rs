
     1.   If posix_spawn() and posix_spawnp() fail for any of the reasons that would cause vfork() or one of the exec to fail, an error value is returned as described by vfork() and exec, respectively (or, if the error occurs after the calling process successfully returns, the
          child process exits with exit status 127).
