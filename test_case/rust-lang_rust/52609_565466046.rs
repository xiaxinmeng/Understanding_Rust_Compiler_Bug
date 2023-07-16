
ERRORS

       EAGAIN The requested entropy was not available, and getrandom() would
              have blocked if the GRND_NONBLOCK flag was not set.

       EFAULT The address referred to by buf is outside the accessible
              address space.

       EINTR  The call was interrupted by a signal handler; see the
              description of how interrupted read(2) calls on "slow" devices
              are handled with and without the SA_RESTART flag in the
              signal(7) man page.

       EINVAL An invalid flag was specified in flags.

       ENOSYS The glibc wrapper function for getrandom() determined that the
              underlying kernel does not implement this system call.
