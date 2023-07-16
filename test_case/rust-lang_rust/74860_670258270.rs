diff
 This method does not pass ownership of the raw file descriptor to the caller. The descriptor is only guaranteed to be valid while the original object has not yet been destroyed.
+Do not attempt to take ownership of this file descriptor (e.g. by using `FromRawFd`). On Android API level 30 and beyond, file descriptor ownership is enforced, which will lead to the process aborting if this occurs.
