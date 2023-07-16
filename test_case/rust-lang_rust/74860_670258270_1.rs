diff
 This function consumes ownership of the specified file descriptor. The returned object will take responsibility for closing it when the object goes out of scope.
+The file descriptor consumed must not be already owned by another object. On Android API level 30 and beyond, file descriptor ownership is enforced, which will lead to the process aborting if this occurs.
